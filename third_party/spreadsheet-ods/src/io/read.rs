use crate::sheet_::Header;
use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, Write};
use std::mem;
use std::path::Path;
use std::str::from_utf8;

use chrono::{Duration, NaiveDateTime};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use zip::ZipArchive;

use crate::attrmap2::AttrMap2;
use crate::cell_::CellData;
use crate::condition::{Condition, ValueCondition};
use crate::config::{Config, ConfigItem, ConfigItemType, ConfigValue};
use crate::draw::{Annotation, DrawFrame, DrawFrameContent, DrawImage};
use crate::ds::detach::Detach;
use crate::error::OdsError;
use crate::format::{FormatPart, FormatPartType, ValueFormatTrait, ValueStyleMap};
use crate::io::parse::{
    parse_bool, parse_currency, parse_datetime, parse_duration, parse_f64, parse_i16, parse_i32,
    parse_i64, parse_string, parse_u32, parse_visibility, parse_xlink_actuate, parse_xlink_show,
    parse_xlink_type,
};
use crate::io::NamespaceMap;
use crate::manifest::Manifest;
use crate::metadata::{
    MetaAutoReload, MetaDocumentStatistics, MetaHyperlinkBehaviour, MetaTemplate, MetaUserDefined,
    MetaValue,
};
use crate::refs::{parse_cellranges, parse_cellref};
use crate::sheet::{Grouped, SplitMode};
use crate::sheet_::{dedup_colheader, CellDataIter, CellDataIterMut, ColHeader, RowHeader};
use crate::style::stylemap::StyleMap;
use crate::style::tabstop::TabStop;
use crate::style::{
    AnyStyleRef, ColStyle, ColStyleRef, FontFaceDecl, GraphicStyle, HeaderFooter, MasterPage,
    MasterPageRef, PageStyle, ParagraphStyle, RowStyle, RowStyleRef, RubyStyle, StyleOrigin,
    StyleUse, TableStyle, TableStyleRef, TextStyle,
};
use crate::text::{TextP, TextTag};
use crate::validation::{MessageType, Validation, ValidationError, ValidationHelp, ValidationRef};
use crate::workbook::{EventListener, Script};
use crate::xmltree::XmlTag;
use crate::{
    CellStyle, CellStyleRef, Length, Sheet, Value, ValueFormatBoolean, ValueFormatCurrency,
    ValueFormatDateTime, ValueFormatNumber, ValueFormatPercentage, ValueFormatText,
    ValueFormatTimeDuration, ValueType, WorkBook,
};

type OdsXmlReader<'a> = quick_xml::Reader<&'a mut dyn BufRead>;

/// Read options for ods-files.
#[derive(Debug, Default)]
pub struct OdsOptions {
    // parse the content only.
    content_only: bool,
    // expand duplicated cells
    use_repeat_for_cells: bool,
    // ignore empty cells.
    ignore_empty_cells: bool,
}

impl OdsOptions {
    /// Parse the content only.
    ///
    /// Doesn't buffer any extra files and ignores styles etc.
    /// This saves quite some time if only the cell-data is needed.
    pub fn content_only(mut self) -> Self {
        self.content_only = true;
        self
    }

    /// Parse everything.
    ///
    /// Reads styles and buffers extra files.
    /// This is the default. If the data will be written again this options
    /// should be used.
    pub fn read_styles(mut self) -> Self {
        self.content_only = false;
        self
    }

    /// The value of table:number-columns-repeated is stored as part of the
    /// cell-data, and the cell-data is not duplicated. The cell-data can
    /// only be found at the original row/col.
    ///
    /// This can save a bit of time when reading, but makes working with the
    /// data harder. Keeping track of overlapping cells makes this tricky.
    pub fn use_repeat_for_cells(mut self) -> Self {
        self.use_repeat_for_cells = true;
        self
    }

    /// Cells are cloned based on their table:number-columns-repeated.
    ///
    /// This is the default behaviour. The cell-data can be found at each row/col
    /// that the repeat count includes.
    ///
    /// Most of the time the repeat-count is used for empty cells to fill the
    /// required structure. These completely empty cells are always dumped.
    ///
    /// See: ignore_empty_cells().
    pub fn use_clone_for_cells(mut self) -> Self {
        self.use_repeat_for_cells = false;
        self
    }

    /// Ignores cells without value and formula.
    ///
    /// This can be useful, if only the data is needed. If you store such
    /// a spreadsheet you will loose cell-formating, spans etc.
    pub fn ignore_empty_cells(mut self) -> Self {
        self.ignore_empty_cells = true;
        self
    }

    /// Reads cells without value and formula.
    ///
    /// This is the default behaviour. As such cells can have a style,
    /// annotations etc it is recommended to use this option.
    ///
    /// Cells without any information, that are only structural are always
    /// ignored.
    pub fn read_empty_cells(mut self) -> Self {
        self.ignore_empty_cells = false;
        self
    }

    /// Reads a .ods file.
    pub fn read_ods<T: Read + Seek>(&self, read: T) -> Result<WorkBook, OdsError> {
        let zip = ZipArchive::new(read)?;
        if self.content_only {
            read_ods_impl_content_only(zip, self)
        } else {
            read_ods_impl(zip, self)
        }
    }

    /// Reads a flat .fods file.
    pub fn read_fods<T: BufRead>(&self, mut read: T) -> Result<WorkBook, OdsError> {
        if self.content_only {
            read_fods_impl_content_only(&mut read, self)
        } else {
            read_fods_impl(&mut read, self)
        }
    }
}

/// Reads an ODS-file from a buffer
pub fn read_ods_buf(buf: &[u8]) -> Result<WorkBook, OdsError> {
    let read = Cursor::new(buf);
    OdsOptions::default().read_ods(read)
}

/// Reads an ODS-file from a reader
pub fn read_ods_from<T: Read + Seek>(read: T) -> Result<WorkBook, OdsError> {
    OdsOptions::default().read_ods(read)
}

/// Reads an ODS-file.
pub fn read_ods<P: AsRef<Path>>(path: P) -> Result<WorkBook, OdsError> {
    let read = BufReader::new(File::open(path.as_ref())?);
    OdsOptions::default().read_ods(read)
}

/// Reads an FODS-file from a buffer
pub fn read_fods_buf(buf: &[u8]) -> Result<WorkBook, OdsError> {
    let mut read = Cursor::new(buf);
    OdsOptions::default().read_fods(&mut read)
}

/// Reads an FODS-file from a reader
pub fn read_fods_from<T: Read>(read: T) -> Result<WorkBook, OdsError> {
    let read = BufReader::new(read);
    OdsOptions::default().read_fods(read)
}

/// Reads an FODS-file.
pub fn read_fods<P: AsRef<Path>>(path: P) -> Result<WorkBook, OdsError> {
    let read = BufReader::new(File::open(path.as_ref())?);
    OdsOptions::default().read_fods(read)
}

#[derive(Default)]
struct OdsContext {
    book: WorkBook,

    #[allow(dead_code)]
    content_only: bool,
    use_repeat_for_cells: bool,
    ignore_empty_cells: bool,

    buffers: Vec<Vec<u8>>,
    xml_buffer: Vec<XmlTag>,
    col_group_buffer: Vec<Grouped>,
    row_group_buffer: Vec<Grouped>,
}

impl OdsContext {
    fn new(options: &OdsOptions) -> Self {
        Self {
            content_only: options.content_only,
            use_repeat_for_cells: options.use_repeat_for_cells,
            ignore_empty_cells: options.ignore_empty_cells,
            ..Default::default()
        }
    }

    fn pop_xml_buf(&mut self) -> Vec<XmlTag> {
        mem::take(&mut self.xml_buffer)
    }

    fn push_xml_buf(&mut self, mut buf: Vec<XmlTag>) {
        buf.clear();
        self.xml_buffer = buf;
    }

    fn pop_colgroup_buf(&mut self) -> Vec<Grouped> {
        mem::take(&mut self.col_group_buffer)
    }

    fn push_colgroup_buf(&mut self, mut buf: Vec<Grouped>) {
        buf.clear();
        self.col_group_buffer = buf;
    }

    fn pop_rowgroup_buf(&mut self) -> Vec<Grouped> {
        mem::take(&mut self.row_group_buffer)
    }

    fn push_rowgroup_buf(&mut self, mut buf: Vec<Grouped>) {
        buf.clear();
        self.row_group_buffer = buf;
    }

    // Return a temporary buffer.
    fn pop_buf(&mut self) -> Vec<u8> {
        self.buffers.pop().unwrap_or_default()
    }

    // Give back a buffer to be reused later.
    fn push_buf(&mut self, mut buf: Vec<u8>) {
        buf.clear();
        self.buffers.push(buf);
    }
}

fn read_fods_impl(read: &mut dyn BufRead, options: &OdsOptions) -> Result<WorkBook, OdsError> {
    let mut ctx = OdsContext::new(options);
    let mut xml = quick_xml::Reader::from_reader(read);

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_fods_content {:?}", evt);
        }

        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:document" => {
                let (version, xmlns) = read_namespaces_and_version(&mut xml, xml_tag)?;
                ctx.book.xmlns.insert("fods.xml".to_string(), xmlns);
                if let Some(version) = version {
                    ctx.book.set_version(version);
                }
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:document" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:meta" => {
                read_office_meta(&mut ctx, &mut xml)?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:settings" => {
                read_office_settings(&mut ctx, &mut xml)?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:scripts" => {
                read_scripts(&mut ctx, &mut xml)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:font-face-decls" => {
                read_office_font_face_decls(&mut ctx, &mut xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:styles" => {
                read_office_styles(&mut ctx, &mut xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:automatic-styles" => {
                read_office_automatic_styles(&mut ctx, &mut xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:master-styles" => {
                read_office_master_styles(&mut ctx, &mut xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:body" => {
                read_office_body(&mut ctx, &mut xml)?;
            }

            Event::Decl(_) => {}
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_fods_content", &evt)?;
            }
        }
    }
    ctx.push_buf(buf);

    calculations(&mut ctx)?;

    // We do some data duplication here, to make everything easier to use.
    calc_derived(&mut ctx.book)?;

    Ok(ctx.book)
}

fn read_fods_impl_content_only(
    read: &mut dyn BufRead,
    options: &OdsOptions,
) -> Result<WorkBook, OdsError> {
    let mut ctx = OdsContext::new(options);
    let mut xml: quick_xml::Reader<&mut dyn BufRead> = quick_xml::Reader::from_reader(read);

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_fods_content_only {:?}", evt);
        }

        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:body" => {
                read_office_body(&mut ctx, &mut xml)?;
            }
            Event::Eof => {
                break;
            }
            _ => {
                // a lot is ignored.
            }
        }
    }
    ctx.push_buf(buf);

    calculations(&mut ctx)?;

    Ok(ctx.book)
}

/// Reads an ODS-file.
fn read_ods_impl<R: Read + Seek>(
    mut zip: ZipArchive<R>,
    options: &OdsOptions,
) -> Result<WorkBook, OdsError> {
    let mut ctx = OdsContext::new(options);

    if let Ok(z) = zip.by_name("META-INF/manifest.xml") {
        let mut read = BufReader::new(z);
        let read: &mut dyn BufRead = &mut read;
        let mut xml = quick_xml::Reader::from_reader(read);

        read_ods_manifest(&mut ctx, &mut xml)?;
    }

    read_ods_extras(&mut ctx, &mut zip)?;

    if let Ok(z) = zip.by_name("meta.xml") {
        let mut read = BufReader::new(z);
        let read: &mut dyn BufRead = &mut read;
        let mut xml = quick_xml::Reader::from_reader(read);

        read_ods_metadata(&mut ctx, &mut xml)?;
    }

    if let Ok(z) = zip.by_name("settings.xml") {
        let mut read = BufReader::new(z);
        let read: &mut dyn BufRead = &mut read;
        let mut xml = quick_xml::Reader::from_reader(read);
        read_ods_settings(&mut ctx, &mut xml)?;
    }

    if let Ok(z) = zip.by_name("styles.xml") {
        let mut read = BufReader::new(z);
        let read: &mut dyn BufRead = &mut read;
        let mut xml = quick_xml::Reader::from_reader(read);
        read_ods_styles(&mut ctx, &mut xml)?;
    }

    {
        let mut read = BufReader::new(zip.by_name("content.xml")?);
        let read: &mut dyn BufRead = &mut read;
        let mut xml = quick_xml::Reader::from_reader(read);
        read_ods_content(&mut ctx, &mut xml)?;
    }

    calculations(&mut ctx)?;

    // We do some data duplication here, to make everything easier to use.
    calc_derived(&mut ctx.book)?;

    Ok(ctx.book)
}

/// Reads an ODS-file.
fn read_ods_impl_content_only<R: Read + Seek>(
    mut zip: ZipArchive<R>,
    options: &OdsOptions,
) -> Result<WorkBook, OdsError> {
    let mut ctx = OdsContext::new(options);

    let mut read = BufReader::new(zip.by_name("content.xml")?);
    let read: &mut dyn BufRead = &mut read;
    let mut xml = quick_xml::Reader::from_reader(read);

    // todo: this still reads styles etc from content.xml
    read_ods_content(&mut ctx, &mut xml)?;

    calculations(&mut ctx)?;

    Ok(ctx.book)
}

fn read_ods_extras<R: Read + Seek>(
    ctx: &mut OdsContext,
    zip: &mut ZipArchive<R>,
) -> Result<(), OdsError> {
    // now the data if needed ...
    for manifest in ctx.book.manifest.values_mut().filter(|v| !v.is_dir()) {
        if !matches!(
            manifest.full_path.as_str(),
            "/" | "settings.xml" | "styles.xml" | "content.xml" | "meta.xml"
        ) {
            let mut ze = zip.by_name(manifest.full_path.as_str())?;
            let mut buf = Vec::new();
            ze.read_to_end(&mut buf)?;
            manifest.buffer = Some(buf);
        }
    }

    Ok(())
}

fn read_ods_manifest(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        match &evt {
            Event::Decl(_) => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"manifest:manifest" => {}
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"manifest:manifest" => {}

            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"manifest:file-entry" => {
                let mut manifest = Manifest::default();

                for attr in xml_tag.attributes().with_checks(false) {
                    let attr = attr?;

                    if attr.key.as_ref() == b"manifest:full-path" {
                        manifest.full_path = attr.decode_and_unescape_value(xml)?.to_string();
                    } else if attr.key.as_ref() == b"manifest:version" {
                        manifest.version = Some(attr.decode_and_unescape_value(xml)?.to_string());
                    } else if attr.key.as_ref() == b"manifest:media-type" {
                        manifest.media_type = attr.decode_and_unescape_value(xml)?.to_string();
                    }
                }

                // some files shouldn't be in the manifest
                if manifest.full_path != "mimetype" && manifest.full_path != "META-INF/manifest.xml"
                {
                    ctx.book.add_manifest(manifest);
                }
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_manifest", &evt)?;
            }
        }
        buf.clear();
    }
    ctx.push_buf(buf);
    Ok(())
}

// Clone cell-data.
fn calculations(ctx: &mut OdsContext) -> Result<(), OdsError> {
    for i in 0..ctx.book.num_sheets() {
        dedup_colheader(ctx.book.sheet_mut(i))?;
        if ctx.use_repeat_for_cells {
            calc_repeat_sheet(ctx.book.sheet_mut(i))?;
        } else {
            calc_cloned_sheet(ctx.book.sheet_mut(i))?;
        }
    }
    Ok(())
}

// Cleanup repeat cell-data.
fn calc_repeat_sheet(sheet: &mut Sheet) -> Result<(), OdsError> {
    let mut dropped = Vec::new();

    // clone by row-repeat

    // last two rows often have insane repeat values. clear now.
    for (_row, rh) in sheet.row_header.iter_mut().rev().take(5) {
        if rh.repeat > 1000 {
            rh.repeat = 1;
        }
    }

    // clone by cell-repeat
    let mut it = CellDataIterMut::new(sheet.data.range_mut(..));
    loop {
        let Some(((row, col), data)) = it.next() else {
            break;
        };

        if data.repeat > 1 {
            let last_in_row = if let Some((next_row, _next_col)) = it.peek_cell() {
                row != next_row
            } else {
                true
            };
            if last_in_row && data.is_empty() {
                // skip on empty last cell. this is just an editing artifact.
                dropped.push((row, col));
                continue;
            }
        }
    }
    for (row, col) in dropped {
        sheet.data.remove(&(row, col));
    }

    Ok(())
}

// Clone cell-data.
fn calc_cloned_sheet(sheet: &mut Sheet) -> Result<(), OdsError> {
    let mut cloned = Vec::new();
    let mut dropped = Vec::new();

    // clone by row-repeat

    // last two rows often have insane repeat values. clear now.
    for (_row, rh) in sheet.row_header.iter_mut().rev().take(5) {
        if rh.repeat > 1000 {
            rh.repeat = 1;
        }
    }
    // duplicate by row-repeat
    for (row, rh) in sheet.row_header.iter().filter(|(_, v)| v.repeat > 1) {
        // get one row
        let cit = CellDataIter::new(sheet.data.range((*row, 0)..(row + 1, 0)));
        for ((row, col), data) in cit {
            for i in 1..rh.repeat {
                cloned.push((row + i, col, data.clone()));
            }
        }
    }
    for (row, col, data) in cloned.drain(..) {
        sheet.data.insert((row, col), data);
    }
    // after the previous operation the repeat value is reduced to a span where
    // the header-values are valid. no longer denotes repeated row-data.
    for (_row, rh) in sheet.row_header.iter_mut() {
        mem::swap(&mut rh.repeat, &mut rh.span);
    }

    // clone by cell-repeat

    let mut it = CellDataIterMut::new(sheet.data.range_mut(..));
    loop {
        let Some(((row, col), data)) = it.next() else {
            break;
        };

        if data.repeat > 1 {
            let repeat = mem::replace(&mut data.repeat, 1);

            let last_in_row = if let Some((next_row, _next_col)) = it.peek_cell() {
                row != next_row
            } else {
                true
            };
            if last_in_row && data.is_empty() {
                // skip on empty last cell. this is just an editing artifact.
                dropped.push((row, col));
                continue;
            }

            for i in 1..repeat {
                cloned.push((row, col + i, data.clone()));
            }
        }
    }
    for (row, col) in dropped {
        sheet.data.remove(&(row, col));
    }
    for (row, col, data) in cloned {
        sheet.data.insert((row, col), data);
    }

    Ok(())
}

// Sets some values from the styles on the corresponding data fields.
fn calc_derived(book: &mut WorkBook) -> Result<(), OdsError> {
    let v = book
        .config
        .get_value(&["ooo:view-settings", "Views", "0", "ActiveTable"]);
    if let Some(ConfigValue::String(n)) = v {
        book.config_mut().active_table = n.clone();
    }
    let v = book
        .config
        .get_value(&["ooo:view-settings", "Views", "0", "HasSheetTabs"]);
    if let Some(ConfigValue::Boolean(n)) = v {
        book.config_mut().has_sheet_tabs = *n;
    }
    let v = book
        .config
        .get_value(&["ooo:view-settings", "Views", "0", "ShowGrid"]);
    if let Some(ConfigValue::Boolean(n)) = v {
        book.config_mut().show_grid = *n;
    }
    let v = book
        .config
        .get_value(&["ooo:view-settings", "Views", "0", "ShowPageBreaks"]);
    if let Some(ConfigValue::Boolean(n)) = v {
        book.config_mut().show_page_breaks = *n;
    }

    for i in 0..book.num_sheets() {
        let mut sheet = book.detach_sheet(i);

        // Set the column widths.
        for ch in sheet.col_header.values_mut() {
            if let Some(style_name) = &ch.style {
                if let Some(style) = book.colstyle(style_name) {
                    if style.use_optimal_col_width()? {
                        ch.width = Length::Default;
                    } else {
                        ch.width = style.col_width()?;
                    }
                }
            }
        }

        // Set the row heights
        for rh in sheet.row_header.values_mut() {
            if let Some(style_name) = &rh.style {
                if let Some(style) = book.rowstyle(style_name) {
                    if style.use_optimal_row_height()? {
                        rh.height = Length::Default;
                    } else {
                        rh.height = style.row_height()?;
                    }
                }
            }
        }

        let v = book.config.get(&[
            "ooo:view-settings",
            "Views",
            "0",
            "Tables",
            sheet.name().as_str(),
        ]);

        if let Some(cc) = v {
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["CursorPositionX"]) {
                sheet.config_mut().cursor_x = *n as u32;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["CursorPositionY"]) {
                sheet.config_mut().cursor_y = *n as u32;
            }
            if let Some(ConfigValue::Short(n)) = cc.get_value_rec(&["HorizontalSplitMode"]) {
                sheet.config_mut().hor_split_mode = SplitMode::try_from(*n)?;
            }
            if let Some(ConfigValue::Short(n)) = cc.get_value_rec(&["VerticalSplitMode"]) {
                sheet.config_mut().vert_split_mode = SplitMode::try_from(*n)?;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["HorizontalSplitPosition"]) {
                sheet.config_mut().hor_split_pos = *n as u32;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["VerticalSplitPosition"]) {
                sheet.config_mut().vert_split_pos = *n as u32;
            }
            if let Some(ConfigValue::Short(n)) = cc.get_value_rec(&["ActiveSplitRange"]) {
                sheet.config_mut().active_split_range = *n;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["PositionLeft"]) {
                sheet.config_mut().position_left = *n as u32;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["PositionRight"]) {
                sheet.config_mut().position_right = *n as u32;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["PositionTop"]) {
                sheet.config_mut().position_top = *n as u32;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["PositionBottom"]) {
                sheet.config_mut().position_bottom = *n as u32;
            }
            if let Some(ConfigValue::Short(n)) = cc.get_value_rec(&["ZoomType"]) {
                sheet.config_mut().zoom_type = *n;
            }
            if let Some(ConfigValue::Int(n)) = cc.get_value_rec(&["ZoomValue"]) {
                sheet.config_mut().zoom_value = *n;
            }
            if let Some(ConfigValue::Boolean(n)) = cc.get_value_rec(&["ShowGrid"]) {
                sheet.config_mut().show_grid = *n;
            }
        }

        book.attach_sheet(sheet);
    }

    Ok(())
}

// Reads the content.xml
fn read_ods_content(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_ods_content {:?}", evt);
        }
        match &evt {
            Event::Decl(_) => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:document-content" => {
                let (version, xmlns) = read_namespaces_and_version(xml, xml_tag)?;
                if let Some(version) = version {
                    ctx.book.set_version(version);
                }
                ctx.book.xmlns.insert("content.xml".to_string(), xmlns);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:document-content" => {}

            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"office:scripts" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:scripts" => {
                read_scripts(ctx, xml)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:font-face-decls" => {
                read_office_font_face_decls(ctx, xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:styles" => {
                read_office_styles(ctx, xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:automatic-styles" => {
                read_office_automatic_styles(ctx, xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:master-styles" => {
                read_office_master_styles(ctx, xml, StyleOrigin::Content)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:body" => {
                read_office_body(ctx, xml)?;
            }

            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_ods_content", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// Reads the content.xml
fn read_office_body(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!("read_office_body {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:body" => {}
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:body" => {
                break;
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:spreadsheet" => {}
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:spreadsheet" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:content-validations" => {
                read_validations(ctx, xml)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table" => {
                read_table(ctx, xml, xml_tag)?
            }

            // from the prelude
            Event::Empty(xml_tag) | Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"table:calculation-settings"
                    || xml_tag.name().as_ref() == b"table:label-ranges"
                    || xml_tag.name().as_ref() == b"table:tracked-changes"
                    || xml_tag.name().as_ref() == b"text:alphabetical-index-auto-mark-file"
                    || xml_tag.name().as_ref() == b"text:dde-connection-decls"
                    || xml_tag.name().as_ref() == b"text:sequence-decls"
                    || xml_tag.name().as_ref() == b"text:user-field-decls"
                    || xml_tag.name().as_ref() == b"text:variable-decls" =>
            {
                let v = read_xml(ctx, xml, xml_tag, empty_tag)?;
                ctx.book.extra.push(v);
            }
            // from the epilogue
            Event::Empty(xml_tag) | Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"table:consolidation"
                    || xml_tag.name().as_ref() == b"table:data-pilot-tables"
                    || xml_tag.name().as_ref() == b"table:database-ranges"
                    || xml_tag.name().as_ref() == b"table:dde-links"
                    || xml_tag.name().as_ref() == b"table:named-expressions"
                    || xml_tag.name().as_ref() == b"calcext:conditional-formats" =>
            {
                let v = read_xml(ctx, xml, xml_tag, empty_tag)?;
                ctx.book.extra.push(v);
            }
            // from the prelude
            Event::End(xml_tag)
                if xml_tag.name().as_ref() == b"table:calculation-settings"
                    || xml_tag.name().as_ref() == b"table:label-ranges"
                    || xml_tag.name().as_ref() == b"table:tracked-changes"
                    || xml_tag.name().as_ref() == b"text:alphabetical-index-auto-mark-file"
                    || xml_tag.name().as_ref() == b"text:dde-connection-decls"
                    || xml_tag.name().as_ref() == b"text:sequence-decls"
                    || xml_tag.name().as_ref() == b"text:user-field-decls"
                    || xml_tag.name().as_ref() == b"text:variable-decls" => {}
            // from the epilogue
            Event::End(xml_tag)
                if xml_tag.name().as_ref() == b"table:consolidation"
                    || xml_tag.name().as_ref() == b"table:data-pilot-tables"
                    || xml_tag.name().as_ref() == b"table:database-ranges"
                    || xml_tag.name().as_ref() == b"table:dde-links"
                    || xml_tag.name().as_ref() == b"table:named-expressions" => {}

            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_office_body", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_namespaces_and_version(
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(Option<String>, NamespaceMap), OdsError> {
    let mut version = None;
    let mut xmlns = NamespaceMap::new();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"office:version" => {
                version = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref().starts_with(b"xmlns:") => {
                let k = from_utf8(attr.key.as_ref())?.to_string();
                let v = attr.decode_and_unescape_value(xml)?.to_string();
                xmlns.insert(k, v);
            }
            attr if attr.key.as_ref() == b"office:mimetype" => {
                if attr.decode_and_unescape_value(xml)?
                    != "application/vnd.oasis.opendocument.spreadsheet"
                {
                    return Err(OdsError::Parse(
                        "invalid content-type",
                        Some(attr.decode_and_unescape_value(xml)?.to_string()),
                    ));
                }
            }
            attr => {
                unused_attr(
                    "read_namespaces_and_version",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }
    Ok((version, xmlns))
}

// Reads the table.
fn read_table(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    let mut sheet = Sheet::new("");

    read_table_attr(xml, &mut sheet, super_tag)?;

    // Cell
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut col_data: bool = false;

    // Columns
    let mut col_range_from = 0;
    let mut col_group = ctx.pop_colgroup_buf();

    // Rows
    let mut row_repeat: u32 = 1;
    let mut row_range_from = 0;
    let mut row_group = ctx.pop_rowgroup_buf();

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_table {:?}", evt);
        }
        match &evt {
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table" => {
                break;
            }

            // Prelude
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"table:title"
                    || xml_tag.name().as_ref() == b"table:desc"
                    || xml_tag.name().as_ref() == b"table:table-source"
                    || xml_tag.name().as_ref() == b"office:dde-source"
                    || xml_tag.name().as_ref() == b"table:scenario"
                    || xml_tag.name().as_ref() == b"office:forms"
                    || xml_tag.name().as_ref() == b"table:shapes" =>
            {
                sheet.extra.push(read_xml(ctx, xml, xml_tag, empty_tag)?);
            }
            Event::End(xml_tag)
                if xml_tag.name().as_ref() == b"table:title"
                    || xml_tag.name().as_ref() == b"table:desc"
                    || xml_tag.name().as_ref() == b"table:table-source"
                    || xml_tag.name().as_ref() == b"office:dde-source"
                    || xml_tag.name().as_ref() == b"table:scenario"
                    || xml_tag.name().as_ref() == b"office:forms"
                    || xml_tag.name().as_ref() == b"table:shapes" => {}

            // Epilogue
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"table:named-expressions"
                    || xml_tag.name().as_ref() == b"calcext:conditional-formats" =>
            {
                sheet.extra.push(read_xml(ctx, xml, xml_tag, empty_tag)?);
            }
            Event::End(xml_tag)
                if xml_tag.name().as_ref() == b"table:named-expressions"
                    || xml_tag.name().as_ref() == b"calcext:conditional-formats" => {}

            //
            // table columns
            //
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-column-group" => {
                let v = read_table_column_group_attr(col, xml_tag)?;
                col_group.push(v);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-column-group" => {
                if let Some(mut v) = col_group.pop() {
                    v.set_to(col - 1);
                    sheet.group_cols.push(v);
                }
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-header-columns" => {
                col_range_from = col;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-header-columns" => {
                if let Some(header_cols) = &mut sheet.header_cols {
                    header_cols.to = col - 1;
                } else {
                    sheet.header_cols = Some(Header {
                        from: col_range_from,
                        to: col - 1,
                    });
                }
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-columns" => {}
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-columns" => {}

            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"table:table-column" => {
                let col_repeat = read_table_col_attr(xml, &mut sheet, xml_tag, col)?;
                col += col_repeat;
            }

            //
            // table rows
            //
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-row-group" => {
                let v = read_table_row_group_attr(row, xml_tag)?;
                row_group.push(v);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-row-group" => {
                if let Some(mut v) = row_group.pop() {
                    v.set_to(row - 1);
                    sheet.group_rows.push(v);
                } else {
                    // there are no unbalanced tags.
                }
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-header-rows" => {
                row_range_from = row;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-header-rows" => {
                if let Some(header_rows) = &mut sheet.header_rows {
                    header_rows.to = row - 1;
                } else {
                    sheet.header_rows = Some(Header {
                        from: row_range_from,
                        to: row - 1,
                    });
                }
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-rows" => {
                // noop
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-rows" => {
                // noop
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:table-row" => {
                col = 0;
                row_repeat = read_table_row_attr(xml, &mut sheet, row, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:table-row" => {
                if col_data {
                    // row-repeat is ignored unless there is any cell-data in that row.
                    sheet.set_row_repeat(row, row_repeat);
                }
                row += row_repeat;
                row_repeat = 1;
                col_data = false;
            }

            //
            // table cells
            //
            Event::Empty(xml_tag) | Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"table:table-cell"
                    || xml_tag.name().as_ref() == b"table:covered-table-cell" =>
            {
                let (cell_repeat, have_data) =
                    read_table_cell(ctx, xml, &mut sheet, row, col, xml_tag, empty_tag)?;
                col += cell_repeat;
                col_data |= have_data;
            }

            _ => {
                unused_event("read_table", &evt)?;
            }
        }
        buf.clear();
    }

    ctx.push_buf(buf);
    ctx.push_colgroup_buf(col_group);
    ctx.push_rowgroup_buf(row_group);

    ctx.book.push_sheet(sheet);

    Ok(())
}

// Reads the table attributes.
fn read_table_attr(
    xml: &mut OdsXmlReader<'_>,
    sheet: &mut Sheet,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:name" => {
                sheet.set_name(attr.decode_and_unescape_value(xml)?);
            }
            attr if attr.key.as_ref() == b"table:style-name" => {
                let name = &attr.decode_and_unescape_value(xml)?;
                sheet.style = Some(TableStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"table:print" => {
                sheet.set_print(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:display" => {
                sheet.set_display(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:print-ranges" => {
                let v = attr.decode_and_unescape_value(xml)?;
                sheet.print_ranges = parse_cellranges(v.as_ref())?;
            }
            attr => {
                unused_attr("read_table_attr", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    Ok(())
}

// Reads table-row attributes. Returns the repeat-count.
fn read_table_row_attr(
    xml: &mut OdsXmlReader<'_>,
    sheet: &mut Sheet,
    row: u32,
    super_tag: &BytesStart<'_>,
) -> Result<u32, OdsError> {
    let mut row_repeat: u32 = 1;
    let mut row_header = None;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            // table:default-cell-style-name 19.615, table:visibility 19.749 and xml:id 19.914.
            attr if attr.key.as_ref() == b"table:number-rows-repeated" => {
                row_repeat = parse_u32(&attr.value)?;
            }
            attr if attr.key.as_ref() == b"table:style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                row_header.get_or_insert_with(RowHeader::default).style =
                    Some(RowStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"table:default-cell-style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                row_header.get_or_insert_with(RowHeader::default).cellstyle =
                    Some(CellStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"table:visibility" => {
                let visible = parse_visibility(&attr.value)?;
                row_header.get_or_insert_with(RowHeader::default).visible = visible;
            }
            attr => {
                unused_attr("read_table_row_attr", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    if let Some(mut row_header) = row_header {
        row_header.repeat = row_repeat;
        sheet.row_header.insert(row, row_header);
    }

    Ok(row_repeat)
}

// Reads the table:table-column-group attributes.
fn read_table_column_group_attr(
    table_col: u32,
    super_tag: &BytesStart<'_>,
) -> Result<Grouped, OdsError> {
    let mut display = true;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:display" => {
                display = parse_bool(&attr.value)?;
            }
            attr => {
                unused_attr(
                    "read_table_column_group_attr",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    Ok(Grouped {
        from: table_col,
        to: 0,
        display,
    })
}

// Reads the table:table-row-group attributes.
fn read_table_row_group_attr(row: u32, super_tag: &BytesStart<'_>) -> Result<Grouped, OdsError> {
    let mut display = true;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:display" => {
                display = parse_bool(&attr.value)?;
            }
            attr => {
                unused_attr(
                    "read_table_row_group_attr",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    Ok(Grouped {
        from: row,
        to: 0,
        display,
    })
}

// Reads the table-column attributes. Creates as many copies as indicated.
fn read_table_col_attr(
    xml: &mut OdsXmlReader<'_>,
    sheet: &mut Sheet,
    super_tag: &BytesStart<'_>,
    table_col: u32,
) -> Result<u32, OdsError> {
    let mut col_repeat = 1;
    let mut col_header = None;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:number-columns-repeated" => {
                col_repeat = parse_u32(&attr.value)?;
            }
            attr if attr.key.as_ref() == b"table:style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                col_header.get_or_insert_with(ColHeader::default).style =
                    Some(ColStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"table:default-cell-style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                col_header.get_or_insert_with(ColHeader::default).cellstyle =
                    Some(CellStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"table:visibility" => {
                let visible = parse_visibility(&attr.value)?;
                col_header.get_or_insert_with(ColHeader::default).visible = visible;
            }
            attr => {
                unused_attr("read_table_col_attr", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    if let Some(mut col_header) = col_header {
        col_header.span = col_repeat;
        sheet.col_header.insert(table_col, col_header);
    }

    Ok(col_repeat)
}

#[derive(Debug)]
#[allow(variant_size_differences)]
enum TextContent {
    Empty,
    Text(String),
    Xml(TextTag),
    XmlVec(Vec<TextTag>),
}

#[derive(Debug)]
struct ReadTableCell {
    val_type: ValueType,
    val_datetime: Option<NaiveDateTime>,
    val_duration: Option<Duration>,
    val_float: Option<f64>,
    val_bool: Option<bool>,
    val_string: Option<String>,
    val_currency: Option<String>,

    content: TextContent,
}

fn read_table_cell(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    sheet: &mut Sheet,
    row: u32,
    col: u32,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(u32, bool), OdsError> {
    let mut cell = None;
    let mut repeat = 1;

    // find default-cell-style for this column.
    let default_cellstyle = if let Some(ch) = sheet.valid_col_header(col) {
        ch.cellstyle.as_ref()
    } else {
        None
    };

    let mut tc = ReadTableCell {
        val_type: ValueType::Empty,
        val_datetime: None,
        val_duration: None,
        val_float: None,
        val_bool: None,
        val_string: None,
        val_currency: None,
        content: TextContent::Empty,
    };

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:number-columns-repeated" => {
                repeat = parse_u32(&attr.value)?;
            }
            attr if attr.key.as_ref() == b"table:number-rows-spanned" => {
                let row_span = parse_u32(&attr.value)?;
                if row_span > 1 {
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .span
                        .row_span = row_span;
                }
            }
            attr if attr.key.as_ref() == b"table:number-columns-spanned" => {
                let col_span = parse_u32(&attr.value)?;
                if col_span > 1 {
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .span
                        .col_span = col_span;
                }
            }
            attr if attr.key.as_ref() == b"table:number-matrix-rows-spanned" => {
                let row_span = parse_u32(&attr.value)?;
                if row_span > 1 {
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .matrix_span
                        .row_span = row_span;
                }
            }
            attr if attr.key.as_ref() == b"table:number-matrix-columns-spanned" => {
                let col_span = parse_u32(&attr.value)?;
                if col_span > 1 {
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .matrix_span
                        .col_span = col_span;
                }
            }
            attr if attr.key.as_ref() == b"table:content-validation-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                cell.get_or_insert_with(CellData::default)
                    .extra_mut()
                    .validation_name = Some(ValidationRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"calcext:value-type" => {
                // not used. office:value-type seems to be good enough.
            }
            attr if attr.key.as_ref() == b"office:value-type" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_type = match attr.value.as_ref() {
                    b"string" => ValueType::Text,
                    b"float" => ValueType::Number,
                    b"percentage" => ValueType::Percentage,
                    b"date" => ValueType::DateTime,
                    b"time" => ValueType::TimeDuration,
                    b"boolean" => ValueType::Boolean,
                    b"currency" => ValueType::Currency,
                    other => {
                        return Err(OdsError::Parse(
                            "Unknown cell-type {:?}",
                            Some(from_utf8(other)?.into()),
                        ));
                    }
                }
            }
            attr if attr.key.as_ref() == b"office:date-value" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_datetime = Some(parse_datetime(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"office:time-value" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_duration = Some(parse_duration(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"office:value" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_float = Some(parse_f64(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"office:boolean-value" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_bool = Some(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"office:string-value" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_string = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref() == b"office:currency" => {
                cell.get_or_insert_with(CellData::default);
                tc.val_currency = Some(parse_currency(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:formula" => {
                cell.get_or_insert_with(CellData::default).formula =
                    Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref() == b"table:style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                cell.get_or_insert_with(CellData::default).style =
                    Some(CellStyleRef::from(name.as_ref()));
            }
            attr => {
                unused_attr("read_table_cell2", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_table_cell {:?}", evt);
            }
            match &evt {
                Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"text:p" => {}
                Event::Start(xml_tag) if xml_tag.name().as_ref() == b"text:p" => {
                    let new_txt = read_text_or_tag(ctx, xml, xml_tag, false)?;
                    tc.content = append_text(new_txt, tc.content);
                }

                Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:annotation" => {
                    let annotation = read_annotation(ctx, xml, xml_tag)?;
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .annotation = Some(annotation);
                }
                Event::Start(xml_tag) if xml_tag.name().as_ref() == b"draw:frame" => {
                    let draw_frame = read_draw_frame(ctx, xml, xml_tag)?;
                    cell.get_or_insert_with(CellData::default)
                        .extra_mut()
                        .draw_frames
                        .push(draw_frame);
                }

                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    break;
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_table_cell", &evt)?;
                }
            }

            buf.clear();
        }
        ctx.push_buf(buf);
    }

    let have_data = if let Some(mut cell) = cell {
        // composes a Value
        set_value(tc, &mut cell)?;

        // store cell-data
        if ignore_cell(ctx, default_cellstyle, &cell) {
            false
        } else {
            cell.repeat = repeat;
            sheet.add_cell_data(row, col, cell);
            true
        }
    } else {
        false
    };

    Ok((repeat, have_data))
}

#[allow(clippy::if_same_then_else)]
#[inline]
fn ignore_cell(
    ctx: &mut OdsContext,
    default_cellstyle: Option<&CellStyleRef>,
    cell: &CellData,
) -> bool {
    if cell.is_void(default_cellstyle) {
        return true;
    }
    if ctx.ignore_empty_cells && cell.is_empty() {
        return true;
    }
    false
}

fn append_text(new_txt: TextContent, mut content: TextContent) -> TextContent {
    // There can be multiple text:p elements within the cell.
    content = match content {
        TextContent::Empty => new_txt,
        TextContent::Text(txt) => {
            // Have a destructured text:p from before.
            // Wrap up and create list.
            let p = TextP::new().text(txt).into_xmltag();
            let mut vec = vec![p];

            match new_txt {
                TextContent::Empty => {}
                TextContent::Text(txt) => {
                    let p2 = TextP::new().text(txt).into_xmltag();
                    vec.push(p2);
                }
                TextContent::Xml(xml) => {
                    vec.push(xml);
                }
                TextContent::XmlVec(_) => {
                    unreachable!();
                }
            }
            TextContent::XmlVec(vec)
        }
        TextContent::Xml(xml) => {
            let mut vec = vec![xml];
            match new_txt {
                TextContent::Empty => {}
                TextContent::Text(txt) => {
                    let p2 = TextP::new().text(txt).into_xmltag();
                    vec.push(p2);
                }
                TextContent::Xml(xml) => {
                    vec.push(xml);
                }
                TextContent::XmlVec(_) => {
                    unreachable!();
                }
            }
            TextContent::XmlVec(vec)
        }
        TextContent::XmlVec(mut vec) => {
            match new_txt {
                TextContent::Empty => {}
                TextContent::Text(txt) => {
                    let p2 = TextP::new().text(txt).into_xmltag();
                    vec.push(p2);
                }
                TextContent::Xml(xml) => {
                    vec.push(xml);
                }
                TextContent::XmlVec(_) => {
                    unreachable!();
                }
            }
            TextContent::XmlVec(vec)
        }
    };

    content
}

#[inline(always)]
fn set_value(tc: ReadTableCell, cell: &mut CellData) -> Result<(), OdsError> {
    match tc.val_type {
        ValueType::Empty => {
            // noop
        }
        ValueType::Boolean => {
            if let Some(v) = tc.val_bool {
                cell.value = Value::Boolean(v);
            } else {
                return Err(OdsError::Parse("no boolean value", None));
            }
        }
        ValueType::Number => {
            if let Some(v) = tc.val_float {
                cell.value = Value::Number(v);
            } else {
                return Err(OdsError::Parse("no float value", None));
            }
        }
        ValueType::Percentage => {
            if let Some(v) = tc.val_float {
                cell.value = Value::Percentage(v);
            } else {
                return Err(OdsError::Parse("no float value", None));
            }
        }
        ValueType::Currency => {
            if let Some(v) = tc.val_float {
                if let Some(c) = tc.val_currency {
                    cell.value = Value::Currency(v, c.into_boxed_str());
                } else {
                    cell.value = Value::Currency(v, "".into());
                }
            } else {
                return Err(OdsError::Parse("no float value", None));
            }
        }
        ValueType::Text => {
            if let Some(v) = tc.val_string {
                cell.value = Value::Text(v);
            } else {
                match tc.content {
                    TextContent::Empty => {
                        // noop
                    }
                    TextContent::Text(txt) => {
                        cell.value = Value::Text(txt);
                    }
                    TextContent::Xml(xml) => {
                        cell.value = Value::TextXml(vec![xml]);
                    }
                    TextContent::XmlVec(vec) => {
                        cell.value = Value::TextXml(vec);
                    }
                }
            }
        }
        ValueType::TextXml => {
            unreachable!();
        }
        ValueType::DateTime => {
            if let Some(v) = tc.val_datetime {
                cell.value = Value::DateTime(v);
            } else {
                return Err(OdsError::Parse("no datetime value", None));
            }
        }
        ValueType::TimeDuration => {
            if let Some(v) = tc.val_duration {
                cell.value = Value::TimeDuration(v);
            } else {
                return Err(OdsError::Parse("no duration value", None));
            }
        }
    }

    Ok(())
}

fn read_annotation(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<Box<Annotation>, OdsError> {
    let mut annotation = Box::new(Annotation::new_empty());

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"office:display" => {
                annotation.set_display(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"office:name" => {
                annotation.set_name(attr.decode_and_unescape_value(xml)?);
            }
            attr => {
                let k = from_utf8(attr.key.as_ref())?;
                let v = attr.decode_and_unescape_value(xml)?.to_string();
                annotation.attrmap_mut().push_attr(k, v);
            }
        }
    }

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!("read_annotation {:?}", evt);
        }
        match &evt {
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:annotation" => {
                break;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"dc:creator" =>
            {
                annotation.set_creator(read_text(ctx, xml, xml_tag, empty_tag, parse_string)?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"dc:date" =>
            {
                annotation.set_date(read_text(ctx, xml, xml_tag, empty_tag, parse_datetime)?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"text:list"
                    || xml_tag.name().as_ref() == b"text:p" =>
            {
                annotation.push_text(read_xml(ctx, xml, xml_tag, empty_tag)?);
            }

            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_annotation", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(annotation)
}

fn read_draw_frame(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<DrawFrame, OdsError> {
    let mut draw_frame = DrawFrame::new();

    copy_attr2(xml, draw_frame.attrmap_mut(), super_tag)?;

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!("read_draw_frame {:?}", evt);
        }
        match &evt {
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"draw:frame" => {
                break;
            }
            Event::Empty(xml_tag) | Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"draw:image" =>
            {
                draw_frame.push_content(DrawFrameContent::Image(read_image(
                    ctx, xml, xml_tag, empty_tag,
                )?));
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"svg:desc" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"svg:desc" => {
                if let Some(v) = read_text(ctx, xml, xml_tag, empty_tag, parse_string)? {
                    draw_frame.set_desc(v);
                }
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"svg:title" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"svg:title" => {
                if let Some(v) = read_text(ctx, xml, xml_tag, empty_tag, parse_string)? {
                    draw_frame.set_title(v);
                }
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_draw_frame", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(draw_frame)
}

fn read_image(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<DrawImage, OdsError> {
    let mut draw_image = DrawImage::new();

    copy_attr2(xml, draw_image.attrmap_mut(), super_tag)?;

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            let empty_tag = matches!(evt, Event::Empty(_));
            if cfg!(feature = "dump_xml") {
                println!("read_image {:?}", evt);
            }
            match &evt {
                Event::End(xml_tag) if xml_tag.name().as_ref() == b"draw:image" => {
                    break;
                }

                Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:binary-data" => {
                    if let Some(v) = read_text(ctx, xml, xml_tag, empty_tag, parse_string)? {
                        draw_image.set_binary_base64(v);
                    }
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"text:list"
                        || xml_tag.name().as_ref() == b"text:p" =>
                {
                    draw_image.push_text(read_xml(ctx, xml, xml_tag, empty_tag)?);
                }

                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_image", &evt)?;
                }
            }

            buf.clear();
        }
        ctx.push_buf(buf);
    }

    Ok(draw_image)
}

fn read_scripts(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_scripts {:?}", evt);
        }
        match &evt {
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:scripts" => {
                break;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"office:script" =>
            {
                let script = read_script(ctx, xml, xml_tag)?;
                ctx.book.add_script(script);
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:event-listeners" => {}
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:event-listeners" => {}

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"script:event-listener" =>
            {
                ctx.book
                    .add_event_listener(read_event_listener(xml, xml_tag)?);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"script:event-listener" => {}

            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_scripts", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_script(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<Script, OdsError> {
    let v = read_xml(ctx, xml, super_tag, false)?;
    let script: Script = Script {
        script_lang: v
            .get_attr("script:language")
            .map(|v| v.to_string())
            .unwrap_or_default(),
        script: v.into_mixed_vec(),
    };
    Ok(script)
}

// reads the page-layout tag
fn read_event_listener(
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<EventListener, OdsError> {
    let mut evt = EventListener::new();
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"script:event-name" => {
                evt.event_name = attr.decode_and_unescape_value(xml)?.to_string();
            }
            attr if attr.key.as_ref() == b"script:language" => {
                evt.script_lang = attr.decode_and_unescape_value(xml)?.to_string();
            }
            attr if attr.key.as_ref() == b"script:macro-name" => {
                evt.macro_name = attr.decode_and_unescape_value(xml)?.to_string();
            }
            attr if attr.key.as_ref() == b"xlink:actuate" => {
                evt.actuate = parse_xlink_actuate(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr if attr.key.as_ref() == b"xlink:href" => {
                evt.href = attr.decode_and_unescape_value(xml)?.to_string();
            }
            attr if attr.key.as_ref() == b"xlink:type" => {
                evt.link_type = parse_xlink_type(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr => {
                unused_attr("read_event_listener", super_tag.name().as_ref(), &attr)?;
            }
        }
    }
    Ok(evt)
}

// reads a font-face
fn read_office_font_face_decls(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
) -> Result<(), OdsError> {
    let mut font: FontFaceDecl = FontFaceDecl::new_empty();
    font.set_origin(origin);

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_fonts {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:font-face" =>
            {
                let name = copy_style_attr(xml, font.attrmap_mut(), xml_tag)?;
                font.set_name(name);
                ctx.book.add_font(font);

                font = FontFaceDecl::new_empty();
                font.set_origin(StyleOrigin::Content);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:font-face-decls" => {
                break;
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_fonts", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// reads the page-layout tag
fn read_page_style(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    let mut pl = PageStyle::new_empty();
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:name" => {
                let value = attr.decode_and_unescape_value(xml)?;
                pl.set_name(value);
            }
            attr if attr.key.as_ref() == b"style:page-usage" => {
                let value = attr.decode_and_unescape_value(xml)?;
                pl.master_page_usage = Some(value.to_string());
            }
            attr => {
                unused_attr("read_page_style", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    let mut headerstyle = false;
    let mut footerstyle = false;

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_page_layout {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:page-layout-properties" =>
            {
                copy_attr2(xml, pl.style_mut(), xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:page-layout-properties" => {}

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:header-style" =>
            {
                headerstyle = true;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:header-style" => {
                headerstyle = false;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:footer-style" =>
            {
                footerstyle = true;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:footer-style" => {
                footerstyle = false;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:header-footer-properties" =>
            {
                if headerstyle {
                    copy_attr2(xml, pl.headerstyle_mut().style_mut(), xml_tag)?;
                }
                if footerstyle {
                    copy_attr2(xml, pl.footerstyle_mut().style_mut(), xml_tag)?;
                }
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:header-footer-properties" => {
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:background-image" =>
            {
                // noop for now. sets the background transparent.
            }

            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:page-layout" => {
                break;
            }
            Event::Text(_) => (),
            Event::Eof => break,
            _ => {
                unused_event("read_page_layout", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    ctx.book.add_pagestyle(pl);

    Ok(())
}

fn read_validations(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut valid = Validation::new();

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_validations {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:content-validation" => {
                read_validation(xml, &mut valid, xml_tag)?;
                ctx.book.add_validation(valid);
                valid = Validation::new();
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"table:content-validation" => {
                read_validation(xml, &mut valid, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:content-validation" => {
                ctx.book.add_validation(valid);
                valid = Validation::new();
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"table:error-message" =>
            {
                read_validation_error(ctx, xml, &mut valid, xml_tag, empty_tag)?;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"table:help-message" =>
            {
                read_validation_help(ctx, xml, &mut valid, xml_tag, empty_tag)?;
            }

            Event::End(xml_tag) if xml_tag.name().as_ref() == b"table:content-validations" => {
                break;
            }

            Event::Text(_) => (),
            Event::Eof => break,
            _ => {
                unused_event("read_validations", &evt)?;
            }
        }
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_validation_help(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    valid: &mut Validation,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut vh = ValidationHelp::new();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:display" => {
                vh.set_display(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:title" => {
                vh.set_title(Some(attr.decode_and_unescape_value(xml)?.to_string()));
            }
            attr => {
                unused_attr("read_validations", super_tag.name().as_ref(), &attr)?;
            }
        }
    }
    let txt = read_text_or_tag(ctx, xml, super_tag, empty_tag)?;
    match txt {
        TextContent::Empty => {}
        TextContent::Xml(txt) => {
            vh.set_text(Some(txt));
        }
        _ => {
            return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(format!(
                "table:help-message invalid {:?}",
                txt
            ))));
        }
    }

    valid.set_help(Some(vh));
    Ok(())
}

fn read_validation_error(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    valid: &mut Validation,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut ve = ValidationError::new();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:display" => {
                ve.set_display(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:message-type" => {
                let mt = match attr.value.as_ref() {
                    b"stop" => MessageType::Error,
                    b"warning" => MessageType::Warning,
                    b"information" => MessageType::Info,
                    _ => {
                        return Err(OdsError::Parse(
                            "unknown message-type",
                            Some(attr.decode_and_unescape_value(xml)?.into()),
                        ));
                    }
                };
                ve.set_msg_type(mt);
            }
            attr if attr.key.as_ref() == b"table:title" => {
                ve.set_title(Some(attr.decode_and_unescape_value(xml)?.to_string()));
            }
            attr => {
                unused_attr("read_validations", super_tag.name().as_ref(), &attr)?;
            }
        }
    }
    let txt = read_text_or_tag(ctx, xml, super_tag, empty_tag)?;
    match txt {
        TextContent::Empty => {}
        TextContent::Xml(txt) => {
            ve.set_text(Some(txt));
        }
        _ => {
            return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(format!(
                "table:error-message invalid {:?}",
                txt
            ))));
        }
    }

    valid.set_err(Some(ve));

    Ok(())
}

fn read_validation(
    xml: &mut OdsXmlReader<'_>,
    valid: &mut Validation,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"table:name" => {
                valid.set_name(attr.decode_and_unescape_value(xml)?);
            }
            attr if attr.key.as_ref() == b"table:condition" => {
                // split off 'of:' prefix
                let v = attr.decode_and_unescape_value(xml)?;
                valid.set_condition(Condition::new(v.split_at(3).1));
            }
            attr if attr.key.as_ref() == b"table:allow-empty-cell" => {
                valid.set_allow_empty(parse_bool(&attr.value)?);
            }
            attr if attr.key.as_ref() == b"table:base-cell-address" => {
                let v = attr.decode_and_unescape_value(xml)?;
                valid.set_base_cell(parse_cellref(&v)?);
            }
            attr if attr.key.as_ref() == b"table:display-list" => {
                valid.set_display(attr.value.as_ref().try_into()?);
            }
            attr => {
                unused_attr("read_validation", super_tag.name().as_ref(), &attr)?;
            }
        }
    }
    Ok(())
}

// read the master-styles tag
fn read_office_master_styles(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_master_styles {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:master-page" =>
            {
                read_master_page(ctx, xml, origin, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:master-styles" => {
                break;
            }
            Event::Text(_) => (),
            Event::Eof => break,
            _ => {
                unused_event("read_master_styles", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// read the master-page tag
fn read_master_page(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    _origin: StyleOrigin,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    let mut masterpage = MasterPage::new_empty();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:name" => {
                masterpage.set_name(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref() == b"style:page-layout-name" => {
                masterpage.set_pagestyle(&attr.decode_and_unescape_value(xml)?.as_ref().into());
            }
            attr if attr.key.as_ref() == b"style:display-name" => {
                masterpage.set_display_name(attr.decode_and_unescape_value(xml)?.as_ref().into());
            }
            attr if attr.key.as_ref() == b"style:next-style-name" => {
                let v = attr.decode_and_unescape_value(xml)?.to_string();
                masterpage.set_next_masterpage(&MasterPageRef::from(v));
            }
            attr => {
                unused_attr("read_master_page", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_master_page {:?}", evt);
        }
        match &evt {
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:header" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:header" => {
                masterpage.set_header(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:header-first" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:header-first" => {
                masterpage.set_header_first(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:header-left" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:header-left" => {
                masterpage.set_header_left(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:footer" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:footer" => {
                masterpage.set_footer(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:footer-first" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:footer-first" => {
                masterpage.set_footer_first(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"style:footer-left" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"style:footer-left" => {
                masterpage.set_footer_left(read_headerfooter(ctx, xml, xml_tag)?);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:master-page" => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_master_page", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    ctx.book.add_masterpage(masterpage);

    Ok(())
}

// reads any header or footer tags
fn read_headerfooter(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<HeaderFooter, OdsError> {
    let mut hf = HeaderFooter::new();
    let mut content = TextContent::Empty;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:display" => {
                hf.set_display(parse_bool(&attr.value)?);
            }
            attr => {
                unused_attr("read_headerfooter", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_headerfooter {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:region-left" =>
            {
                let reg = read_xml(ctx, xml, xml_tag, empty_tag)?;
                hf.set_left(reg.into_vec()?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:region-center" =>
            {
                let reg = read_xml(ctx, xml, xml_tag, empty_tag)?;
                hf.set_center(reg.into_vec()?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:region-right" =>
            {
                let reg = read_xml(ctx, xml, xml_tag, empty_tag)?;
                hf.set_right(reg.into_vec()?);
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"text:p" =>
            {
                let new_txt = read_text_or_tag(ctx, xml, xml_tag, empty_tag)?;
                content = append_text(new_txt, content);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"text:h" =>
            {
                let new_txt = read_text_or_tag(ctx, xml, xml_tag, empty_tag)?;
                content = append_text(new_txt, content);
            }
            // no other tags supported for now. they have never been seen in the wild.
            Event::Text(_) => (),
            Event::End(xml_tag) => {
                if xml_tag.name() == super_tag.name() {
                    hf.set_content(match content {
                        TextContent::Empty => Vec::new(),
                        TextContent::Text(v) => vec![TextP::new().text(v).into()],
                        TextContent::Xml(v) => vec![v],
                        TextContent::XmlVec(v) => v,
                    });
                    break;
                }
            }
            Event::Eof => break,
            _ => {
                unused_event("read_headerfooter", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(hf)
}

// reads the office-styles tag
fn read_office_styles(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_styles_tag {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:style" =>
            {
                read_style_style(ctx, xml, origin, StyleUse::Named, xml_tag, empty_tag)?;
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:default-style" =>
            {
                read_style_style(ctx, xml, origin, StyleUse::Default, xml_tag, empty_tag)?;
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:boolean-style"
                    || xml_tag.name().as_ref() == b"number:date-style"
                    || xml_tag.name().as_ref() == b"number:time-style"
                    || xml_tag.name().as_ref() == b"number:number-style"
                    || xml_tag.name().as_ref() == b"number:currency-style"
                    || xml_tag.name().as_ref() == b"number:percentage-style"
                    || xml_tag.name().as_ref() == b"number:text-style" =>
            {
                read_value_format(ctx, xml, origin, StyleUse::Named, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:styles" => {
                break;
            }
            Event::Text(_) => (),
            Event::Eof => break,
            _ => {
                unused_event("read_styles_tag", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// read the automatic-styles tag
fn read_office_automatic_styles(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_auto_styles {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:style" =>
            {
                read_style_style(ctx, xml, origin, StyleUse::Automatic, xml_tag, empty_tag)?;
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:boolean-style"
                    || xml_tag.name().as_ref() == b"number:date-style"
                    || xml_tag.name().as_ref() == b"number:time-style"
                    || xml_tag.name().as_ref() == b"number:number-style"
                    || xml_tag.name().as_ref() == b"number:currency-style"
                    || xml_tag.name().as_ref() == b"number:percentage-style"
                    || xml_tag.name().as_ref() == b"number:text-style" =>
            {
                read_value_format(ctx, xml, origin, StyleUse::Automatic, xml_tag)?;
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:page-layout" =>
            {
                read_page_style(ctx, xml, xml_tag)?;
            }

            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:automatic-styles" => {
                break;
            }
            Event::Text(_) => (),
            Event::Eof => break,
            _ => {
                unused_event("read_auto_styles", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// Reads any of the number:xxx tags
fn read_value_format(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    styleuse: StyleUse,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    match super_tag.name().as_ref() {
        b"number:boolean-style" => {
            let mut valuestyle = ValueFormatBoolean::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_boolean_format(valuestyle);
        }
        b"number:date-style" => {
            let mut valuestyle = ValueFormatDateTime::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_datetime_format(valuestyle);
        }
        b"number:time-style" => {
            let mut valuestyle = ValueFormatTimeDuration::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_timeduration_format(valuestyle);
        }
        b"number:number-style" => {
            let mut valuestyle = ValueFormatNumber::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_number_format(valuestyle);
        }
        b"number:currency-style" => {
            let mut valuestyle = ValueFormatCurrency::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_currency_format(valuestyle);
        }
        b"number:percentage-style" => {
            let mut valuestyle = ValueFormatPercentage::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_percentage_format(valuestyle);
        }
        b"number:text-style" => {
            let mut valuestyle = ValueFormatText::new_empty();
            read_value_format_parts(ctx, xml, origin, styleuse, &mut valuestyle, super_tag)?;
            ctx.book.add_text_format(valuestyle);
        }
        _ => {
            if cfg!(feature = "dump_unused") {
                println!(
                    " read_value_format unused {}",
                    from_utf8(super_tag.name().as_ref())?
                );
            }
        }
    }

    Ok(())
}

// Reads any of the number:xxx tags
fn read_value_format_parts<T: ValueFormatTrait>(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    styleuse: StyleUse,
    valuestyle: &mut T,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    //
    valuestyle.set_origin(origin);
    valuestyle.set_styleuse(styleuse);
    let name = copy_style_attr(xml, valuestyle.attrmap_mut(), super_tag)?;
    valuestyle.set_name(name.as_str());

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        let empty_tag = matches!(evt, Event::Empty(_));
        if cfg!(feature = "dump_xml") {
            println!(" read_value_format_parts {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:boolean" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Boolean,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:number" =>
            {
                valuestyle.push_part(read_part_number(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Number,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:fraction" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Fraction,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:scientific-number" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::ScientificNumber,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:text"
                    || xml_tag.name().as_ref() == b"loext:text" =>
            {
                valuestyle.push_part(read_part_text(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Text,
                )?);
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:am-pm" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::AmPm,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:day" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Day,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:day-of-week" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::DayOfWeek,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:era" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Era,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:hours" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Hours,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:minutes" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Minutes,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:month" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Month,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:quarter" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Quarter,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:seconds" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Seconds,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:week-of-year" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::WeekOfYear,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:year" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::Year,
                )?);
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:currency-symbol" =>
            {
                valuestyle.push_part(read_part_text(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::CurrencySymbol,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:fill-character"
                    || xml_tag.name().as_ref() == b"loext:fill-character" =>
            {
                valuestyle.push_part(read_part_text(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::FillCharacter,
                )?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"number:text-content" =>
            {
                valuestyle.push_part(read_part(
                    ctx,
                    xml,
                    xml_tag,
                    empty_tag,
                    FormatPartType::TextContent,
                )?);
            }

            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:map" =>
            {
                valuestyle.push_stylemap(read_value_stylemap(xml, xml_tag)?);
            }
            Event::Start(xml_tag) | Event::Empty(xml_tag)
                if xml_tag.name().as_ref() == b"style:text-properties" =>
            {
                copy_attr2(xml, valuestyle.textstyle_mut(), xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_value_format_parts", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_part(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
    part_type: FormatPartType,
) -> Result<FormatPart, OdsError> {
    let mut part = FormatPart::new(part_type);
    copy_attr2(xml, part.attrmap_mut(), super_tag)?;

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_part {:?}", evt);
            }
            match &evt {
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    break;
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_part", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(part)
}

// value format part with text content
fn read_part_text(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
    part_type: FormatPartType,
) -> Result<FormatPart, OdsError> {
    let mut part = FormatPart::new(part_type);
    copy_attr2(xml, part.attrmap_mut(), super_tag)?;

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_part_text {:?}", evt);
            }
            match &evt {
                Event::Text(xml_text) => {
                    part.set_content(xml_text.unescape()?);
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    break;
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_part_text", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(part)
}

fn read_part_number(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
    part_type: FormatPartType,
) -> Result<FormatPart, OdsError> {
    let mut part = FormatPart::new(part_type);
    copy_attr2(xml, part.attrmap_mut(), super_tag)?;

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_part_embedded_text {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"number:embedded-text" =>
                {
                    for attr in xml_tag.attributes().with_checks(false) {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"number:position" => {
                                part.set_position(parse_i32(&attr.value)?);
                            }
                            _ => {
                                unused_attr(
                                    "read_part_embedded_text",
                                    xml_tag.name().as_ref(),
                                    &attr,
                                )?;
                            }
                        }
                    }
                }
                Event::End(xml_tag) if xml_tag.name().as_ref() == b"number:embedded-text" => {}
                Event::Text(xml_text) => {
                    part.set_content(parse_string(xml_text.as_ref())?);
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    break;
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_part_embedded_text", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(part)
}

// style:style tag
#[allow(clippy::too_many_arguments)]
fn read_style_style(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:family" => {
                match attr.value.as_ref() {
                    b"table" => read_tablestyle(ctx, xml, origin, style_use, super_tag, empty_tag)?,
                    b"table-column" => {
                        read_colstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?
                    }
                    b"table-row" => {
                        read_rowstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?
                    }
                    b"table-cell" => {
                        read_cellstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?
                    }
                    b"graphic" => {
                        read_graphicstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?
                    }
                    b"paragraph" => {
                        read_paragraphstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?
                    }
                    b"text" => read_textstyle(ctx, xml, origin, style_use, super_tag, empty_tag)?,
                    b"ruby" => read_rubystyle(ctx, xml, origin, style_use, super_tag, empty_tag)?,
                    value => {
                        return Err(OdsError::Ods(format!(
                            "style:family unknown {} ",
                            from_utf8(value)?
                        )));
                    }
                };
            }
            _ => {
                // not read here
            }
        }
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_tablestyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = TableStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_tablestyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_table_style {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag) => match xml_tag.name().as_ref() {
                    b"style:table-properties" => copy_attr2(xml, style.tablestyle_mut(), xml_tag)?,
                    _ => {
                        unused_event("read_table_style", &evt)?;
                    }
                },
                Event::Text(_) => (),
                Event::End(xml_tag) => {
                    if xml_tag.name().as_ref() == super_tag.name().as_ref() {
                        ctx.book.add_tablestyle(style);
                        break;
                    } else {
                        unused_event("read_table_style", &evt)?;
                    }
                }
                Event::Eof => break,
                _ => {
                    unused_event("read_table_style", &evt)?;
                }
            }
        }

        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_rowstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = RowStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_rowstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_rowstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag) => match xml_tag.name().as_ref() {
                    b"style:table-row-properties" => {
                        copy_attr2(xml, style.rowstyle_mut(), xml_tag)?
                    }
                    _ => {
                        unused_event("read_rowstyle", &evt)?;
                    }
                },
                Event::Text(_) => (),
                Event::End(xml_tag) => {
                    if xml_tag.name() == super_tag.name() {
                        ctx.book.add_rowstyle(style);
                        break;
                    } else {
                        unused_event("read_rowstyle", &evt)?;
                    }
                }
                Event::Eof => break,
                _ => {
                    unused_event("read_rowstyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_colstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = ColStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_colstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_colstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag) => match xml_tag.name().as_ref() {
                    b"style:table-column-properties" => {
                        copy_attr2(xml, style.colstyle_mut(), xml_tag)?
                    }
                    _ => {
                        unused_event("read_colstyle", &evt)?;
                    }
                },
                Event::Text(_) => (),
                Event::End(xml_tag) => {
                    if xml_tag.name() == super_tag.name() {
                        ctx.book.add_colstyle(style);
                        break;
                    } else {
                        unused_event("read_colstyle", &evt)?;
                    }
                }
                Event::Eof => break,
                _ => {
                    unused_event("read_colstyle", &evt)?;
                }
            }
        }

        ctx.push_buf(buf);
    }
    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_cellstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = CellStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_cellstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_cellstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:table-cell-properties" =>
                {
                    copy_attr2(xml, style.cellstyle_mut(), xml_tag)?;
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:text-properties" =>
                {
                    copy_attr2(xml, style.textstyle_mut(), xml_tag)?;
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:paragraph-properties" =>
                {
                    copy_attr2(xml, style.paragraphstyle_mut(), xml_tag)?;
                }
                Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:paragraph-properties" => {
                }
                // Event::Start(xml_tag) | Event::Empty(xml_tag)
                //     if xml_tag.name().as_ref() == b"style:graphic-properties" =>
                // {
                //     copy_attr(style.graphic_mut(), xml, xml_tag)?;
                // }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:map" =>
                {
                    style.push_stylemap(read_stylemap(xml, xml_tag)?);
                }
                // todo: tab-stops
                // b"style:tab-stops" => (),
                // b"style:tab-stop" => {
                //     let mut ts = TabStop::new();
                //     copy_attr(&mut ts, xml, xml_tag)?;
                //     style.paragraph_mut().add_tabstop(ts);
                // }
                Event::Text(_) => (),
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    ctx.book.add_cellstyle(style);
                    break;
                }
                Event::Eof => break,
                _ => {
                    unused_event("read_cellstyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_paragraphstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = ParagraphStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_paragraphstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_paragraphstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:text-properties" =>
                {
                    copy_attr2(xml, style.textstyle_mut(), xml_tag)?;
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:paragraph-properties" =>
                {
                    copy_attr2(xml, style.paragraphstyle_mut(), xml_tag)?;
                }
                Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:paragraph-properties" => {
                }
                // b"style:graphic-properties" => copy_attr(style.graphic_mut(), xml, xml_tag)?,
                // b"style:map" => style.push_stylemap(read_stylemap(xml, xml_tag)?),
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:tab-stops" => {}
                Event::End(xml_tag) if xml_tag.name().as_ref() == b"style:tab-stops" => {}
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:tab-stop" =>
                {
                    let mut ts = TabStop::new();
                    copy_attr2(xml, ts.attrmap_mut(), xml_tag)?;
                    style.add_tabstop(ts);
                }

                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    ctx.book.add_paragraphstyle(style);
                    break;
                }

                Event::Text(_) => (),
                Event::Eof => break,
                _ => {
                    unused_event("read_paragraphstyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_textstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = TextStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_textstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_textstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:text-properties" =>
                {
                    copy_attr2(xml, style.textstyle_mut(), xml_tag)?;
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    ctx.book.add_textstyle(style);
                    break;
                }
                Event::Text(_) => (),
                Event::Eof => break,
                _ => {
                    unused_event("read_textstyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_rubystyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = RubyStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_rubystyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_rubystyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:ruby-properties" =>
                {
                    copy_attr2(xml, style.rubystyle_mut(), xml_tag)?;
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    ctx.book.add_rubystyle(style);
                    break;
                }
                Event::Text(_) => (),
                Event::Eof => break,
                _ => {
                    unused_event("read_rubystyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:style tag
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::too_many_arguments)]
fn read_graphicstyle(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    origin: StyleOrigin,
    style_use: StyleUse,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<(), OdsError> {
    let mut style = GraphicStyle::new_empty();
    style.set_origin(origin);
    style.set_styleuse(style_use);
    let name = copy_style_attr(xml, style.attrmap_mut(), super_tag)?;
    style.set_name(name);

    // In case of an empty xml-tag we are done here.
    if empty_tag {
        ctx.book.add_graphicstyle(style);
    } else {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_graphicstyle {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:graphic-properties" =>
                {
                    copy_attr2(xml, style.graphicstyle_mut(), xml_tag)?;
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:paragraph-properties" =>
                {
                    copy_attr2(xml, style.paragraphstyle_mut(), xml_tag)?;
                }
                Event::Start(xml_tag) | Event::Empty(xml_tag)
                    if xml_tag.name().as_ref() == b"style:text-properties" =>
                {
                    copy_attr2(xml, style.textstyle_mut(), xml_tag)?;
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    ctx.book.add_graphicstyle(style);
                    break;
                }
                Event::Text(_) => (),
                Event::Eof => break,
                _ => {
                    unused_event("read_graphicstyle", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    Ok(())
}

// style:map inside a number style.
fn read_value_stylemap(
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<ValueStyleMap, OdsError> {
    let mut sm = ValueStyleMap::default();
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:condition" => {
                sm.set_condition(ValueCondition::new(
                    attr.decode_and_unescape_value(xml)?.to_string(),
                ));
            }
            attr if attr.key.as_ref() == b"style:apply-style-name" => {
                sm.set_applied_style(attr.decode_and_unescape_value(xml)?);
            }
            attr => {
                unused_attr("read_value_stylemap", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    Ok(sm)
}

fn read_stylemap(
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<StyleMap, OdsError> {
    let mut sm = StyleMap::new_empty();
    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:condition" => {
                sm.set_condition(Condition::new(
                    attr.decode_and_unescape_value(xml)?.to_string(),
                ));
            }
            attr if attr.key.as_ref() == b"style:apply-style-name" => {
                let name = attr.decode_and_unescape_value(xml)?;
                sm.set_applied_style(AnyStyleRef::from(name.as_ref()));
            }
            attr if attr.key.as_ref() == b"style:base-cell-address" => {
                let v = attr.decode_and_unescape_value(xml)?;
                sm.set_base_cell(Some(parse_cellref(v.as_ref())?));
            }
            attr => {
                unused_attr("read_stylemap", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    Ok(sm)
}

/// Copies all attributes to the map, excluding "style:name" which is returned.
fn copy_style_attr(
    xml: &mut OdsXmlReader<'_>,
    attrmap: &mut AttrMap2,
    super_tag: &BytesStart<'_>,
) -> Result<String, OdsError> {
    let mut name = None;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"style:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr => {
                let k = from_utf8(attr.key.as_ref())?;
                let v = attr.decode_and_unescape_value(xml)?.to_string();
                attrmap.push_attr(k, v);
            }
        }
    }

    Ok(name.unwrap_or_default())
}

/// Copies all attributes to the given map.
fn copy_attr2(
    xml: &mut OdsXmlReader<'_>,
    attrmap: &mut AttrMap2,
    super_tag: &BytesStart<'_>,
) -> Result<(), OdsError> {
    for attr in super_tag.attributes().with_checks(false) {
        let attr = attr?;

        let k = from_utf8(attr.key.as_ref())?;
        let v = attr.decode_and_unescape_value(xml)?.to_string();
        attrmap.push_attr(k, v);
    }

    Ok(())
}

fn read_ods_styles(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_styles {:?}", evt);
        }
        match &evt {
            Event::Decl(_) => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:document-styles" => {
                let (_, xmlns) = read_namespaces_and_version(xml, xml_tag)?;
                ctx.book.xmlns.insert("styles.xml".to_string(), xmlns);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:document-styles" => {
                // noop
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:font-face-decls" => {
                read_office_font_face_decls(ctx, xml, StyleOrigin::Styles)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:styles" => {
                read_office_styles(ctx, xml, StyleOrigin::Styles)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:automatic-styles" => {
                read_office_automatic_styles(ctx, xml, StyleOrigin::Styles)?
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:master-styles" => {
                read_office_master_styles(ctx, xml, StyleOrigin::Styles)?
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_styles", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

#[allow(unused_variables)]
pub(crate) fn default_settings() -> Detach<Config> {
    let mut dc = Detach::new(Config::new());
    let p0 = dc.create_path(&[("ooo:view-settings", ConfigItemType::Set)]);
    p0.insert("VisibleAreaTop", 0);
    p0.insert("VisibleAreaLeft", 0);
    p0.insert("VisibleAreaWidth", 2540);
    p0.insert("VisibleAreaHeight", 1270);

    let p0 = dc.create_path(&[
        ("ooo:view-settings", ConfigItemType::Set),
        ("Views", ConfigItemType::Vec),
        ("0", ConfigItemType::Entry),
    ]);
    p0.insert("ViewId", "view1");
    let p0 = dc.create_path(&[
        ("ooo:view-settings", ConfigItemType::Set),
        ("Views", ConfigItemType::Vec),
        ("0", ConfigItemType::Entry),
        ("Tables", ConfigItemType::Map),
    ]);
    let p0 = dc.create_path(&[
        ("ooo:view-settings", ConfigItemType::Set),
        ("Views", ConfigItemType::Vec),
        ("0", ConfigItemType::Entry),
    ]);
    p0.insert("ActiveTable", "");
    p0.insert("HorizontalScrollbarWidth", 702);
    p0.insert("ZoomType", 0i16);
    p0.insert("ZoomValue", 100);
    p0.insert("PageViewZoomValue", 60);
    p0.insert("ShowPageBreakPreview", false);
    p0.insert("ShowZeroValues", true);
    p0.insert("ShowNotes", true);
    p0.insert("ShowGrid", true);
    p0.insert("GridColor", 12632256);
    p0.insert("ShowPageBreaks", false);
    p0.insert("HasColumnRowHeaders", true);
    p0.insert("HasSheetTabs", true);
    p0.insert("IsOutlineSymbolsSet", true);
    p0.insert("IsValueHighlightingEnabled", false);
    p0.insert("IsSnapToRaster", false);
    p0.insert("RasterIsVisible", false);
    p0.insert("RasterResolutionX", 1000);
    p0.insert("RasterResolutionY", 1000);
    p0.insert("RasterSubdivisionX", 1);
    p0.insert("RasterSubdivisionY", 1);
    p0.insert("IsRasterAxisSynchronized", true);
    p0.insert("AnchoredTextOverflowLegacy", false);

    let p0 = dc.create_path(&[("ooo:configuration-settings", ConfigItemType::Set)]);
    p0.insert("HasSheetTabs", true);
    p0.insert("ShowNotes", true);
    p0.insert("EmbedComplexScriptFonts", true);
    p0.insert("ShowZeroValues", true);
    p0.insert("ShowGrid", true);
    p0.insert("GridColor", 12632256);
    p0.insert("ShowPageBreaks", false);
    p0.insert("IsKernAsianPunctuation", false);
    p0.insert("LinkUpdateMode", 3i16);
    p0.insert("HasColumnRowHeaders", true);
    p0.insert("EmbedLatinScriptFonts", true);
    p0.insert("IsOutlineSymbolsSet", true);
    p0.insert("EmbedLatinScriptFonts", true);
    p0.insert("IsOutlineSymbolsSet", true);
    p0.insert("IsSnapToRaster", false);
    p0.insert("RasterIsVisible", false);
    p0.insert("RasterResolutionX", 1000);
    p0.insert("RasterResolutionY", 1000);
    p0.insert("RasterSubdivisionX", 1);
    p0.insert("RasterSubdivisionY", 1);
    p0.insert("IsRasterAxisSynchronized", true);
    p0.insert("AutoCalculate", true);
    p0.insert("ApplyUserData", true);
    p0.insert("PrinterName", "");
    p0.insert("PrinterSetup", ConfigValue::Base64Binary("".to_string()));
    p0.insert("SaveThumbnail", true);
    p0.insert("CharacterCompressionType", 0i16);
    p0.insert("SaveVersionOnClose", false);
    p0.insert("UpdateFromTemplate", true);
    p0.insert("AllowPrintJobCancel", true);
    p0.insert("LoadReadonly", false);
    p0.insert("IsDocumentShared", false);
    p0.insert("EmbedFonts", false);
    p0.insert("EmbedOnlyUsedFonts", false);
    p0.insert("EmbedAsianScriptFonts", true);
    p0.insert("SyntaxStringRef", 7i16);

    dc
}

fn read_ods_metadata(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();

    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_ods_metadata {:?}", evt);
        }

        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:document-meta" => {
                let (_, xmlns) = read_namespaces_and_version(xml, xml_tag)?;
                ctx.book.xmlns.insert("meta.xml".to_string(), xmlns);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:document-meta" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:meta" => {
                read_office_meta(ctx, xml)?;
            }

            Event::Decl(_) => {}
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_ods_metadata", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_office_meta(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();

    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_metadata {:?}", evt);
        }

        match &evt {
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:meta" => {
                break;
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:generator" => {
                ctx.book.metadata.generator = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:title" => {
                ctx.book.metadata.title = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:description" => {
                ctx.book.metadata.description = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:subject" => {
                ctx.book.metadata.subject = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:keyword" => {
                ctx.book.metadata.keyword = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:initial-creator" => {
                ctx.book.metadata.initial_creator = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:creator" => {
                ctx.book.metadata.creator = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:printed-by" => {
                ctx.book.metadata.printed_by = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:creation-date" => {
                ctx.book.metadata.creation_date = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(Some(parse_datetime(v)?)),
                    || None,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:date" => {
                ctx.book.metadata.date = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(Some(parse_datetime(v)?)),
                    || None,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:print-date" => {
                ctx.book.metadata.print_date = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(Some(parse_datetime(v)?)),
                    || None,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"dc:language" => {
                ctx.book.metadata.language = read_metadata_value(
                    ctx,
                    xml,
                    xml_tag,
                    |v| Ok(from_utf8(v)?.to_string()),
                    String::new,
                )?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:editing-cycles" => {
                ctx.book.metadata.editing_cycles =
                    read_metadata_value(ctx, xml, xml_tag, parse_u32, || 0)?;
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:editing-duration" => {
                ctx.book.metadata.editing_duration =
                    read_metadata_value(ctx, xml, xml_tag, parse_duration, || {
                        Duration::default()
                    })?;
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:template" => {
                ctx.book.metadata.template = read_metadata_template(xml, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"meta:template" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:auto-reload" => {
                ctx.book.metadata.auto_reload = read_metadata_auto_reload(xml, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"meta:auto-reload" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:hyperlink-behaviour" => {
                ctx.book.metadata.hyperlink_behaviour =
                    read_metadata_hyperlink_behaviour(xml, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"meta:hyperlink-behaviour" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:document-statistic" => {
                ctx.book.metadata.document_statistics =
                    read_metadata_document_statistics(xml, xml_tag)?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"meta:document-statistic" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"meta:user-defined" => {
                let userdefined = read_metadata_user_defined(ctx, xml, xml_tag)?;
                ctx.book.metadata.user_defined.push(userdefined);
            }

            Event::Empty(_) => {}
            Event::Text(_) => {}
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_metadata", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

fn read_metadata_template(
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
) -> Result<MetaTemplate, OdsError> {
    let mut template = MetaTemplate::default();

    for attr in tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"meta:date" => {
                template.date = Some(parse_datetime(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr if attr.key.as_ref() == b"xlink:actuate" => {
                template.actuate = Some(parse_xlink_actuate(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr if attr.key.as_ref() == b"xlink:href" => {
                template.href = Some(attr.decode_and_unescape_value(xml)?.to_string())
            }
            attr if attr.key.as_ref() == b"xlink:title" => {
                template.title = Some(attr.decode_and_unescape_value(xml)?.to_string())
            }
            attr if attr.key.as_ref() == b"xlink:type" => {
                template.link_type = Some(parse_xlink_type(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr => {
                unused_attr("read_metadata_template", tag.name().as_ref(), &attr)?;
            }
        }
    }

    Ok(template)
}

fn read_metadata_auto_reload(
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
) -> Result<MetaAutoReload, OdsError> {
    let mut auto_reload = MetaAutoReload::default();

    for attr in tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"meta:delay" => {
                auto_reload.delay = Some(parse_duration(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr if attr.key.as_ref() == b"xlink:actuate" => {
                auto_reload.actuate = Some(parse_xlink_actuate(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr if attr.key.as_ref() == b"xlink:href" => {
                auto_reload.href = Some(attr.decode_and_unescape_value(xml)?.to_string())
            }
            attr if attr.key.as_ref() == b"xlink:show" => {
                auto_reload.show = Some(parse_xlink_show(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr if attr.key.as_ref() == b"xlink:type" => {
                auto_reload.link_type = Some(parse_xlink_type(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr => {
                unused_attr("read_metadata_auto_reload", tag.name().as_ref(), &attr)?;
            }
        }
    }

    Ok(auto_reload)
}

fn read_metadata_hyperlink_behaviour(
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
) -> Result<MetaHyperlinkBehaviour, OdsError> {
    let mut hyperlink_behaviour = MetaHyperlinkBehaviour::default();

    for attr in tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"office:targetframe-name" => {
                hyperlink_behaviour.target_frame_name =
                    Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref() == b"xlink:show" => {
                hyperlink_behaviour.show = Some(parse_xlink_show(
                    attr.decode_and_unescape_value(xml)?.as_bytes(),
                )?);
            }
            attr => {
                unused_attr(
                    "read_metadata_hyperlink_behaviour",
                    tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    Ok(hyperlink_behaviour)
}

fn read_metadata_document_statistics(
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
) -> Result<MetaDocumentStatistics, OdsError> {
    let mut document_statistics = MetaDocumentStatistics::default();

    for attr in tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"meta:cell-count" => {
                document_statistics.cell_count =
                    parse_u32(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr if attr.key.as_ref() == b"meta:object-count" => {
                document_statistics.object_count =
                    parse_u32(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr if attr.key.as_ref() == b"meta:ole-object-count" => {
                document_statistics.ole_object_count =
                    parse_u32(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr if attr.key.as_ref() == b"meta:table-count" => {
                document_statistics.table_count =
                    parse_u32(attr.decode_and_unescape_value(xml)?.as_bytes())?;
            }
            attr => {
                unused_attr(
                    "read_metadata_document_statistics",
                    tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    Ok(document_statistics)
}

fn read_metadata_user_defined(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
) -> Result<MetaUserDefined, OdsError> {
    let mut user_defined = MetaUserDefined::default();
    let mut value_type = None;
    for attr in tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"meta:name" => {
                user_defined.name = attr.decode_and_unescape_value(xml)?.to_string();
            }
            attr if attr.key.as_ref() == b"meta:value-type" => {
                value_type = Some(match attr.decode_and_unescape_value(xml)?.as_ref() {
                    "boolean" => "boolean",
                    "date" => "date",
                    "float" => "float",
                    "time" => "time",
                    _ => "string",
                });
            }
            attr => {
                unused_attr("read_meta_user_defined", tag.name().as_ref(), &attr)?;
            }
        }
    }

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_meta_user_defined {:?}", evt);
        }

        match &evt {
            Event::End(xml_tag) if xml_tag.name() == tag.name() => {
                break;
            }
            Event::Text(xml_text) => {
                user_defined.value = match value_type {
                    Some("boolean") => {
                        MetaValue::Boolean(parse_bool(xml_text.unescape()?.as_bytes())?)
                    }
                    Some("date") => {
                        MetaValue::Datetime(parse_datetime(xml_text.unescape()?.as_bytes())?)
                    }
                    Some("float") => MetaValue::Float(parse_f64(xml_text.unescape()?.as_bytes())?),
                    Some("time") => {
                        MetaValue::TimeDuration(parse_duration(xml_text.unescape()?.as_bytes())?)
                    }
                    _ => MetaValue::String(xml_text.unescape()?.to_string()),
                };
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_meta_user_defined", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(user_defined)
}

// Parse a metadata value.
fn read_metadata_value<T>(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    tag: &BytesStart<'_>,
    parse: fn(&[u8]) -> Result<T, OdsError>,
    default: fn() -> T,
) -> Result<T, OdsError> {
    let mut buf = ctx.pop_buf();
    let mut value = None;
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!("read_metadata_value {:?}", evt);
        }

        match &evt {
            Event::End(xml_tag) if xml_tag.name() == tag.name() => {
                break;
            }
            Event::Text(xml_text) => {
                value = Some(parse(xml_text.unescape()?.as_bytes())?);
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_metadata_value", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(value.unwrap_or(default()))
}

fn read_ods_settings(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_settings {:?}", evt);
        }

        match &evt {
            Event::Decl(_) => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:document-settings" => {
                let (_, xmlns) = read_namespaces_and_version(xml, xml_tag)?;
                ctx.book.xmlns.insert("settings.xml".to_string(), xmlns);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:document-settings" => {}

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"office:settings" => {
                read_office_settings(ctx, xml)?;
            }

            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_settings", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok(())
}

// read the automatic-styles tag
fn read_office_settings(ctx: &mut OdsContext, xml: &mut OdsXmlReader<'_>) -> Result<(), OdsError> {
    let mut config = Config::new();

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_office_settings {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-set" => {
                let (name, set) = read_config_item_set(ctx, xml, xml_tag)?;
                config.insert(name, set);
            }

            Event::End(xml_tag) if xml_tag.name().as_ref() == b"office:settings" => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_office_settings", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    ctx.book.config = Detach::new(config);

    Ok(())
}

// read the automatic-styles tag
fn read_config_item_set(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(String, ConfigItem), OdsError> {
    let mut name = None;
    let mut config_set = ConfigItem::new_set();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"config:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr => {
                unused_attr("read_config_item_set", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    let name = if let Some(name) = name {
        name
    } else {
        return Err(OdsError::Ods("config-item-set without name".to_string()));
    };

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_office_item_set {:?}", evt);
        }
        match &evt {
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"config:config-item" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item" => {
                let (name, val) = read_config_item(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-set" => {
                let (name, val) = read_config_item_set(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }
            Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"config:config-item-map-indexed" =>
            {
                let (name, val) = read_config_item_map_indexed(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-named" => {
                let (name, val) = read_config_item_map_named(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-set" => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_config_item_set", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok((name, config_set))
}

// read the automatic-styles tag
fn read_config_item_map_indexed(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(String, ConfigItem), OdsError> {
    let mut name = None;
    let mut config_vec = ConfigItem::new_vec();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"config:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr => {
                unused_attr(
                    "read_config_item_map_indexed",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    let name = if let Some(name) = name {
        name
    } else {
        return Err(OdsError::Ods(
            "config-item-map-indexed without name".to_string(),
        ));
    };

    let mut index = 0;

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_office_item_set {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-entry" => {
                let (_, entry) = read_config_item_map_entry(ctx, xml, xml_tag)?;
                config_vec.insert(index.to_string(), entry);
                index += 1;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-indexed" => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_config_item_map_indexed", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok((name, config_vec))
}

// read the automatic-styles tag
fn read_config_item_map_named(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(String, ConfigItem), OdsError> {
    let mut name = None;
    let mut config_map = ConfigItem::new_map();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"config:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr => {
                unused_attr(
                    "read_config_item_map_named",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    let name = if let Some(name) = name {
        name
    } else {
        return Err(OdsError::Ods(
            "config-item-map-named without name".to_string(),
        ));
    };

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_config_item_map_named {:?}", evt);
        }
        match &evt {
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-entry" => {
                let (name, entry) = read_config_item_map_entry(ctx, xml, xml_tag)?;

                let name = if let Some(name) = name {
                    name
                } else {
                    return Err(OdsError::Ods(
                        "config-item-map-entry without name".to_string(),
                    ));
                };

                config_map.insert(name, entry);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-named" => {
                break;
            }
            Event::Eof => break,
            _ => {
                unused_event("read_config_item_map_named", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok((name, config_map))
}

// read the automatic-styles tag
fn read_config_item_map_entry(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(Option<String>, ConfigItem), OdsError> {
    let mut name = None;
    let mut config_set = ConfigItem::new_entry();

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"config:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr => {
                unused_attr(
                    "read_config_item_map_entry",
                    super_tag.name().as_ref(),
                    &attr,
                )?;
            }
        }
    }

    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        if cfg!(feature = "dump_xml") {
            println!(" read_config_item_map_entry {:?}", evt);
        }
        match &evt {
            Event::Empty(xml_tag) if xml_tag.name().as_ref() == b"config:config-item" => {}
            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item" => {
                let (name, val) = read_config_item(ctx, xml, xml_tag)?;
                config_set.insert(name, ConfigItem::from(val));
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-set" => {
                let (name, val) = read_config_item_set(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }

            Event::Start(xml_tag)
                if xml_tag.name().as_ref() == b"config:config-item-map-indexed" =>
            {
                let (name, val) = read_config_item_map_indexed(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }

            Event::Start(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-named" => {
                let (name, val) = read_config_item_map_named(ctx, xml, xml_tag)?;
                config_set.insert(name, val);
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"config:config-item-map-entry" => {
                break;
            }

            Event::Eof => break,
            _ => {
                unused_event("read_config_item_map_entry", &evt)?;
            }
        }

        buf.clear();
    }
    ctx.push_buf(buf);

    Ok((name, config_set))
}

// read the automatic-styles tag
fn read_config_item(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
) -> Result<(String, ConfigValue), OdsError> {
    #[derive(PartialEq)]
    enum ConfigValueType {
        None,
        Base64Binary,
        Boolean,
        DateTime,
        Double,
        Int,
        Long,
        Short,
        String,
    }

    let mut name = None;
    let mut val_type = ConfigValueType::None;
    let mut config_val = None;

    for attr in super_tag.attributes().with_checks(false) {
        match attr? {
            attr if attr.key.as_ref() == b"config:name" => {
                name = Some(attr.decode_and_unescape_value(xml)?.to_string());
            }
            attr if attr.key.as_ref() == b"config:type" => {
                val_type = match attr.value.as_ref() {
                    b"base64Binary" => ConfigValueType::Base64Binary,
                    b"boolean" => ConfigValueType::Boolean,
                    b"datetime" => ConfigValueType::DateTime,
                    b"double" => ConfigValueType::Double,
                    b"int" => ConfigValueType::Int,
                    b"long" => ConfigValueType::Long,
                    b"short" => ConfigValueType::Short,
                    b"string" => ConfigValueType::String,
                    x => {
                        return Err(OdsError::Ods(format!(
                            "unknown config:type {}",
                            from_utf8(x)?
                        )));
                    }
                };
            }
            attr => {
                unused_attr("read_config_item", super_tag.name().as_ref(), &attr)?;
            }
        }
    }

    let name = if let Some(name) = name {
        name
    } else {
        return Err(OdsError::Ods(
            "config value without config:name".to_string(),
        ));
    };

    if val_type == ConfigValueType::None {
        return Err(OdsError::Ods(
            "config value without config:type".to_string(),
        ));
    };

    let mut value = ctx.pop_buf();
    let mut buf = ctx.pop_buf();
    loop {
        let evt = xml.read_event_into(&mut buf)?;
        match &evt {
            Event::Text(xml_text) => {
                value.write_all(xml_text.unescape()?.as_bytes())?;
            }
            Event::End(xml_tag) if xml_tag.name().as_ref() == b"config:config-item" => {
                let value = <Cow<'_, [u8]> as From<&Vec<u8>>>::from(value.as_ref());
                match val_type {
                    ConfigValueType::None => {}
                    ConfigValueType::Base64Binary => {
                        config_val =
                            Some(ConfigValue::Base64Binary(from_utf8(&value)?.to_string()));
                    }
                    ConfigValueType::Boolean => {
                        config_val = Some(ConfigValue::Boolean(parse_bool(&value)?));
                    }
                    ConfigValueType::DateTime => {
                        config_val = Some(ConfigValue::DateTime(parse_datetime(&value)?));
                    }
                    ConfigValueType::Double => {
                        config_val = Some(ConfigValue::Double(parse_f64(&value)?));
                    }
                    ConfigValueType::Int => {
                        config_val = Some(ConfigValue::Int(parse_i32(&value)?));
                    }
                    ConfigValueType::Long => {
                        config_val = Some(ConfigValue::Long(parse_i64(&value)?));
                    }
                    ConfigValueType::Short => {
                        config_val = Some(ConfigValue::Short(parse_i16(&value)?));
                    }
                    ConfigValueType::String => {
                        config_val =
                            Some(ConfigValue::String(from_utf8(value.as_ref())?.to_string()));
                    }
                }
                break;
            }
            Event::Eof => {
                break;
            }
            _ => {
                unused_event("read_config_item", &evt)?;
            }
        }

        if cfg!(feature = "dump_xml") {
            println!(" read_config_item {:?}", evt);
        }
        buf.clear();
    }
    ctx.push_buf(buf);
    ctx.push_buf(value);

    let config_val = if let Some(config_val) = config_val {
        config_val
    } else {
        return Err(OdsError::Ods("config-item without value???".to_string()));
    };

    Ok((name, config_val))
}

// Reads a part of the XML as XmlTag's.
fn read_xml(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<XmlTag, OdsError> {
    let mut stack = ctx.pop_xml_buf();

    let mut tag = XmlTag::new(from_utf8(super_tag.name().as_ref())?);
    copy_attr2(xml, tag.attrmap_mut(), super_tag)?;
    stack.push(tag);

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_xml {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) => {
                    let mut tag = XmlTag::new(from_utf8(xml_tag.name().as_ref())?);
                    copy_attr2(xml, tag.attrmap_mut(), xml_tag)?;
                    stack.push(tag);
                }
                Event::End(xml_tag) => {
                    if xml_tag.name() == super_tag.name() {
                        break;
                    } else {
                        let tag = stack.pop().expect("valid stack");
                        if let Some(parent) = stack.last_mut() {
                            parent.add_tag(tag);
                        } else {
                            unreachable!()
                        }
                    }
                }
                Event::Empty(xml_tag) => {
                    let mut emptytag = XmlTag::new(from_utf8(xml_tag.name().as_ref())?);
                    copy_attr2(xml, emptytag.attrmap_mut(), xml_tag)?;

                    if let Some(parent) = stack.last_mut() {
                        parent.add_tag(emptytag);
                    } else {
                        unreachable!()
                    }
                }
                Event::Text(xml_text) => {
                    if let Some(parent) = stack.last_mut() {
                        parent.add_text(xml_text.unescape()?.as_ref());
                    } else {
                        unreachable!()
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_xml", &evt)?;
                }
            }
            buf.clear();
        }

        ctx.push_buf(buf);
    }

    let tag = stack.pop().unwrap();
    ctx.push_xml_buf(stack);
    Ok(tag)
}

fn read_text_or_tag(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
) -> Result<TextContent, OdsError> {
    let mut stack = ctx.pop_xml_buf();
    let mut cellcontent = TextContent::Empty;

    // The toplevel element is passed in with the xml_tag.
    // It is only created if there are further xml tags in the
    // element. If there is only text this is not needed.
    let create_toplevel =
        |xml: &mut OdsXmlReader<'_>, t: Option<String>| -> Result<XmlTag, OdsError> {
            // No parent tag on the stack. Create the parent.
            let mut toplevel = XmlTag::new(from_utf8(super_tag.name().as_ref())?);
            copy_attr2(xml, toplevel.attrmap_mut(), super_tag)?;
            if let Some(t) = t {
                toplevel.add_text(t);
            }
            Ok(toplevel)
        };

    if !empty_tag {
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_xml {:?}", evt);
            }
            match &evt {
                Event::Start(xml_tag) => {
                    match cellcontent {
                        TextContent::Empty => {
                            stack.push(create_toplevel(xml, None)?);
                        }
                        TextContent::Text(old_txt) => {
                            stack.push(create_toplevel(xml, Some(old_txt))?);
                        }
                        TextContent::Xml(parent) => {
                            stack.push(parent);
                        }
                        TextContent::XmlVec(_) => {
                            unreachable!()
                        }
                    }

                    // Set the new tag.
                    let mut new_tag = XmlTag::new(from_utf8(xml_tag.name().as_ref())?);
                    copy_attr2(xml, new_tag.attrmap_mut(), xml_tag)?;
                    cellcontent = TextContent::Xml(new_tag)
                }
                Event::Empty(xml_tag) => {
                    match cellcontent {
                        TextContent::Empty => {
                            stack.push(create_toplevel(xml, None)?);
                        }
                        TextContent::Text(txt) => {
                            stack.push(create_toplevel(xml, Some(txt))?);
                        }
                        TextContent::Xml(parent) => {
                            stack.push(parent);
                        }
                        TextContent::XmlVec(_) => {
                            unreachable!()
                        }
                    }
                    if let Some(mut parent) = stack.pop() {
                        // Create the tag and append it immediately to the parent.
                        let mut emptytag = XmlTag::new(from_utf8(xml_tag.name().as_ref())?);
                        copy_attr2(xml, emptytag.attrmap_mut(), xml_tag)?;
                        parent.add_tag(emptytag);

                        cellcontent = TextContent::Xml(parent);
                    } else {
                        unreachable!()
                    }
                }
                Event::Text(xml_text) => {
                    let v = xml_text.unescape()?;

                    cellcontent = match cellcontent {
                        TextContent::Empty => {
                            // Fresh plain text string.
                            TextContent::Text(v.to_string())
                        }
                        TextContent::Text(mut old_txt) => {
                            // We have a previous plain text string. Append to it.
                            old_txt.push_str(&v);
                            TextContent::Text(old_txt)
                        }
                        TextContent::Xml(mut xml) => {
                            // There is already a tag. Append the text to its children.
                            xml.add_text(v);
                            TextContent::Xml(xml)
                        }
                        TextContent::XmlVec(_) => {
                            unreachable!()
                        }
                    };
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    if !stack.is_empty() {
                        return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(format!(
                            "XML corrupted. Endtag {} occured before all elements are closed: {:?}",
                            from_utf8(super_tag.name().as_ref())?,
                            stack
                        ))));
                    }
                    break;
                }
                Event::End(xml_tag) => {
                    cellcontent = match cellcontent {
                        TextContent::Empty | TextContent::Text(_) => {
                            return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(format!(
                                "XML corrupted. Endtag {} occured without start tag",
                                from_utf8(xml_tag.name().as_ref())?
                            ))));
                        }
                        TextContent::Xml(tag) => {
                            if let Some(mut parent) = stack.pop() {
                                parent.add_tag(tag);
                                TextContent::Xml(parent)
                            } else {
                                return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(
                                    format!(
                                        "XML corrupted. Endtag {} occured without start tag",
                                        from_utf8(xml_tag.name().as_ref())?
                                    ),
                                )));
                            }
                        }
                        TextContent::XmlVec(_) => {
                            unreachable!()
                        }
                    }
                }

                Event::Eof => {
                    break;
                }

                _ => {
                    unused_event("read_text_or_tag", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);
    }

    ctx.push_xml_buf(stack);

    Ok(cellcontent)
}

/// Read simple text content.
/// Fail on any tag other than the end-tag to the supertag.
fn read_text<T, E>(
    ctx: &mut OdsContext,
    xml: &mut OdsXmlReader<'_>,
    super_tag: &BytesStart<'_>,
    empty_tag: bool,
    parse: fn(&[u8]) -> Result<T, E>,
) -> Result<Option<T>, OdsError>
where
    OdsError: From<E>,
{
    if empty_tag {
        Ok(None)
    } else {
        let mut result_buf = ctx.pop_buf();
        let mut buf = ctx.pop_buf();
        loop {
            let evt = xml.read_event_into(&mut buf)?;
            if cfg!(feature = "dump_xml") {
                println!(" read_text {:?}", evt);
            }
            match &evt {
                Event::Text(xml_text) => {
                    result_buf.extend_from_slice(xml_text.as_ref());
                }
                Event::End(xml_tag) if xml_tag.name() == super_tag.name() => {
                    break;
                }
                Event::Empty(xml_tag) | Event::Start(xml_tag) => {
                    return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(
                        from_utf8(xml_tag.as_ref())?.to_string(),
                    )));
                }
                Event::End(xml_tag) => {
                    return Err(OdsError::Xml(quick_xml::Error::UnexpectedToken(
                        from_utf8(xml_tag.as_ref())?.to_string(),
                    )));
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    unused_event("read_text", &evt)?;
                }
            }
        }
        ctx.push_buf(buf);

        let result = parse(&result_buf)?;
        ctx.push_buf(result_buf);

        Ok(Some(result))
    }
}

#[inline(always)]
fn unused_attr(func: &str, tag: &[u8], attr: &Attribute<'_>) -> Result<(), OdsError> {
    if cfg!(feature = "dump_unused") {
        let tag = from_utf8(tag)?;
        let key = from_utf8(attr.key.as_ref())?;
        let value = from_utf8(attr.value.as_ref())?;
        println!("unused attr: {} '{}' ({}:{})", func, tag, key, value);
    }
    Ok(())
}

#[inline(always)]
fn unused_event(func: &str, evt: &Event<'_>) -> Result<(), OdsError> {
    if cfg!(feature = "dump_unused") {
        match &evt {
            Event::Text(xml_text) => {
                if !xml_text.unescape()?.trim().is_empty() {
                    println!("unused text: {} ({:?})", func, evt);
                }
            }
            _ => {
                println!("unused event: {} ({:?})", func, evt);
            }
        }
    }
    Ok(())
}

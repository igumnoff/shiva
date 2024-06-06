use crate::cell_::CellData;
use crate::config::{ConfigItem, ConfigItemType, ConfigValue};
use crate::draw::{Annotation, DrawFrame, DrawFrameContent, DrawImage};
use crate::error::OdsError;
use crate::format::{FormatPartType, ValueFormatTrait};
use crate::io::format::{format_duration2, format_validation_condition};
use crate::io::xmlwriter::XmlWriter;
use crate::io::NamespaceMap;
use crate::manifest::Manifest;
use crate::metadata::MetaValue;
use crate::refs::{format_cellranges, CellRange};
use crate::sheet::Visibility;
use crate::sheet_::{dedup_colheader, CellDataIter};
use crate::style::{
    CellStyle, ColStyle, FontFaceDecl, GraphicStyle, HeaderFooter, MasterPage, MasterPageRef,
    PageStyle, PageStyleRef, ParagraphStyle, RowStyle, RubyStyle, StyleOrigin, StyleUse,
    TableStyle, TextStyle,
};
use crate::validation::ValidationDisplay;
use crate::workbook::{EventListener, Script};
use crate::xmltree::{XmlContent, XmlTag};
use crate::HashMap;
use crate::{Length, Sheet, Value, ValueType, WorkBook};
use std::borrow::Cow;
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Cursor, Seek, Write};
use std::path::Path;
use std::{io, mem};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

#[cfg(test)]
mod tests;

type OdsXmlWriter<'a> = XmlWriter<&'a mut dyn Write>;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

#[allow(dead_code)]
trait SeekWrite: Seek + Write {}

impl<T> SeekWrite for T where T: Seek + Write {}

struct OdsContext<W: Seek + Write> {
    compression: CompressionMethod,
    zip_writer: ZipWriter<W>,
}

struct FodsContext<'a> {
    writer: &'a mut dyn Write,
}

/// Writes the ODS file into a supplied buffer.
pub fn write_ods_buf_uncompressed(book: &mut WorkBook, buf: Vec<u8>) -> Result<Vec<u8>, OdsError> {
    let mut cursor = Cursor::new(buf);

    let ctx = OdsContext {
        compression: CompressionMethod::Stored,
        zip_writer: ZipWriter::new(&mut cursor),
    };

    write_ods_impl(ctx, book)?;

    Ok(cursor.into_inner())
}

/// Writes the ODS file into a supplied buffer.
pub fn write_ods_buf(book: &mut WorkBook, buf: Vec<u8>) -> Result<Vec<u8>, OdsError> {
    let mut cursor = Cursor::new(buf);

    let ctx = OdsContext {
        compression: CompressionMethod::Deflated,
        zip_writer: ZipWriter::new(&mut cursor),
    };

    write_ods_impl(ctx, book)?;

    Ok(cursor.into_inner())
}

/// Writes the ODS file to the given Write.
pub fn write_ods_to<T: Write + Seek>(book: &mut WorkBook, mut write: T) -> Result<(), OdsError> {
    let ctx = OdsContext {
        compression: CompressionMethod::Deflated,
        zip_writer: ZipWriter::new(&mut write),
    };

    write_ods_impl(ctx, book)?;

    Ok(())
}

/// Writes the ODS file.
pub fn write_ods<P: AsRef<Path>>(book: &mut WorkBook, ods_path: P) -> Result<(), OdsError> {
    let mut write = BufWriter::new(File::create(ods_path)?);

    let ctx = OdsContext {
        compression: CompressionMethod::Deflated,
        zip_writer: ZipWriter::new(&mut write),
    };

    write_ods_impl(ctx, book)?;

    write.flush()?;

    Ok(())
}

/// Writes the FODS file into a supplied buffer.
pub fn write_fods_buf(book: &mut WorkBook, mut buf: Vec<u8>) -> Result<Vec<u8>, OdsError> {
    let ctx = FodsContext { writer: &mut buf };

    write_fods_impl(ctx, book)?;

    Ok(buf)
}

/// Writes the FODS file to the given Write.
pub fn write_fods_to<T: Write + Seek>(book: &mut WorkBook, mut write: T) -> Result<(), OdsError> {
    let ctx = FodsContext { writer: &mut write };

    write_fods_impl(ctx, book)?;

    Ok(())
}

/// Writes the FODS file.
pub fn write_fods<P: AsRef<Path>>(book: &mut WorkBook, fods_path: P) -> Result<(), OdsError> {
    let mut write = BufWriter::new(File::create(fods_path)?);

    let ctx = FodsContext { writer: &mut write };

    write_fods_impl(ctx, book)?;

    Ok(())
}

/// Writes the ODS file.
///
fn write_fods_impl(ctx: FodsContext<'_>, book: &mut WorkBook) -> Result<(), OdsError> {
    sanity_checks(book)?;
    calculations(book)?;

    convert(book)?;

    let mut xml_out = XmlWriter::new(ctx.writer).line_break(true);
    write_fods_content(book, &mut xml_out)?;

    Ok(())
}

fn convert(book: &mut WorkBook) -> Result<(), OdsError> {
    for v in book.tablestyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.rowstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.colstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.cellstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.paragraphstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.textstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.rubystyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.graphicstyles.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }

    for v in book.formats_boolean.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_number.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_percentage.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_currency.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_text.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_datetime.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }
    for v in book.formats_timeduration.values_mut() {
        v.set_origin(StyleOrigin::Content);
    }

    Ok(())
}

fn write_fods_content(book: &mut WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let xmlns = book
        .xmlns
        .entry("meta.xml".into())
        .or_insert_with(NamespaceMap::new);

    xmlns.insert_str(
        "xmlns:office",
        "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    );
    xmlns.insert_str("xmlns:ooo", "http://openoffice.org/2004/office");
    xmlns.insert_str(
        "xmlns:fo",
        "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0",
    );
    xmlns.insert_str("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xmlns.insert_str(
        "xmlns:config",
        "urn:oasis:names:tc:opendocument:xmlns:config:1.0",
    );
    xmlns.insert_str("xmlns:dc", "http://purl.org/dc/elements/1.1/");
    xmlns.insert_str(
        "xmlns:meta",
        "urn:oasis:names:tc:opendocument:xmlns:meta:1.0",
    );
    xmlns.insert_str(
        "xmlns:style",
        "urn:oasis:names:tc:opendocument:xmlns:style:1.0",
    );
    xmlns.insert_str(
        "xmlns:text",
        "urn:oasis:names:tc:opendocument:xmlns:text:1.0",
    );
    xmlns.insert_str("xmlns:rpt", "http://openoffice.org/2005/report");
    xmlns.insert_str(
        "xmlns:draw",
        "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0",
    );
    xmlns.insert_str(
        "xmlns:dr3d",
        "urn:oasis:names:tc:opendocument:xmlns:dr3d:1.0",
    );
    xmlns.insert_str(
        "xmlns:svg",
        "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0",
    );
    xmlns.insert_str(
        "xmlns:chart",
        "urn:oasis:names:tc:opendocument:xmlns:chart:1.0",
    );
    xmlns.insert_str(
        "xmlns:table",
        "urn:oasis:names:tc:opendocument:xmlns:table:1.0",
    );
    xmlns.insert_str(
        "xmlns:number",
        "urn:oasis:names:tc:opendocument:xmlns:datastyle:1.0",
    );
    xmlns.insert_str("xmlns:ooow", "http://openoffice.org/2004/writer");
    xmlns.insert_str("xmlns:oooc", "http://openoffice.org/2004/calc");
    xmlns.insert_str("xmlns:of", "urn:oasis:names:tc:opendocument:xmlns:of:1.2");
    xmlns.insert_str("xmlns:xforms", "http://www.w3.org/2002/xforms");
    xmlns.insert_str("xmlns:tableooo", "http://openoffice.org/2009/table");
    xmlns.insert_str(
        "xmlns:calcext",
        "urn:org:documentfoundation:names:experimental:calc:xmlns:calcext:1.0",
    );
    xmlns.insert_str("xmlns:drawooo", "http://openoffice.org/2010/draw");
    xmlns.insert_str(
        "xmlns:loext",
        "urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0",
    );
    xmlns.insert_str(
        "xmlns:field",
        "urn:openoffice:names:experimental:ooo-ms-interop:xmlns:field:1.0",
    );
    xmlns.insert_str("xmlns:math", "http://www.w3.org/1998/Math/MathML");
    xmlns.insert_str(
        "xmlns:form",
        "urn:oasis:names:tc:opendocument:xmlns:form:1.0",
    );
    xmlns.insert_str(
        "xmlns:script",
        "urn:oasis:names:tc:opendocument:xmlns:script:1.0",
    );
    xmlns.insert_str(
        "xmlns:formx",
        "urn:openoffice:names:experimental:ooxml-odf-interop:xmlns:form:1.0",
    );
    xmlns.insert_str("xmlns:dom", "http://www.w3.org/2001/xml-events");
    xmlns.insert_str("xmlns:xsd", "http://www.w3.org/2001/XMLSchema");
    xmlns.insert_str("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
    xmlns.insert_str("xmlns:xhtml", "http://www.w3.org/1999/xhtml");
    xmlns.insert_str("xmlns:grddl", "http://www.w3.org/2003/g/data-view#");
    xmlns.insert_str("xmlns:css3t", "http://www.w3.org/TR/css3-text/");
    xmlns.insert_str(
        "xmlns:presentation",
        "urn:oasis:names:tc:opendocument:xmlns:presentation:1.0",
    );

    xml_out.dtd("UTF-8")?;

    xml_out.elem("office:document")?;
    write_xmlns(xmlns, xml_out)?;
    xml_out.attr_esc("office:version", book.version())?;
    xml_out.attr_esc(
        "office:mimetype",
        "application/vnd.oasis.opendocument.spreadsheet",
    )?;

    write_office_meta(book, xml_out)?;
    write_office_settings(book, xml_out)?;
    write_office_scripts(book, xml_out)?;
    write_office_font_face_decls(book, StyleOrigin::Content, xml_out)?;
    write_office_styles(book, StyleOrigin::Content, xml_out)?;
    write_office_automatic_styles(book, StyleOrigin::Content, xml_out)?;
    write_office_master_styles(book, xml_out)?;
    write_office_body(book, xml_out)?;

    xml_out.end_elem("office:document")?;

    xml_out.close()?;

    Ok(())
}

/// Writes the ODS file.
///
fn write_ods_impl<W: Write + Seek>(
    mut ctx: OdsContext<W>,
    book: &mut WorkBook,
) -> Result<(), OdsError> {
    sanity_checks(book)?;
    calculations(book)?;

    create_manifest(book)?;

    ctx.zip_writer.start_file(
        "mimetype",
        FileOptions::<()>::default().compression_method(CompressionMethod::Stored),
    )?;
    write_ods_mimetype(&mut ctx.zip_writer)?;

    ctx.zip_writer
        .add_directory("META-INF", FileOptions::<()>::default())?;
    ctx.zip_writer.start_file(
        "META-INF/manifest.xml",
        FileOptions::<()>::default().compression_method(ctx.compression),
    )?;
    write_ods_manifest(book, &mut XmlWriter::new(&mut ctx.zip_writer))?;

    ctx.zip_writer.start_file(
        "meta.xml",
        FileOptions::<()>::default().compression_method(ctx.compression),
    )?;
    write_ods_metadata(book, &mut XmlWriter::new(&mut ctx.zip_writer))?;

    ctx.zip_writer.start_file(
        "settings.xml",
        FileOptions::<()>::default().compression_method(ctx.compression),
    )?;
    write_ods_settings(book, &mut XmlWriter::new(&mut ctx.zip_writer))?;

    ctx.zip_writer.start_file(
        "styles.xml",
        FileOptions::<()>::default().compression_method(ctx.compression),
    )?;
    write_ods_styles(book, &mut XmlWriter::new(&mut ctx.zip_writer))?;

    ctx.zip_writer.start_file(
        "content.xml",
        FileOptions::<()>::default().compression_method(ctx.compression),
    )?;
    write_ods_content(book, &mut XmlWriter::new(&mut ctx.zip_writer))?;

    write_ods_extra(&mut ctx, book)?;

    ctx.zip_writer.finish()?;

    Ok(())
}

/// Sanity checks.
fn sanity_checks(book: &mut WorkBook) -> Result<(), OdsError> {
    if book.sheets.is_empty() {
        return Err(OdsError::Ods("Workbook contains no sheets.".to_string()));
    }
    Ok(())
}

/// Before write calculations.
fn calculations(book: &mut WorkBook) -> Result<(), OdsError> {
    calc_metadata(book)?;
    calc_config(book)?;

    calc_row_header_styles(book)?;
    calc_col_header_styles(book)?;
    calc_col_headers(book)?;

    Ok(())
}

/// Compacting and normalizing column-headers.
fn calc_col_headers(book: &mut WorkBook) -> Result<(), OdsError> {
    for i in 0..book.num_sheets() {
        let mut sheet = book.detach_sheet(i);

        // deduplicate all col-headers
        dedup_colheader(&mut sheet)?;

        // resplit along column-groups and header-columns.
        let mut split_pos = HashSet::new();
        for grp in &sheet.group_cols {
            split_pos.insert(grp.from);
            split_pos.insert(grp.to + 1);
        }
        if let Some(header_cols) = &sheet.header_cols {
            split_pos.insert(header_cols.from);
            split_pos.insert(header_cols.to + 1);
        }

        let col_header = mem::take(&mut sheet.col_header);
        let mut new_col_header = BTreeMap::new();

        for (mut col, mut header) in col_header {
            let mut cc = col;
            loop {
                if cc == col + header.span {
                    new_col_header.insert(col, header);
                    break;
                }

                if split_pos.contains(&cc) {
                    let new_span = cc - col;
                    if new_span > 0 {
                        let mut new_header = header.clone();
                        new_header.span = new_span;
                        new_col_header.insert(col, new_header);
                    }

                    header.span -= new_span;
                    col = cc;
                }

                cc += 1;
            }
        }

        sheet.col_header = new_col_header;

        book.attach_sheet(sheet);
    }

    Ok(())
}

/// Sync row/column styles with row/col header values.
fn calc_col_header_styles(book: &mut WorkBook) -> Result<(), OdsError> {
    for i in 0..book.num_sheets() {
        let mut sheet = book.detach_sheet(i);

        // Set the column widths.
        for ch in sheet.col_header.values_mut() {
            // Any non default values?
            if ch.width != Length::Default && ch.style.is_none() {
                let colstyle = book.add_colstyle(ColStyle::new_empty());
                ch.style = Some(colstyle);
            }

            // Write back to the style.
            if let Some(style_name) = ch.style.as_ref() {
                if let Some(style) = book.colstyle_mut(style_name) {
                    if ch.width == Length::Default {
                        style.set_use_optimal_col_width(true);
                        style.set_col_width(Length::Default);
                    } else {
                        style.set_col_width(ch.width);
                    }
                }
            }
        }

        book.attach_sheet(sheet);
    }

    Ok(())
}

/// Sync row/column styles with row/col header values.
fn calc_row_header_styles(book: &mut WorkBook) -> Result<(), OdsError> {
    for i in 0..book.num_sheets() {
        let mut sheet = book.detach_sheet(i);

        for rh in sheet.row_header.values_mut() {
            if rh.height != Length::Default && rh.style.is_none() {
                let rowstyle = book.add_rowstyle(RowStyle::new_empty());
                rh.style = Some(rowstyle);
            }

            if let Some(style_name) = rh.style.as_ref() {
                if let Some(style) = book.rowstyle_mut(style_name) {
                    if rh.height == Length::Default {
                        style.set_use_optimal_row_height(true);
                        style.set_row_height(Length::Default);
                    } else {
                        style.set_row_height(rh.height);
                    }
                }
            }
        }

        book.attach_sheet(sheet);
    }

    Ok(())
}

/// Calculate metadata values.
fn calc_metadata(book: &mut WorkBook) -> Result<(), OdsError> {
    // Manifest
    book.metadata.generator = format!("spreadsheet-ods {}", env!("CARGO_PKG_VERSION"));
    book.metadata.document_statistics.table_count = book.sheets.len() as u32;
    let mut cell_count = 0;
    for sheet in book.iter_sheets() {
        cell_count += sheet.data.len() as u32;
    }
    book.metadata.document_statistics.cell_count = cell_count;

    Ok(())
}

/// - Syncs book.config back to the config tree structure.
/// - Syncs row-heights and col-widths back to the corresponding styles.
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::collapsible_if)]
fn calc_config(book: &mut WorkBook) -> Result<(), OdsError> {
    // Config
    let mut config = book.config.detach(0);

    let bc = config.create_path(&[
        ("ooo:view-settings", ConfigItemType::Set),
        ("Views", ConfigItemType::Vec),
        ("0", ConfigItemType::Entry),
    ]);
    if book.config().active_table.is_empty() {
        book.config_mut().active_table = book.sheet(0).name().clone();
    }
    bc.insert("ActiveTable", book.config().active_table.clone());
    bc.insert("HasSheetTabs", book.config().has_sheet_tabs);
    bc.insert("ShowGrid", book.config().show_grid);
    bc.insert("ShowPageBreaks", book.config().show_page_breaks);

    for i in 0..book.num_sheets() {
        let sheet = book.detach_sheet(i);

        let bc = config.create_path(&[
            ("ooo:view-settings", ConfigItemType::Set),
            ("Views", ConfigItemType::Vec),
            ("0", ConfigItemType::Entry),
            ("Tables", ConfigItemType::Map),
            (sheet.name().as_str(), ConfigItemType::Entry),
        ]);

        bc.insert("CursorPositionX", sheet.config().cursor_x);
        bc.insert("CursorPositionY", sheet.config().cursor_y);
        bc.insert("HorizontalSplitMode", sheet.config().hor_split_mode as i16);
        bc.insert("VerticalSplitMode", sheet.config().vert_split_mode as i16);
        bc.insert("HorizontalSplitPosition", sheet.config().hor_split_pos);
        bc.insert("VerticalSplitPosition", sheet.config().vert_split_pos);
        bc.insert("ActiveSplitRange", sheet.config().active_split_range);
        bc.insert("PositionLeft", sheet.config().position_left);
        bc.insert("PositionRight", sheet.config().position_right);
        bc.insert("PositionTop", sheet.config().position_top);
        bc.insert("PositionBottom", sheet.config().position_bottom);
        bc.insert("ZoomType", sheet.config().zoom_type);
        bc.insert("ZoomValue", sheet.config().zoom_value);
        bc.insert("PageViewZoomValue", sheet.config().page_view_zoom_value);
        bc.insert("ShowGrid", sheet.config().show_grid);

        let bc = config.create_path(&[
            ("ooo:configuration-settings", ConfigItemType::Set),
            ("ScriptConfiguration", ConfigItemType::Map),
            (sheet.name().as_str(), ConfigItemType::Entry),
        ]);
        // maybe this is not accurate. there seem to be cases where the codename
        // is not the same as the table name, but I can't find any uses of the
        // codename anywhere.
        bc.insert("CodeName", sheet.name().as_str().to_string());

        book.attach_sheet(sheet);
    }

    book.config.attach(config);

    Ok(())
}

// Create the standard manifest entries.
fn create_manifest(book: &mut WorkBook) -> Result<(), OdsError> {
    if !book.manifest.contains_key("/") {
        book.add_manifest(Manifest {
            full_path: "/".to_string(),
            version: Some(book.version().clone()),
            media_type: "application/vnd.oasis.opendocument.spreadsheet".to_string(),
            buffer: None,
        });
    }
    if !book.manifest.contains_key("manifest.rdf") {
        book.add_manifest(create_manifest_rdf()?);
    }
    if !book.manifest.contains_key("styles.xml") {
        book.add_manifest(Manifest::new("styles.xml", "text/xml"));
    }
    if !book.manifest.contains_key("meta.xml") {
        book.add_manifest(Manifest::new("meta.xml", "text/xml"));
    }
    if !book.manifest.contains_key("content.xml") {
        book.add_manifest(Manifest::new("content.xml", "text/xml"));
    }
    if !book.manifest.contains_key("settings.xml") {
        book.add_manifest(Manifest::new("settings.xml", "text/xml"));
    }

    Ok(())
}

fn create_manifest_rdf() -> Result<Manifest, OdsError> {
    let mut buf = Vec::new();
    let mut xml_out = XmlWriter::new(&mut buf);

    xml_out.dtd("UTF-8")?;
    xml_out.elem("rdf:RDF")?;
    xml_out.attr_str("xmlns:rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;
    xml_out.elem("rdf:Description")?;
    xml_out.attr_str("rdf:about", "content.xml")?;
    xml_out.empty("rdf:type")?;
    xml_out.attr_str(
        "rdf:resource",
        "http://docs.oasis-open.org/ns/office/1.2/meta/odf#ContentFile",
    )?;
    xml_out.end_elem("rdf:Description")?;
    xml_out.elem("rdf:Description")?;
    xml_out.attr_str("rdf:about", "")?;
    xml_out.empty("ns0:hasPart")?;
    xml_out.attr_str(
        "xmlns:ns0",
        "http://docs.oasis-open.org/ns/office/1.2/meta/pkg#",
    )?;
    xml_out.attr_str("rdf:resource", "content.xml")?;
    xml_out.end_elem("rdf:Description")?;
    xml_out.elem("rdf:Description")?;
    xml_out.attr_str("rdf:about", "")?;
    xml_out.empty("rdf:type")?;
    xml_out.attr_str(
        "rdf:resource",
        "http://docs.oasis-open.org/ns/office/1.2/meta/pkg#Document",
    )?;
    xml_out.end_elem("rdf:Description")?;
    xml_out.end_elem("rdf:RDF")?;
    xml_out.close()?;

    Ok(Manifest::with_buf(
        "manifest.rdf",
        "application/rdf+xml",
        buf,
    ))
}

fn write_ods_mimetype(write: &'_ mut dyn Write) -> Result<(), io::Error> {
    write.write_all("application/vnd.oasis.opendocument.spreadsheet".as_bytes())?;
    Ok(())
}

fn write_ods_manifest(book: &WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.dtd("UTF-8")?;

    xml_out.elem("manifest:manifest")?;
    xml_out.attr_str(
        "xmlns:manifest",
        "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0",
    )?;
    xml_out.attr_esc("manifest:version", &book.version())?;

    for manifest in book.manifest.values() {
        xml_out.empty("manifest:file-entry")?;
        xml_out.attr_esc("manifest:full-path", &manifest.full_path)?;
        if let Some(version) = &manifest.version {
            xml_out.attr_esc("manifest:version", version)?;
        }
        xml_out.attr_esc("manifest:media-type", &manifest.media_type)?;
    }

    xml_out.end_elem("manifest:manifest")?;

    xml_out.close()?;

    Ok(())
}

fn write_xmlns(xmlns: &NamespaceMap, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    for (k, v) in xmlns.entries() {
        match k {
            Cow::Borrowed(k) => {
                xml_out.attr(k, v.as_ref())?;
            }
            Cow::Owned(k) => {
                xml_out.attr_esc(k, v.as_ref())?;
            }
        }
    }
    Ok(())
}

fn write_ods_metadata(book: &mut WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let xmlns = book
        .xmlns
        .entry("meta.xml".into())
        .or_insert_with(NamespaceMap::new);

    xmlns.insert_str(
        "xmlns:meta",
        "urn:oasis:names:tc:opendocument:xmlns:meta:1.0",
    );
    xmlns.insert_str(
        "xmlns:office",
        "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    );

    xml_out.dtd("UTF-8")?;

    xml_out.elem("office:document-meta")?;
    write_xmlns(xmlns, xml_out)?;
    xml_out.attr_esc("office:version", book.version())?;

    write_office_meta(book, xml_out)?;

    xml_out.end_elem("office:document-meta")?;

    xml_out.close()?;

    Ok(())
}

fn write_office_meta(book: &WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.elem("office:meta")?;

    xml_out.elem_text("meta:generator", &book.metadata.generator)?;
    if !book.metadata.title.is_empty() {
        xml_out.elem_text_esc("dc:title", &book.metadata.title)?;
    }
    if !book.metadata.description.is_empty() {
        xml_out.elem_text_esc("dc:description", &book.metadata.description)?;
    }
    if !book.metadata.subject.is_empty() {
        xml_out.elem_text_esc("dc:subject", &book.metadata.subject)?;
    }
    if !book.metadata.keyword.is_empty() {
        xml_out.elem_text_esc("meta:keyword", &book.metadata.keyword)?;
    }
    if !book.metadata.initial_creator.is_empty() {
        xml_out.elem_text_esc("meta:initial-creator", &book.metadata.initial_creator)?;
    }
    if !book.metadata.creator.is_empty() {
        xml_out.elem_text_esc("dc:creator", &book.metadata.creator)?;
    }
    if !book.metadata.printed_by.is_empty() {
        xml_out.elem_text_esc("meta:printed-by", &book.metadata.printed_by)?;
    }
    if let Some(v) = book.metadata.creation_date {
        xml_out.elem_text("meta:creation-date", &v.format(DATETIME_FORMAT))?;
    }
    if let Some(v) = book.metadata.date {
        xml_out.elem_text("dc:date", &v.format(DATETIME_FORMAT))?;
    }
    if let Some(v) = book.metadata.print_date {
        xml_out.elem_text("meta:print-date", &v.format(DATETIME_FORMAT))?;
    }
    if !book.metadata.language.is_empty() {
        xml_out.elem_text_esc("dc:language", &book.metadata.language)?;
    }
    if book.metadata.editing_cycles > 0 {
        xml_out.elem_text("meta:editing-cycles", &book.metadata.editing_cycles)?;
    }
    if book.metadata.editing_duration.num_seconds() > 0 {
        xml_out.elem_text(
            "meta:editing-duration",
            &format_duration2(book.metadata.editing_duration),
        )?;
    }

    if !book.metadata.template.is_empty() {
        xml_out.empty("meta:template")?;
        if let Some(v) = book.metadata.template.date {
            xml_out.attr("meta:date", &v.format(DATETIME_FORMAT))?;
        }
        if let Some(v) = book.metadata.template.actuate {
            xml_out.attr("xlink:actuate", &v)?;
        }
        if let Some(v) = &book.metadata.template.href {
            xml_out.attr_esc("xlink:href", v)?;
        }
        if let Some(v) = &book.metadata.template.title {
            xml_out.attr_esc("xlink:title", v)?;
        }
        if let Some(v) = book.metadata.template.link_type {
            xml_out.attr("xlink:type", &v)?;
        }
    }

    if !book.metadata.auto_reload.is_empty() {
        xml_out.empty("meta:auto_reload")?;
        if let Some(v) = book.metadata.auto_reload.delay {
            xml_out.attr("meta:delay", &format_duration2(v))?;
        }
        if let Some(v) = book.metadata.auto_reload.actuate {
            xml_out.attr("xlink:actuate", &v)?;
        }
        if let Some(v) = &book.metadata.auto_reload.href {
            xml_out.attr_esc("xlink:href", v)?;
        }
        if let Some(v) = &book.metadata.auto_reload.show {
            xml_out.attr("xlink:show", v)?;
        }
        if let Some(v) = book.metadata.auto_reload.link_type {
            xml_out.attr("xlink:type", &v)?;
        }
    }

    if !book.metadata.hyperlink_behaviour.is_empty() {
        xml_out.empty("meta:hyperlink-behaviour")?;
        if let Some(v) = &book.metadata.hyperlink_behaviour.target_frame_name {
            xml_out.attr_esc("office:target-frame-name", v)?;
        }
        if let Some(v) = &book.metadata.hyperlink_behaviour.show {
            xml_out.attr("xlink:show", v)?;
        }
    }

    xml_out.empty("meta:document-statistic")?;
    xml_out.attr(
        "meta:table-count",
        &book.metadata.document_statistics.table_count,
    )?;
    xml_out.attr(
        "meta:cell-count",
        &book.metadata.document_statistics.cell_count,
    )?;
    xml_out.attr(
        "meta:object-count",
        &book.metadata.document_statistics.object_count,
    )?;
    xml_out.attr(
        "meta:ole-object-count",
        &book.metadata.document_statistics.ole_object_count,
    )?;

    for userdef in &book.metadata.user_defined {
        xml_out.elem("meta:user-defined")?;
        xml_out.attr("meta:name", &userdef.name)?;
        if !matches!(userdef.value, MetaValue::String(_)) {
            xml_out.attr_str(
                "meta:value-type",
                match &userdef.value {
                    MetaValue::Boolean(_) => "boolean",
                    MetaValue::Datetime(_) => "date",
                    MetaValue::Float(_) => "float",
                    MetaValue::TimeDuration(_) => "time",
                    MetaValue::String(_) => unreachable!(),
                },
            )?;
        }
        match &userdef.value {
            MetaValue::Boolean(v) => xml_out.text_str(if *v { "true" } else { "false" })?,
            MetaValue::Datetime(v) => xml_out.text(&v.format(DATETIME_FORMAT))?,
            MetaValue::Float(v) => xml_out.text(&v)?,
            MetaValue::TimeDuration(v) => xml_out.text(&format_duration2(*v))?,
            MetaValue::String(v) => xml_out.text_esc(v)?,
        }
        xml_out.end_elem("meta:user-defined")?;
    }

    xml_out.end_elem("office:meta")?;
    Ok(())
}

fn write_ods_settings(book: &mut WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let xmlns = book
        .xmlns
        .entry("settings.xml".into())
        .or_insert_with(NamespaceMap::new);

    xmlns.insert_str(
        "xmlns:office",
        "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    );
    xmlns.insert_str("xmlns:ooo", "http://openoffice.org/2004/office");
    xmlns.insert_str(
        "xmlns:config",
        "urn:oasis:names:tc:opendocument:xmlns:config:1.0",
    );

    xml_out.dtd("UTF-8")?;

    xml_out.elem("office:document-settings")?;
    write_xmlns(xmlns, xml_out)?;
    xml_out.attr_esc("office:version", book.version())?;

    write_office_settings(book, xml_out)?;

    xml_out.end_elem("office:document-settings")?;

    xml_out.close()?;

    Ok(())
}

fn write_office_settings(book: &WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.elem("office:settings")?;

    for (name, item) in book.config.iter() {
        match item {
            ConfigItem::Value(_) => {
                panic!("office-settings must not contain config-item");
            }
            ConfigItem::Set(_) => write_config_item_set(name, item, xml_out)?,
            ConfigItem::Vec(_) => {
                panic!("office-settings must not contain config-item-map-index")
            }
            ConfigItem::Map(_) => {
                panic!("office-settings must not contain config-item-map-named")
            }
            ConfigItem::Entry(_) => {
                panic!("office-settings must not contain config-item-map-entry")
            }
        }
    }

    xml_out.end_elem("office:settings")?;
    Ok(())
}

fn write_config_item_set(
    name: &str,
    set: &ConfigItem,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("config:config-item-set")?;
    xml_out.attr_esc("config:name", name)?;

    for (name, item) in set.iter() {
        match item {
            ConfigItem::Value(value) => write_config_item(name, value, xml_out)?,
            ConfigItem::Set(_) => write_config_item_set(name, item, xml_out)?,
            ConfigItem::Vec(_) => write_config_item_map_indexed(name, item, xml_out)?,
            ConfigItem::Map(_) => write_config_item_map_named(name, item, xml_out)?,
            ConfigItem::Entry(_) => {
                panic!("config-item-set must not contain config-item-map-entry")
            }
        }
    }

    xml_out.end_elem("config:config-item-set")?;

    Ok(())
}

fn write_config_item_map_indexed(
    name: &str,
    vec: &ConfigItem,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("config:config-item-map-indexed")?;
    xml_out.attr_esc("config:name", name)?;

    let mut index = 0;
    loop {
        let index_str = index.to_string();
        if let Some(item) = vec.get(&index_str) {
            match item {
                ConfigItem::Value(value) => write_config_item(name, value, xml_out)?,
                ConfigItem::Set(_) => {
                    panic!("config-item-map-index must not contain config-item-set")
                }
                ConfigItem::Vec(_) => {
                    panic!("config-item-map-index must not contain config-item-map-index")
                }
                ConfigItem::Map(_) => {
                    panic!("config-item-map-index must not contain config-item-map-named")
                }
                ConfigItem::Entry(_) => write_config_item_map_entry(None, item, xml_out)?,
            }
        } else {
            break;
        }

        index += 1;
    }

    xml_out.end_elem("config:config-item-map-indexed")?;

    Ok(())
}

fn write_config_item_map_named(
    name: &str,
    map: &ConfigItem,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("config:config-item-map-named")?;
    xml_out.attr_esc("config:name", name)?;

    for (name, item) in map.iter() {
        match item {
            ConfigItem::Value(value) => write_config_item(name, value, xml_out)?,
            ConfigItem::Set(_) => {
                panic!("config-item-map-index must not contain config-item-set")
            }
            ConfigItem::Vec(_) => {
                panic!("config-item-map-index must not contain config-item-map-index")
            }
            ConfigItem::Map(_) => {
                panic!("config-item-map-index must not contain config-item-map-named")
            }
            ConfigItem::Entry(_) => write_config_item_map_entry(Some(name), item, xml_out)?,
        }
    }

    xml_out.end_elem("config:config-item-map-named")?;

    Ok(())
}

fn write_config_item_map_entry(
    name: Option<&String>,
    map_entry: &ConfigItem,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("config:config-item-map-entry")?;
    if let Some(name) = name {
        xml_out.attr_esc("config:name", name)?;
    }

    for (name, item) in map_entry.iter() {
        match item {
            ConfigItem::Value(value) => write_config_item(name, value, xml_out)?,
            ConfigItem::Set(_) => write_config_item_set(name, item, xml_out)?,
            ConfigItem::Vec(_) => write_config_item_map_indexed(name, item, xml_out)?,
            ConfigItem::Map(_) => write_config_item_map_named(name, item, xml_out)?,
            ConfigItem::Entry(_) => {
                panic!("config:config-item-map-entry must not contain config-item-map-entry")
            }
        }
    }

    xml_out.end_elem("config:config-item-map-entry")?;

    Ok(())
}

fn write_config_item(
    name: &str,
    value: &ConfigValue,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    let is_empty = match value {
        ConfigValue::Base64Binary(t) => t.is_empty(),
        ConfigValue::String(t) => t.is_empty(),
        _ => false,
    };

    xml_out.elem_if(!is_empty, "config:config-item")?;

    xml_out.attr_esc("config:name", name)?;

    match value {
        ConfigValue::Base64Binary(v) => {
            xml_out.attr_str("config:type", "base64Binary")?;
            xml_out.text(v)?;
        }
        ConfigValue::Boolean(v) => {
            xml_out.attr_str("config:type", "boolean")?;
            xml_out.text(&v)?;
        }
        ConfigValue::DateTime(v) => {
            xml_out.attr_str("config:type", "datetime")?;
            xml_out.text(&v.format(DATETIME_FORMAT))?;
        }
        ConfigValue::Double(v) => {
            xml_out.attr_str("config:type", "double")?;
            xml_out.text(&v)?;
        }
        ConfigValue::Int(v) => {
            xml_out.attr_str("config:type", "int")?;
            xml_out.text(&v)?;
        }
        ConfigValue::Long(v) => {
            xml_out.attr_str("config:type", "long")?;
            xml_out.text(&v)?;
        }
        ConfigValue::Short(v) => {
            xml_out.attr_str("config:type", "short")?;
            xml_out.text(&v)?;
        }
        ConfigValue::String(v) => {
            xml_out.attr_str("config:type", "string")?;
            xml_out.text(v)?;
        }
    }

    xml_out.end_elem_if(!is_empty, "config:config-item")?;

    Ok(())
}

fn write_ods_styles(book: &mut WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let xmlns = book
        .xmlns
        .entry("styles.xml".into())
        .or_insert_with(NamespaceMap::new);

    xmlns.insert_str(
        "xmlns:meta",
        "urn:oasis:names:tc:opendocument:xmlns:meta:1.0",
    );
    xmlns.insert_str(
        "xmlns:office",
        "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    );
    xmlns.insert_str(
        "xmlns:fo",
        "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0",
    );
    xmlns.insert_str("xmlns:ooo", "http://openoffice.org/2004/office");
    xmlns.insert_str("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xmlns.insert_str("xmlns:dc", "http://purl.org/dc/elements/1.1/");
    xmlns.insert_str(
        "xmlns:style",
        "urn:oasis:names:tc:opendocument:xmlns:style:1.0",
    );
    xmlns.insert_str(
        "xmlns:text",
        "urn:oasis:names:tc:opendocument:xmlns:text:1.0",
    );
    xmlns.insert_str(
        "xmlns:dr3d",
        "urn:oasis:names:tc:opendocument:xmlns:dr3d:1.0",
    );
    xmlns.insert_str(
        "xmlns:svg",
        "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0",
    );
    xmlns.insert_str(
        "xmlns:chart",
        "urn:oasis:names:tc:opendocument:xmlns:chart:1.0",
    );
    xmlns.insert_str("xmlns:rpt", "http://openoffice.org/2005/report");
    xmlns.insert_str(
        "xmlns:table",
        "urn:oasis:names:tc:opendocument:xmlns:table:1.0",
    );
    xmlns.insert_str(
        "xmlns:number",
        "urn:oasis:names:tc:opendocument:xmlns:datastyle:1.0",
    );
    xmlns.insert_str("xmlns:ooow", "http://openoffice.org/2004/writer");
    xmlns.insert_str("xmlns:oooc", "http://openoffice.org/2004/calc");
    xmlns.insert_str("xmlns:of", "urn:oasis:names:tc:opendocument:xmlns:of:1.2");
    xmlns.insert_str("xmlns:tableooo", "http://openoffice.org/2009/table");
    xmlns.insert_str(
        "xmlns:calcext",
        "urn:org:documentfoundation:names:experimental:calc:xmlns:calcext:1.0",
    );
    xmlns.insert_str("xmlns:drawooo", "http://openoffice.org/2010/draw");
    xmlns.insert_str(
        "xmlns:draw",
        "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0",
    );
    xmlns.insert_str(
        "xmlns:loext",
        "urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0",
    );
    xmlns.insert_str(
        "xmlns:field",
        "urn:openoffice:names:experimental:ooo-ms-interop:xmlns:field:1.0",
    );
    xmlns.insert_str("xmlns:math", "http://www.w3.org/1998/Math/MathML");
    xmlns.insert_str(
        "xmlns:form",
        "urn:oasis:names:tc:opendocument:xmlns:form:1.0",
    );
    xmlns.insert_str(
        "xmlns:script",
        "urn:oasis:names:tc:opendocument:xmlns:script:1.0",
    );
    xmlns.insert_str("xmlns:dom", "http://www.w3.org/2001/xml-events");
    xmlns.insert_str("xmlns:xhtml", "http://www.w3.org/1999/xhtml");
    xmlns.insert_str("xmlns:grddl", "http://www.w3.org/2003/g/data-view#");
    xmlns.insert_str("xmlns:css3t", "http://www.w3.org/TR/css3-text/");
    xmlns.insert_str(
        "xmlns:presentation",
        "urn:oasis:names:tc:opendocument:xmlns:presentation:1.0",
    );

    xml_out.dtd("UTF-8")?;

    xml_out.elem("office:document-styles")?;
    write_xmlns(xmlns, xml_out)?;
    xml_out.attr_esc("office:version", book.version())?;

    write_office_font_face_decls(book, StyleOrigin::Styles, xml_out)?;
    write_office_styles(book, StyleOrigin::Styles, xml_out)?;
    write_office_automatic_styles(book, StyleOrigin::Styles, xml_out)?;
    write_office_master_styles(book, xml_out)?;

    xml_out.end_elem("office:document-styles")?;

    xml_out.close()?;

    Ok(())
}

fn write_ods_content(book: &mut WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let xmlns = book
        .xmlns
        .entry("content.xml".into())
        .or_insert_with(NamespaceMap::new);

    xmlns.insert_str(
        "xmlns:meta",
        "urn:oasis:names:tc:opendocument:xmlns:meta:1.0",
    );
    xmlns.insert_str(
        "xmlns:office",
        "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    );
    xmlns.insert_str(
        "xmlns:fo",
        "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0",
    );
    xmlns.insert_str("xmlns:ooo", "http://openoffice.org/2004/office");
    xmlns.insert_str("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xmlns.insert_str("xmlns:dc", "http://purl.org/dc/elements/1.1/");
    xmlns.insert_str(
        "xmlns:style",
        "urn:oasis:names:tc:opendocument:xmlns:style:1.0",
    );
    xmlns.insert_str(
        "xmlns:text",
        "urn:oasis:names:tc:opendocument:xmlns:text:1.0",
    );
    xmlns.insert_str(
        "xmlns:draw",
        "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0",
    );
    xmlns.insert_str(
        "xmlns:dr3d",
        "urn:oasis:names:tc:opendocument:xmlns:dr3d:1.0",
    );
    xmlns.insert_str(
        "xmlns:svg",
        "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0",
    );
    xmlns.insert_str(
        "xmlns:chart",
        "urn:oasis:names:tc:opendocument:xmlns:chart:1.0",
    );
    xmlns.insert_str("xmlns:rpt", "http://openoffice.org/2005/report");
    xmlns.insert_str(
        "xmlns:table",
        "urn:oasis:names:tc:opendocument:xmlns:table:1.0",
    );
    xmlns.insert_str(
        "xmlns:number",
        "urn:oasis:names:tc:opendocument:xmlns:datastyle:1.0",
    );
    xmlns.insert_str("xmlns:ooow", "http://openoffice.org/2004/writer");
    xmlns.insert_str("xmlns:oooc", "http://openoffice.org/2004/calc");
    xmlns.insert_str("xmlns:of", "urn:oasis:names:tc:opendocument:xmlns:of:1.2");
    xmlns.insert_str("xmlns:tableooo", "http://openoffice.org/2009/table");
    xmlns.insert_str(
        "xmlns:calcext",
        "urn:org:documentfoundation:names:experimental:calc:xmlns:calcext:1.0",
    );
    xmlns.insert_str("xmlns:drawooo", "http://openoffice.org/2010/draw");
    xmlns.insert_str(
        "xmlns:loext",
        "urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0",
    );
    xmlns.insert_str(
        "xmlns:field",
        "urn:openoffice:names:experimental:ooo-ms-interop:xmlns:field:1.0",
    );
    xmlns.insert_str("xmlns:math", "http://www.w3.org/1998/Math/MathML");
    xmlns.insert_str(
        "xmlns:form",
        "urn:oasis:names:tc:opendocument:xmlns:form:1.0",
    );
    xmlns.insert_str(
        "xmlns:script",
        "urn:oasis:names:tc:opendocument:xmlns:script:1.0",
    );
    xmlns.insert_str("xmlns:dom", "http://www.w3.org/2001/xml-events");
    xmlns.insert_str("xmlns:xforms", "http://www.w3.org/2002/xforms");
    xmlns.insert_str("xmlns:xsd", "http://www.w3.org/2001/XMLSchema");
    xmlns.insert_str("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
    xmlns.insert_str(
        "xmlns:formx",
        "urn:openoffice:names:experimental:ooxml-odf-interop:xmlns:form:1.0",
    );
    xmlns.insert_str("xmlns:xhtml", "http://www.w3.org/1999/xhtml");
    xmlns.insert_str("xmlns:grddl", "http://www.w3.org/2003/g/data-view#");
    xmlns.insert_str("xmlns:css3t", "http://www.w3.org/TR/css3-text/");
    xmlns.insert_str(
        "xmlns:presentation",
        "urn:oasis:names:tc:opendocument:xmlns:presentation:1.0",
    );

    xml_out.dtd("UTF-8")?;

    xml_out.elem("office:document-content")?;
    write_xmlns(xmlns, xml_out)?;
    xml_out.attr_esc("office:version", book.version())?;

    write_office_scripts(book, xml_out)?;
    write_office_font_face_decls(book, StyleOrigin::Content, xml_out)?;
    write_office_automatic_styles(book, StyleOrigin::Content, xml_out)?;

    write_office_body(book, xml_out)?;

    xml_out.end_elem("office:document-content")?;

    xml_out.close()?;

    Ok(())
}

fn write_office_body(book: &WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.elem("office:body")?;
    xml_out.elem("office:spreadsheet")?;

    // extra tags. pass through only
    for tag in &book.extra {
        if tag.name() == "table:calculation-settings"
            || tag.name() == "table:label-ranges"
            || tag.name() == "table:tracked-changes"
            || tag.name() == "text:alphabetical-index-auto-mark-file"
            || tag.name() == "text:dde-connection-decls"
            || tag.name() == "text:sequence-decls"
            || tag.name() == "text:user-field-decls"
            || tag.name() == "text:variable-decls"
        {
            write_xmltag(tag, xml_out)?;
        }
    }

    write_content_validations(book, xml_out)?;

    for sheet in &book.sheets {
        write_sheet(book, sheet, xml_out)?;
    }

    // extra tags. pass through only
    for tag in &book.extra {
        if tag.name() == "table:consolidation"
            || tag.name() == "table:data-pilot-tables"
            || tag.name() == "table:database-ranges"
            || tag.name() == "table:dde-links"
            || tag.name() == "table:named-expressions"
            || tag.name() == "calcext:conditional-formats"
        {
            write_xmltag(tag, xml_out)?;
        }
    }

    xml_out.end_elem("office:spreadsheet")?;
    xml_out.end_elem("office:body")?;
    Ok(())
}

fn write_office_scripts(book: &WorkBook, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.elem_if(!book.scripts.is_empty(), "office:scripts")?;
    write_scripts(&book.scripts, xml_out)?;
    write_event_listeners(&book.event_listener, xml_out)?;
    xml_out.end_elem_if(!book.scripts.is_empty(), "office:scripts")?;
    Ok(())
}

fn write_content_validations(
    book: &WorkBook,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    if !book.validations.is_empty() {
        xml_out.elem("table:content-validations")?;

        for valid in book.validations.values() {
            xml_out.elem("table:content-validation")?;
            xml_out.attr_esc("table:name", valid.name())?;
            xml_out.attr_esc("table:condition", &format_validation_condition(valid))?;
            xml_out.attr_str(
                "table:allow-empty-cell",
                if valid.allow_empty() { "true" } else { "false" },
            )?;
            xml_out.attr_str(
                "table:display-list",
                match valid.display() {
                    ValidationDisplay::NoDisplay => "no",
                    ValidationDisplay::Unsorted => "unsorted",
                    ValidationDisplay::SortAscending => "sort-ascending",
                },
            )?;
            xml_out.attr_esc("table:base-cell-address", &valid.base_cell())?;

            if let Some(err) = valid.err() {
                xml_out.elem_if(err.text().is_some(), "table:error-message")?;
                xml_out.attr("table:display", &err.display())?;
                xml_out.attr("table:message-type", &err.msg_type())?;
                if let Some(title) = err.title() {
                    xml_out.attr_esc("table:title", title)?;
                }
                if let Some(text) = err.text() {
                    write_xmltag(text, xml_out)?;
                }
                xml_out.end_elem_if(err.text().is_some(), "table:error-message")?;
            }
            if let Some(err) = valid.help() {
                xml_out.elem_if(err.text().is_some(), "table:help-message")?;
                xml_out.attr("table:display", &err.display())?;
                if let Some(title) = err.title() {
                    xml_out.attr_esc("table:title", title)?;
                }
                if let Some(text) = err.text() {
                    write_xmltag(text, xml_out)?;
                }
                xml_out.end_elem_if(err.text().is_some(), "table:help-message")?;
            }

            xml_out.end_elem("table:content-validation")?;
        }
        xml_out.end_elem("table:content-validations")?;
    }

    Ok(())
}

#[derive(Debug)]
struct SplitCols {
    col: u32,
    col_to: u32,
    hidden: bool,
}

impl SplitCols {
    fn repeat(&self) -> u32 {
        self.col_to - self.col + 1
    }
}

/// Is the cell (partially) hidden?
fn split_hidden(ranges: &[CellRange], row: u32, col: u32, repeat: u32, out: &mut Vec<SplitCols>) {
    out.clear();
    if repeat == 1 {
        if ranges.iter().any(|v| v.contains(row, col)) {
            let range = SplitCols {
                col,
                col_to: col,
                hidden: true,
            };
            out.push(range);
        } else {
            let range = SplitCols {
                col,
                col_to: col,
                hidden: false,
            };
            out.push(range);
        }
    } else {
        let ranges: Vec<_> = ranges
            .iter()
            .filter(|v| row >= v.row() && row <= v.to_row())
            .collect();

        let mut range: Option<SplitCols> = None;
        'col_loop: for c in col..col + repeat {
            for r in &ranges {
                if c >= r.col() && c <= r.to_col() {
                    if let Some(range) = &mut range {
                        if range.hidden {
                            range.col_to = c;
                        } else {
                            let v = mem::replace(
                                range,
                                SplitCols {
                                    col: c,
                                    col_to: c,
                                    hidden: true,
                                },
                            );
                            out.push(v);
                        }
                    } else {
                        range.replace(SplitCols {
                            col: c,
                            col_to: c,
                            hidden: true,
                        });
                    }

                    continue 'col_loop;
                }
            }
            // not hidden
            if let Some(range) = &mut range {
                if range.hidden {
                    let v = mem::replace(
                        range,
                        SplitCols {
                            col: c,
                            col_to: c,
                            hidden: false,
                        },
                    );
                    out.push(v);
                } else {
                    range.col_to = c;
                }
            } else {
                range.replace(SplitCols {
                    col: c,
                    col_to: c,
                    hidden: false,
                });
            }
        }

        if let Some(range) = range {
            out.push(range);
        }
    }
}

/// Removes any outlived Ranges from the vector.
fn remove_outlived(ranges: &mut Vec<CellRange>, row: u32, col: u32) {
    *ranges = ranges
        .drain(..)
        .filter(|s| !s.out_looped(row, col))
        .collect();
}

fn write_sheet(
    book: &WorkBook,
    sheet: &Sheet,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("table:table")?;
    xml_out.attr_esc("table:name", &sheet.name)?;
    if let Some(style) = sheet.style.as_ref() {
        xml_out.attr_esc("table:style-name", style.as_str())?;
    }
    if let Some(print_ranges) = &sheet.print_ranges {
        xml_out.attr_esc("table:print-ranges", &format_cellranges(print_ranges))?;
    }
    if !sheet.print() {
        xml_out.attr_str("table:print", "false")?;
    }
    if !sheet.display() {
        xml_out.attr_str("table:display", "false")?;
    }

    for tag in &sheet.extra {
        if tag.name() == "table:title"
            || tag.name() == "table:desc"
            || tag.name() == "table:table-source"
            || tag.name() == "office:dde-source"
            || tag.name() == "table:scenario"
            || tag.name() == "office:forms"
            || tag.name() == "table:shapes"
        {
            write_xmltag(tag, xml_out)?;
        }
    }

    let max_cell = sheet.used_grid_size();

    write_table_columns(sheet, max_cell, xml_out)?;

    // list of current spans
    let mut spans = Vec::<CellRange>::new();
    let mut split = Vec::<SplitCols>::new();

    // table-row + table-cell
    let mut first_cell = true;
    let mut prev_row: u32 = 0;
    let mut prev_row_repeat: u32 = 1;
    let mut prev_col: u32 = 0;
    let mut row_group_count = 0;
    let mut row_header = false;

    let mut it = CellDataIter::new(sheet.data.range(..));
    while let Some(((cur_row, cur_col), cell)) = it.next() {
        // Row repeat count.
        let cur_row_repeat = if let Some(row_header) = sheet.row_header.get(&cur_row) {
            row_header.repeat
        } else {
            1
        };
        // Cell repeat count.
        let cur_col_repeat = cell.repeat;

        // There may be a lot of gaps of any kind in our data.
        // In the XML format there is no cell identification, every gap
        // must be filled with empty rows/columns. For this we need some
        // calculations.

        // For the repeat-counter we need to look forward.
        let (next_row, next_col, is_last_cell) = //_
            if let Some((next_row, next_col)) = it.peek_cell()
        {
            (next_row, next_col, false)
        } else {
            (max_cell.0, max_cell.1, true)
        };

        // Looking forward row-wise.
        let forward_delta_row = next_row - cur_row;
        // Column deltas are only relevant in the same row, but we need to
        // fill up to max used columns.
        let forward_delta_col = if forward_delta_row > 0 {
            max_cell.1 - cur_col
        } else {
            next_col - cur_col
        };

        // Looking backward row-wise.
        let backward_delta_row = cur_row - prev_row;
        // When a row changes our delta is from zero to cur_col.
        let backward_delta_col = if backward_delta_row > 0 {
            cur_col
        } else {
            cur_col - prev_col
        };

        // After the first cell there is always an open row tag that
        // needs to be closed.
        if backward_delta_row > 0 && !first_cell {
            write_end_prev_row(
                sheet,
                prev_row,
                prev_row_repeat,
                &mut row_group_count,
                &mut row_header,
                xml_out,
            )?;
        }

        // Any empty rows before this one?
        if backward_delta_row > 0 {
            if backward_delta_row < prev_row_repeat {
                return Err(OdsError::Ods(format!(
                    "{}: row-repeat of {} for row {} overlaps with the following row.",
                    sheet.name, prev_row_repeat, prev_row,
                )));
            }

            // The repeat counter for the empty rows before is reduced by the last
            // real repeat.
            let mut synth_row_repeat = backward_delta_row - prev_row_repeat;
            // At the very beginning last row is 0. But nothing has been written for it.
            // To account for this we add one repeat.
            if first_cell {
                synth_row_repeat += 1;
            }
            let synth_row = cur_row - synth_row_repeat;

            if synth_row_repeat > 0 {
                write_empty_rows_before(
                    sheet,
                    synth_row,
                    synth_row_repeat,
                    max_cell,
                    &mut row_group_count,
                    &mut row_header,
                    xml_out,
                )?;
            }
        }

        // Start a new row if there is a delta or we are at the start.
        // Fills in any blank cells before the current cell.
        if backward_delta_row > 0 || first_cell {
            write_start_current_row(
                sheet,
                cur_row,
                cur_row_repeat,
                backward_delta_col,
                &mut row_group_count,
                &mut row_header,
                xml_out,
            )?;
        }

        // Remove no longer usefull cell-spans.
        remove_outlived(&mut spans, cur_row, cur_col);

        // Split the current cell in visible/hidden ranges.
        split_hidden(&spans, cur_row, cur_col, cur_col_repeat, &mut split);

        // Maybe span, only if visible. That nicely eliminates all double hides.
        // Only check for the start cell in case of repeat.
        if let Some(span) = cell.extra.as_ref().map(|v| v.span) {
            if !split[0].hidden && (span.row_span > 1 || span.col_span > 1) {
                spans.push(CellRange::origin_span(cur_row, cur_col, span.into()));
            }
        }

        // And now to something completely different ...
        for s in &split {
            write_cell(book, cell, s.hidden, s.repeat(), xml_out)?;
        }

        // There may be some blank cells until the next one.
        if forward_delta_row > 0 {
            // Write empty cells to fill up to the max used column.
            // If there is some overlap with repeat, it can be ignored.
            let synth_delta_col = forward_delta_col.saturating_sub(cur_col_repeat);
            if synth_delta_col > 0 {
                split_hidden(
                    &spans,
                    cur_row,
                    cur_col + cur_col_repeat,
                    synth_delta_col,
                    &mut split,
                );
                for s in &split {
                    write_empty_cells(s.hidden, s.repeat(), xml_out)?;
                }
            }
        } else if forward_delta_col > 0 {
            // Write empty cells unto the next cell with data.
            // Fail on overlap with repeat.
            if forward_delta_col < cur_col_repeat {
                return Err(OdsError::Ods(format!(
                    "{}: col-repeat of {} for row/col {}/{} overlaps with the following cell.",
                    sheet.name, cur_col_repeat, cur_row, cur_col,
                )));
            }
            let synth_delta_col = forward_delta_col - cur_col_repeat;
            if synth_delta_col > 0 {
                split_hidden(
                    &spans,
                    cur_row,
                    cur_col + cur_col_repeat,
                    synth_delta_col,
                    &mut split,
                );
                for s in &split {
                    write_empty_cells(s.hidden, s.repeat(), xml_out)?;
                }
            }
        }

        // The last cell we will write? We can close the last row here,
        // where we have all the data.
        if is_last_cell {
            write_end_last_row(&mut row_group_count, &mut row_header, xml_out)?;
        }

        first_cell = false;
        prev_row = cur_row;
        prev_row_repeat = cur_row_repeat;
        prev_col = cur_col;
    }

    xml_out.end_elem("table:table")?;

    for tag in &sheet.extra {
        if tag.name() == "table:named-expressions" || tag.name() == "calcext:conditional-formats" {
            write_xmltag(tag, xml_out)?;
        }
    }

    Ok(())
}

fn write_empty_cells(
    hidden: bool,
    repeat: u32,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    if hidden {
        xml_out.empty("table:covered-table-cell")?;
    } else {
        xml_out.empty("table:table-cell")?;
    }
    if repeat > 1 {
        xml_out.attr("table:number-columns-repeated", &repeat)?;
    }

    Ok(())
}

fn write_start_current_row(
    sheet: &Sheet,
    cur_row: u32,
    cur_row_repeat: u32,
    backward_delta_col: u32,
    row_group_count: &mut u32,
    row_header: &mut bool,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    // groups
    for row_group in &sheet.group_rows {
        if row_group.from() == cur_row {
            *row_group_count += 1;
            xml_out.elem("table:table-row-group")?;
            if !row_group.display() {
                xml_out.attr_str("table:display", "false")?;
            }
        }
        // extra: if the group would start within our repeat range, it's started
        //        at the current row instead.
        if row_group.from() > cur_row && row_group.from() < cur_row + cur_row_repeat {
            *row_group_count += 1;
            xml_out.elem("table:table-row-group")?;
            if !row_group.display() {
                xml_out.attr_str("table:display", "false")?;
            }
        }
    }

    // print-header
    if let Some(header_rows) = &sheet.header_rows {
        if header_rows.from >= cur_row && header_rows.from < cur_row + cur_row_repeat {
            *row_header = true;
        }
    }
    if *row_header {
        xml_out.elem("table:table-header-rows")?;
    }

    // row
    xml_out.elem("table:table-row")?;
    if let Some(row_header) = sheet.valid_row_header(cur_row) {
        if row_header.repeat > 1 {
            xml_out.attr_esc("table:number-rows-repeated", &row_header.repeat)?;
        }
        if let Some(rowstyle) = row_header.style.as_ref() {
            xml_out.attr_esc("table:style-name", rowstyle.as_str())?;
        }
        if let Some(cellstyle) = row_header.cellstyle.as_ref() {
            xml_out.attr_esc("table:default-cell-style-name", cellstyle.as_str())?;
        }
        if row_header.visible != Visibility::Visible {
            xml_out.attr_esc("table:visibility", &row_header.visible)?;
        }
    }

    // Might not be the first column in this row.
    if backward_delta_col > 0 {
        xml_out.empty("table:table-cell")?;
        if backward_delta_col > 1 {
            xml_out.attr_esc("table:number-columns-repeated", &backward_delta_col)?;
        }
    }

    Ok(())
}

fn write_end_prev_row(
    sheet: &Sheet,
    last_r: u32,
    last_r_repeat: u32,
    row_group_count: &mut u32,
    row_header: &mut bool,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    // row
    xml_out.end_elem("table:table-row")?;
    if *row_header {
        xml_out.end_elem("table:table-header-rows")?;
    }

    // end of the print-header
    if let Some(header_rows) = &sheet.header_rows {
        if header_rows.to >= last_r && header_rows.to < last_r + last_r_repeat {
            *row_header = false;
        }
    }

    // groups
    for row_group in &sheet.group_rows {
        if row_group.to() == last_r {
            *row_group_count -= 1;
            xml_out.end_elem("table:table-row-group")?;
        }
        // the group end is somewhere inside the repeated range.
        if row_group.to() > last_r && row_group.to() < last_r + last_r_repeat {
            *row_group_count -= 1;
            xml_out.end_elem("table:table-row-group")?;
        }
    }

    Ok(())
}

fn write_end_last_row(
    row_group_count: &mut u32,
    row_header: &mut bool,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    // row
    xml_out.end_elem("table:table-row")?;

    // end of the print-header.
    // todo: might loose some empty rows?
    if *row_header {
        *row_header = false;
        xml_out.end_elem("table:table-header-rows")?;
    }

    // close all groups
    while *row_group_count > 0 {
        *row_group_count -= 1;
        xml_out.end_elem("table:table-row-group")?;
    }

    Ok(())
}

fn write_empty_rows_before(
    sheet: &Sheet,
    last_row: u32,
    last_row_repeat: u32,
    max_cell: (u32, u32),
    row_group_count: &mut u32,
    row_header: &mut bool,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    // Are there any row groups? Then we don't use repeat but write everything out.
    if !sheet.group_rows.is_empty() || sheet.header_rows.is_some() {
        for r in last_row..last_row + last_row_repeat {
            if *row_header {
                xml_out.end_elem("table:table-header-rows")?;
            }
            // end of the print-header
            if let Some(header_rows) = &sheet.header_rows {
                if header_rows.to == r {
                    *row_header = false;
                }
            }
            // groups
            for row_group in &sheet.group_rows {
                if row_group.to() == r {
                    *row_group_count -= 1;
                    xml_out.end_elem("table:table-row-group")?;
                }
            }
            for row_group in &sheet.group_rows {
                if row_group.from() == r {
                    *row_group_count += 1;
                    xml_out.elem("table:table-row-group")?;
                    if !row_group.display() {
                        xml_out.attr_str("table:display", "false")?;
                    }
                }
            }
            // start of print-header
            if let Some(header_rows) = &sheet.header_rows {
                if header_rows.from == r {
                    *row_header = true;
                }
            }
            if *row_header {
                xml_out.elem("table:table-header-rows")?;
            }
            // row
            write_empty_row(sheet, r, 1, max_cell, xml_out)?;
        }
    } else {
        write_empty_row(sheet, last_row, last_row_repeat, max_cell, xml_out)?;
    }

    Ok(())
}

fn write_empty_row(
    sheet: &Sheet,
    cur_row: u32,
    row_repeat: u32,
    max_cell: (u32, u32),
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("table:table-row")?;
    xml_out.attr("table:number-rows-repeated", &row_repeat)?;
    if let Some(row_header) = sheet.valid_row_header(cur_row) {
        if let Some(rowstyle) = row_header.style.as_ref() {
            xml_out.attr_esc("table:style-name", rowstyle.as_str())?;
        }
        if let Some(cellstyle) = row_header.cellstyle.as_ref() {
            xml_out.attr_esc("table:default-cell-style-name", cellstyle.as_str())?;
        }
        if row_header.visible != Visibility::Visible {
            xml_out.attr_esc("table:visibility", &row_header.visible)?;
        }
    }

    // We fill the empty spaces completely up to max columns.
    xml_out.empty("table:table-cell")?;
    xml_out.attr("table:number-columns-repeated", &max_cell.1)?;

    xml_out.end_elem("table:table-row")?;

    Ok(())
}

fn write_table_columns(
    sheet: &Sheet,
    max_cell: (u32, u32),
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    // determine column count
    let mut max_col = max_cell.1;
    for grp in &sheet.group_cols {
        max_col = max(max_col, grp.to + 1);
    }
    if let Some(header_cols) = &sheet.header_cols {
        max_col = max(max_col, header_cols.to + 1);
    }

    // table:table-column
    let mut c = 0;
    loop {
        if c >= max_col {
            break;
        }

        for grp in &sheet.group_cols {
            if c == grp.from() {
                xml_out.elem("table:table-column-group")?;
                if !grp.display() {
                    xml_out.attr_str("table:display", "false")?;
                }
            }
        }

        // print-header columns
        if let Some(header_cols) = &sheet.header_cols {
            if c >= header_cols.from && c <= header_cols.to {
                xml_out.elem("table:table-header-columns")?;
            }
        }

        xml_out.empty("table:table-column")?;
        let span = if let Some(col_header) = sheet.col_header.get(&c) {
            if col_header.span > 1 {
                xml_out.attr_esc("table:number-columns-repeated", &col_header.span)?;
            }
            if let Some(style) = col_header.style.as_ref() {
                xml_out.attr_esc("table:style-name", style.as_str())?;
            }
            if let Some(style) = col_header.cellstyle.as_ref() {
                xml_out.attr_esc("table:default-cell-style-name", style.as_str())?;
            }
            if col_header.visible != Visibility::Visible {
                xml_out.attr_esc("table:visibility", &col_header.visible)?;
            }

            col_header.span
        } else {
            1
        };

        if let Some(header_cols) = &sheet.header_cols {
            if c >= header_cols.from && c <= header_cols.to {
                xml_out.end_elem("table:table-header-columns")?;
            }
        }

        for col_group in &sheet.group_cols {
            if c == col_group.to() {
                xml_out.end_elem("table:table-column-group")?;
            }
        }

        c += span;
    }

    Ok(())
}

#[allow(clippy::single_char_add_str)]
fn write_cell(
    book: &WorkBook,
    cell: &CellData,
    is_hidden: bool,
    repeat: u32,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    let tag = if is_hidden {
        "table:covered-table-cell"
    } else {
        "table:table-cell"
    };

    let has_subs = cell.value != Value::Empty || cell.has_annotation() || cell.has_draw_frames();
    xml_out.elem_if(has_subs, tag)?;

    if let Some(formula) = &cell.formula {
        xml_out.attr_esc("table:formula", formula)?;
    }

    if repeat > 1 {
        xml_out.attr_esc("table:number-columns-repeated", &repeat)?;
    }

    // Direct style oder value based default style.
    if let Some(style) = &cell.style {
        xml_out.attr_esc("table:style-name", style.as_str())?;
    } else if let Some(style) = book.def_style(cell.value.value_type()) {
        xml_out.attr_esc("table:style-name", style.as_str())?;
    }

    // Content validation
    if let Some(validation_name) = cell.extra.as_ref().and_then(|v| v.validation_name.as_ref()) {
        xml_out.attr_esc("table:content-validation-name", validation_name.as_str())?;
    }

    // Spans
    if let Some(span) = cell.extra.as_ref().map(|v| v.span) {
        if span.row_span > 1 {
            xml_out.attr_esc("table:number-rows-spanned", &span.row_span)?;
        }
        if span.col_span > 1 {
            xml_out.attr_esc("table:number-columns-spanned", &span.col_span)?;
        }
    }
    if let Some(span) = cell.extra.as_ref().map(|v| v.matrix_span) {
        if span.row_span > 1 {
            xml_out.attr_esc("table:number-matrix-rows-spanned", &span.row_span)?;
        }
        if span.col_span > 1 {
            xml_out.attr_esc("table:number-matrix-columns-spanned", &span.col_span)?;
        }
    }

    // This finds the correct ValueFormat, but there is no way to use it.
    // Falls back to: Output the same string as needed for the value-attribute
    // and hope for the best. Seems to work well enough.
    //
    // let valuestyle = if let Some(style_name) = cell.style {
    //     book.find_value_format(style_name)
    // } else {
    //     None
    // };

    match &cell.value {
        Value::Empty => {}
        Value::Text(s) => {
            xml_out.attr_str("office:value-type", "string")?;
            for l in s.split('\n') {
                xml_out.elem_text_esc("text:p", l)?;
            }
        }
        Value::TextXml(t) => {
            xml_out.attr_str("office:value-type", "string")?;
            for tt in t.iter() {
                write_xmltag(tt, xml_out)?;
            }
        }
        Value::DateTime(d) => {
            xml_out.attr_str("office:value-type", "date")?;
            let value = d.format(DATETIME_FORMAT);
            xml_out.attr("office:date-value", &value)?;
            xml_out.elem("text:p")?;
            xml_out.text(&value)?;
            xml_out.end_elem("text:p")?;
        }
        Value::TimeDuration(d) => {
            xml_out.attr_str("office:value-type", "time")?;
            let value = format_duration2(*d);
            xml_out.attr("office:time-value", &value)?;
            xml_out.elem("text:p")?;
            xml_out.text(&value)?;
            xml_out.end_elem("text:p")?;
        }
        Value::Boolean(b) => {
            xml_out.attr_str("office:value-type", "boolean")?;
            xml_out.attr_str("office:boolean-value", if *b { "true" } else { "false" })?;
            xml_out.elem("text:p")?;
            xml_out.text_str(if *b { "true" } else { "false" })?;
            xml_out.end_elem("text:p")?;
        }
        Value::Currency(v, c) => {
            xml_out.attr_str("office:value-type", "currency")?;
            xml_out.attr_esc("office:currency", c)?;
            xml_out.attr("office:value", v)?;
            xml_out.elem("text:p")?;
            xml_out.text_esc(c)?;
            xml_out.text_str(" ")?;
            xml_out.text(v)?;
            xml_out.end_elem("text:p")?;
        }
        Value::Number(v) => {
            xml_out.attr_str("office:value-type", "float")?;
            xml_out.attr("office:value", v)?;
            xml_out.elem("text:p")?;
            xml_out.text(v)?;
            xml_out.end_elem("text:p")?;
        }
        Value::Percentage(v) => {
            xml_out.attr_str("office:value-type", "percentage")?;
            xml_out.attr("office:value", v)?;
            xml_out.elem("text:p")?;
            xml_out.text(v)?;
            xml_out.end_elem("text:p")?;
        }
    }

    if let Some(annotation) = cell.extra.as_ref().and_then(|v| v.annotation.as_ref()) {
        write_annotation(annotation, xml_out)?;
    }

    if let Some(draw_frames) = cell.extra.as_ref().map(|v| &v.draw_frames) {
        for draw_frame in draw_frames {
            write_draw_frame(draw_frame, xml_out)?;
        }
    }

    xml_out.end_elem_if(has_subs, tag)?;

    Ok(())
}

fn write_draw_frame(
    draw_frame: &DrawFrame,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("draw:frame")?;
    for (k, v) in draw_frame.attrmap().iter() {
        xml_out.attr_esc(k.as_ref(), v)?;
    }

    for content in draw_frame.content_ref() {
        match content {
            DrawFrameContent::Image(img) => {
                write_draw_image(img, xml_out)?;
            }
        }
    }

    if let Some(desc) = draw_frame.desc() {
        xml_out.elem("svg:desc")?;
        xml_out.text_esc(desc)?;
        xml_out.end_elem("svg:desc")?;
    }
    if let Some(title) = draw_frame.title() {
        xml_out.elem("svg:title")?;
        xml_out.text_esc(title)?;
        xml_out.end_elem("svg:title")?;
    }

    xml_out.end_elem("draw:frame")?;

    Ok(())
}

fn write_draw_image(
    draw_image: &DrawImage,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("draw:image")?;
    for (k, v) in draw_image.attrmap().iter() {
        xml_out.attr_esc(k.as_ref(), v)?;
    }

    if let Some(bin) = draw_image.get_binary_base64() {
        xml_out.elem("office:binary-data")?;
        xml_out.text(bin)?;
        xml_out.end_elem("office:binary-data")?;
    }

    for content in draw_image.get_text() {
        write_xmltag(content, xml_out)?;
    }

    xml_out.end_elem("draw:image")?;

    Ok(())
}

fn write_annotation(
    annotation: &Annotation,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("office:annotation")?;
    xml_out.attr("office:display", &annotation.display())?;
    xml_out.attr_esc("office:name", &annotation.name())?;
    for (k, v) in annotation.attrmap().iter() {
        xml_out.attr_esc(k.as_ref(), v)?;
    }

    if let Some(creator) = annotation.creator() {
        xml_out.elem("dc:creator")?;
        xml_out.text_esc(creator.as_str())?;
        xml_out.end_elem("dc:creator")?;
    }
    if let Some(date) = annotation.date() {
        xml_out.elem("dc:date")?;
        xml_out.text_esc(&date.format(DATETIME_FORMAT))?;
        xml_out.end_elem("dc:date")?;
    }
    for v in annotation.text() {
        write_xmltag(v, xml_out)?;
    }
    xml_out.end_elem("office:annotation")?;
    Ok(())
}

fn write_scripts(scripts: &Vec<Script>, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    for script in scripts {
        xml_out.elem("office:script")?;
        xml_out.attr_esc("script:language", &script.script_lang)?;

        for content in &script.script {
            write_xmlcontent(content, xml_out)?;
        }

        xml_out.end_elem("/office:script")?;
    }

    Ok(())
}

fn write_event_listeners(
    events: &HashMap<String, EventListener>,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for event in events.values() {
        xml_out.empty("script:event-listener")?;
        xml_out.attr_esc("script:event-name", &event.event_name)?;
        xml_out.attr_esc("script:language", &event.script_lang)?;
        xml_out.attr_esc("script:macro-name", &event.macro_name)?;
        xml_out.attr_esc("xlink:actuate", &event.actuate)?;
        xml_out.attr_esc("xlink:href", &event.href)?;
        xml_out.attr_esc("xlink:type", &event.link_type)?;
    }

    Ok(())
}

fn write_office_font_face_decls(
    book: &WorkBook,
    origin: StyleOrigin,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem_if(!book.fonts.is_empty(), "office:font-face-decls")?;
    write_style_font_face(&book.fonts, origin, xml_out)?;
    xml_out.end_elem_if(!book.fonts.is_empty(), "office:font-face-decls")?;
    Ok(())
}

fn write_style_font_face(
    fonts: &HashMap<String, FontFaceDecl>,
    origin: StyleOrigin,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for font in fonts.values().filter(|s| s.origin() == origin) {
        xml_out.empty("style:font-face")?;
        xml_out.attr_esc("style:name", font.name())?;
        for (a, v) in font.attrmap().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    Ok(())
}

fn write_office_styles(
    book: &WorkBook,
    origin: StyleOrigin,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("office:styles")?;
    write_styles(book, origin, StyleUse::Default, xml_out)?;
    write_styles(book, origin, StyleUse::Named, xml_out)?;
    write_valuestyles(book, origin, StyleUse::Named, xml_out)?;
    write_valuestyles(book, origin, StyleUse::Default, xml_out)?;
    xml_out.end_elem("office:styles")?;
    Ok(())
}

fn write_office_automatic_styles(
    book: &WorkBook,
    origin: StyleOrigin,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("office:automatic-styles")?;
    write_pagestyles(&book.pagestyles, xml_out)?;
    write_styles(book, origin, StyleUse::Automatic, xml_out)?;
    write_valuestyles(book, origin, StyleUse::Automatic, xml_out)?;
    xml_out.end_elem("office:automatic-styles")?;
    Ok(())
}

fn write_office_master_styles(
    book: &WorkBook,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    xml_out.elem("office:master-styles")?;
    write_masterpage(&book.masterpages, xml_out)?;
    xml_out.end_elem("office:master-styles")?;
    Ok(())
}

fn write_styles(
    book: &WorkBook,
    origin: StyleOrigin,
    styleuse: StyleUse,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for style in book.colstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_colstyle(style, xml_out)?;
        }
    }
    for style in book.rowstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_rowstyle(style, xml_out)?;
        }
    }
    for style in book.tablestyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_tablestyle(style, xml_out)?;
        }
    }
    for style in book.cellstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_cellstyle(style, xml_out)?;
        }
    }
    for style in book.paragraphstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_paragraphstyle(style, xml_out)?;
        }
    }
    for style in book.textstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_textstyle(style, xml_out)?;
        }
    }
    for style in book.rubystyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_rubystyle(style, xml_out)?;
        }
    }
    for style in book.graphicstyles.values() {
        if style.origin() == origin && style.styleuse() == styleuse {
            write_graphicstyle(style, xml_out)?;
        }
    }

    // if let Some(stylemaps) = style.stylemaps() {
    //     for sm in stylemaps {
    //         xml_out.empty("style:map")?;
    //         xml_out.attr_esc("style:condition", sm.condition())?;
    //         xml_out.attr_esc("style:apply-style-name", sm.applied_style())?;
    //         xml_out.attr_esc("style:base-cell-address", &sm.base_cell().to_string())?;
    //     }
    // }

    Ok(())
}

fn write_tablestyle(style: &TableStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.tablestyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "table")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.tablestyle().is_empty() {
        xml_out.empty("style:table-properties")?;
        for (a, v) in style.tablestyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_rowstyle(style: &RowStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.rowstyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "table-row")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.rowstyle().is_empty() {
        xml_out.empty("style:table-row-properties")?;
        for (a, v) in style.rowstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_colstyle(style: &ColStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.colstyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "table-column")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.colstyle().is_empty() {
        xml_out.empty("style:table-column-properties")?;
        for (a, v) in style.colstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_cellstyle(style: &CellStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.cellstyle().is_empty()
        && style.paragraphstyle().is_empty()
        && style.textstyle().is_empty()
        && style.stylemaps().is_none();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "table-cell")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.cellstyle().is_empty() {
        xml_out.empty("style:table-cell-properties")?;
        for (a, v) in style.cellstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if !style.paragraphstyle().is_empty() {
        xml_out.empty("style:paragraph-properties")?;
        for (a, v) in style.paragraphstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if !style.textstyle().is_empty() {
        xml_out.empty("style:text-properties")?;
        for (a, v) in style.textstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if let Some(stylemaps) = style.stylemaps() {
        for sm in stylemaps {
            xml_out.empty("style:map")?;
            xml_out.attr_esc("style:condition", sm.condition())?;
            xml_out.attr_esc("style:apply-style-name", sm.applied_style().as_str())?;
            if let Some(r) = sm.base_cell() {
                xml_out.attr_esc("style:base-cell-address", r)?;
            }
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_paragraphstyle(
    style: &ParagraphStyle,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    let is_empty = style.paragraphstyle().is_empty() && style.textstyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "paragraph")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.paragraphstyle().is_empty() {
        if style.tabstops().is_none() {
            xml_out.empty("style:paragraph-properties")?;
            for (a, v) in style.paragraphstyle().iter() {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        } else {
            xml_out.elem("style:paragraph-properties")?;
            for (a, v) in style.paragraphstyle().iter() {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
            xml_out.elem("style:tab-stops")?;
            if let Some(tabstops) = style.tabstops() {
                for ts in tabstops {
                    xml_out.empty("style:tab-stop")?;
                    for (a, v) in ts.attrmap().iter() {
                        xml_out.attr_esc(a.as_ref(), v)?;
                    }
                }
            }
            xml_out.end_elem("style:tab-stops")?;
            xml_out.end_elem("style:paragraph-properties")?;
        }
    }
    if !style.textstyle().is_empty() {
        xml_out.empty("style:text-properties")?;
        for (a, v) in style.textstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_textstyle(style: &TextStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.textstyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "text")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.textstyle().is_empty() {
        xml_out.empty("style:text-properties")?;
        for (a, v) in style.textstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_rubystyle(style: &RubyStyle, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    let is_empty = style.rubystyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "ruby")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.rubystyle().is_empty() {
        xml_out.empty("style:ruby-properties")?;
        for (a, v) in style.rubystyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_graphicstyle(
    style: &GraphicStyle,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    let is_empty = style.graphicstyle().is_empty()
        && style.paragraphstyle().is_empty()
        && style.textstyle().is_empty();

    if style.styleuse() == StyleUse::Default {
        xml_out.elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.elem_if(!is_empty, "style:style")?;
        xml_out.attr_esc("style:name", style.name())?;
    }
    xml_out.attr_str("style:family", "graphic")?;
    for (a, v) in style.attrmap().iter() {
        match a.as_ref() {
            "style:name" => {}
            "style:family" => {}
            _ => {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }
    }

    if !style.graphicstyle().is_empty() {
        xml_out.empty("style:graphic-properties")?;
        for (a, v) in style.graphicstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if !style.paragraphstyle().is_empty() {
        xml_out.empty("style:paragraph-properties")?;
        for (a, v) in style.paragraphstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }
    if !style.textstyle().is_empty() {
        xml_out.empty("style:text-properties")?;
        for (a, v) in style.textstyle().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }
    }

    if style.styleuse() == StyleUse::Default {
        xml_out.end_elem_if(!is_empty, "style:default-style")?;
    } else {
        xml_out.end_elem_if(!is_empty, "style:style")?;
    }

    Ok(())
}

fn write_valuestyles(
    book: &WorkBook,
    origin: StyleOrigin,
    style_use: StyleUse,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    write_valuestyle(&book.formats_boolean, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_currency, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_datetime, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_number, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_percentage, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_text, origin, style_use, xml_out)?;
    write_valuestyle(&book.formats_timeduration, origin, style_use, xml_out)?;
    Ok(())
}

fn write_valuestyle<T: ValueFormatTrait>(
    value_formats: &HashMap<String, T>,
    origin: StyleOrigin,
    styleuse: StyleUse,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for value_format in value_formats
        .values()
        .filter(|s| s.origin() == origin && s.styleuse() == styleuse)
    {
        let tag = match value_format.value_type() {
            ValueType::Empty => unreachable!(),
            ValueType::Boolean => "number:boolean-style",
            ValueType::Number => "number:number-style",
            ValueType::Text => "number:text-style",
            ValueType::TextXml => "number:text-style",
            ValueType::TimeDuration => "number:time-style",
            ValueType::Percentage => "number:percentage-style",
            ValueType::Currency => "number:currency-style",
            ValueType::DateTime => "number:date-style",
        };

        xml_out.elem(tag)?;
        xml_out.attr_esc("style:name", value_format.name())?;
        for (a, v) in value_format.attrmap().iter() {
            xml_out.attr_esc(a.as_ref(), v)?;
        }

        if !value_format.textstyle().is_empty() {
            xml_out.empty("style:text-properties")?;
            for (a, v) in value_format.textstyle().iter() {
                xml_out.attr_esc(a.as_ref(), v)?;
            }
        }

        for part in value_format.parts() {
            let part_tag = match part.part_type() {
                FormatPartType::Boolean => "number:boolean",
                FormatPartType::Number => "number:number",
                FormatPartType::ScientificNumber => "number:scientific-number",
                FormatPartType::CurrencySymbol => "number:currency-symbol",
                FormatPartType::Day => "number:day",
                FormatPartType::Month => "number:month",
                FormatPartType::Year => "number:year",
                FormatPartType::Era => "number:era",
                FormatPartType::DayOfWeek => "number:day-of-week",
                FormatPartType::WeekOfYear => "number:week-of-year",
                FormatPartType::Quarter => "number:quarter",
                FormatPartType::Hours => "number:hours",
                FormatPartType::Minutes => "number:minutes",
                FormatPartType::Seconds => "number:seconds",
                FormatPartType::Fraction => "number:fraction",
                FormatPartType::AmPm => "number:am-pm",
                FormatPartType::Text => "number:text",
                FormatPartType::TextContent => "number:text-content",
                FormatPartType::FillCharacter => "number:fill-character",
            };

            if part.part_type() == FormatPartType::Text
                || part.part_type() == FormatPartType::CurrencySymbol
                || part.part_type() == FormatPartType::FillCharacter
            {
                let content = part.content().filter(|v| !v.is_empty());
                xml_out.elem_if(content.is_some(), part_tag)?;
                for (a, v) in part.attrmap().iter() {
                    xml_out.attr_esc(a.as_ref(), v)?;
                }
                if let Some(content) = content {
                    xml_out.text_esc(content)?;
                }
                xml_out.end_elem_if(content.is_some(), part_tag)?;
            } else if part.part_type() == FormatPartType::Number {
                if let Some(position) = part.position() {
                    xml_out.elem(part_tag)?;
                    for (a, v) in part.attrmap().iter() {
                        xml_out.attr_esc(a.as_ref(), v)?;
                    }

                    // embedded text
                    if let Some(content) = part.content() {
                        xml_out.elem("number:embedded-text")?;
                        xml_out.attr_esc("number:position", &position)?;
                        xml_out.text_esc(content)?;
                        xml_out.end_elem("number:embedded-text")?;
                    } else {
                        xml_out.empty("number:embedded-text")?;
                        xml_out.attr_esc("number:position", &position)?;
                    }

                    xml_out.end_elem(part_tag)?;
                } else {
                    xml_out.empty(part_tag)?;
                    for (a, v) in part.attrmap().iter() {
                        xml_out.attr_esc(a.as_ref(), v)?;
                    }
                }
            } else {
                xml_out.empty(part_tag)?;
                for (a, v) in part.attrmap().iter() {
                    xml_out.attr_esc(a.as_ref(), v)?;
                }
            }
        }

        if let Some(stylemaps) = value_format.stylemaps() {
            for sm in stylemaps {
                xml_out.empty("style:map")?;
                xml_out.attr_esc("style:condition", sm.condition())?;
                xml_out.attr_esc("style:apply-style-name", sm.applied_style())?;
            }
        }

        xml_out.end_elem(tag)?;
    }

    Ok(())
}

fn write_pagestyles(
    styles: &HashMap<PageStyleRef, PageStyle>,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for style in styles.values() {
        xml_out.elem("style:page-layout")?;
        xml_out.attr_esc("style:name", style.name())?;
        if let Some(master_page_usage) = &style.master_page_usage {
            xml_out.attr_esc("style:page-usage", master_page_usage)?;
        }

        if !style.style().is_empty() {
            xml_out.empty("style:page-layout-properties")?;
            for (k, v) in style.style().iter() {
                xml_out.attr_esc(k.as_ref(), v)?;
            }
        }

        xml_out.elem("style:header-style")?;
        xml_out.empty("style:header-footer-properties")?;
        if !style.headerstyle().style().is_empty() {
            for (k, v) in style.headerstyle().style().iter() {
                xml_out.attr_esc(k.as_ref(), v)?;
            }
        }
        xml_out.end_elem("style:header-style")?;

        xml_out.elem("style:footer-style")?;
        xml_out.empty("style:header-footer-properties")?;
        if !style.footerstyle().style().is_empty() {
            for (k, v) in style.footerstyle().style().iter() {
                xml_out.attr_esc(k.as_ref(), v)?;
            }
        }
        xml_out.end_elem("style:footer-style")?;

        xml_out.end_elem("style:page-layout")?;
    }

    Ok(())
}

fn write_masterpage(
    masterpages: &HashMap<MasterPageRef, MasterPage>,
    xml_out: &mut OdsXmlWriter<'_>,
) -> Result<(), OdsError> {
    for masterpage in masterpages.values() {
        xml_out.elem("style:master-page")?;
        xml_out.attr_esc("style:name", masterpage.name())?;
        if !masterpage.display_name().is_empty() {
            xml_out.attr_esc("style:display-name", masterpage.display_name())?;
        }
        if let Some(style) = masterpage.pagestyle() {
            xml_out.attr_esc("style:page-layout-name", style.as_str())?;
        }
        if let Some(next) = masterpage.next_masterpage() {
            xml_out.attr_esc("style:next-style-name", next.as_str())?;
        }

        // header
        xml_out.elem_if(!masterpage.header().is_empty(), "style:header")?;
        if !masterpage.header().display() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.header(), xml_out)?;
        xml_out.end_elem_if(!masterpage.header().is_empty(), "style:header")?;

        xml_out.elem_if(!masterpage.header_first().is_empty(), "style:header-first")?;
        if !masterpage.header_first().display() || masterpage.header_first().is_empty() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.header_first(), xml_out)?;
        xml_out.end_elem_if(!masterpage.header_first().is_empty(), "style:header-first")?;

        xml_out.elem_if(!masterpage.header_left().is_empty(), "style:header-left")?;
        if !masterpage.header_left().display() || masterpage.header_left().is_empty() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.header_left(), xml_out)?;
        xml_out.end_elem_if(!masterpage.header_left().is_empty(), "style:header-left")?;

        // footer
        xml_out.elem_if(!masterpage.footer().is_empty(), "style:footer")?;
        if !masterpage.footer().display() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.footer(), xml_out)?;
        xml_out.end_elem_if(!masterpage.footer().is_empty(), "style:footer")?;

        xml_out.elem_if(!masterpage.footer_first().is_empty(), "style:footer-first")?;
        if !masterpage.footer_first().display() || masterpage.footer_first().is_empty() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.footer_first(), xml_out)?;
        xml_out.end_elem_if(!masterpage.footer_first().is_empty(), "style:footer-first")?;

        xml_out.elem_if(!masterpage.footer_left().is_empty(), "style:footer-left")?;
        if !masterpage.footer_left().display() || masterpage.footer_left().is_empty() {
            xml_out.attr_str("style:display", "false")?;
        }
        write_regions(masterpage.footer_left(), xml_out)?;
        xml_out.end_elem_if(!masterpage.footer_left().is_empty(), "style:footer-left")?;

        xml_out.end_elem("style:master-page")?;
    }

    Ok(())
}

fn write_regions(hf: &HeaderFooter, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    if !hf.left().is_empty() {
        xml_out.elem("style:region-left")?;
        for v in hf.left() {
            write_xmltag(v, xml_out)?;
        }
        xml_out.end_elem("style:region-left")?;
    }
    if !hf.center().is_empty() {
        xml_out.elem("style:region-center")?;
        for v in hf.center() {
            write_xmltag(v, xml_out)?;
        }
        xml_out.end_elem("style:region-center")?;
    }
    if !hf.right().is_empty() {
        xml_out.elem("style:region-right")?;
        for v in hf.right() {
            write_xmltag(v, xml_out)?;
        }
        xml_out.end_elem("style:region-right")?;
    }
    for content in hf.content() {
        write_xmltag(content, xml_out)?;
    }

    Ok(())
}

fn write_xmlcontent(x: &XmlContent, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    match x {
        XmlContent::Text(t) => {
            xml_out.text_esc(t)?;
        }
        XmlContent::Tag(t) => {
            write_xmltag(t, xml_out)?;
        }
    }
    Ok(())
}

fn write_xmltag(x: &XmlTag, xml_out: &mut OdsXmlWriter<'_>) -> Result<(), OdsError> {
    xml_out.elem_if(!x.is_empty(), x.name())?;

    for (k, v) in x.attrmap().iter() {
        xml_out.attr_esc(k.as_ref(), v)?;
    }

    for c in x.content() {
        match c {
            XmlContent::Text(t) => {
                xml_out.text_esc(t)?;
            }
            XmlContent::Tag(t) => {
                write_xmltag(t, xml_out)?;
            }
        }
    }

    xml_out.end_elem_if(!x.is_empty(), x.name())?;

    Ok(())
}

// All extra entries from the manifest.
fn write_ods_extra<W: Write + Seek>(
    ctx: &mut OdsContext<W>,
    book: &WorkBook,
) -> Result<(), OdsError> {
    for manifest in book.manifest.values() {
        if !matches!(
            manifest.full_path.as_str(),
            "/" | "settings.xml" | "styles.xml" | "content.xml" | "meta.xml"
        ) {
            if manifest.is_dir() {
                ctx.zip_writer
                    .add_directory(&manifest.full_path, FileOptions::<()>::default())?;
            } else {
                ctx.zip_writer.start_file(
                    manifest.full_path.as_str(),
                    FileOptions::<()>::default().compression_method(ctx.compression),
                )?;
                if let Some(buf) = &manifest.buffer {
                    ctx.zip_writer.write_all(buf.as_slice())?;
                }
            }
        }
    }

    Ok(())
}

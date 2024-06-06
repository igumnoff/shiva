//!
//! Workbook
//!

use get_size::GetSize;
use get_size_derive::GetSize;
use std::borrow::Borrow;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

use icu_locid::{locale, Locale};

use crate::config::Config;
use crate::defaultstyles::{DefaultFormat, DefaultStyle};
use crate::ds::detach::{Detach, Detached};
use crate::format::ValueFormatTrait;
use crate::io::read::default_settings;
use crate::io::NamespaceMap;
use crate::manifest::Manifest;
use crate::metadata::Metadata;
use crate::sheet_::Sheet;
use crate::style::{
    ColStyle, ColStyleRef, FontFaceDecl, GraphicStyle, GraphicStyleRef, MasterPage, MasterPageRef,
    PageStyle, PageStyleRef, ParagraphStyle, ParagraphStyleRef, RowStyle, RowStyleRef, RubyStyle,
    RubyStyleRef, TableStyle, TableStyleRef, TextStyle, TextStyleRef,
};
use crate::validation::{Validation, ValidationRef};
use crate::value_::ValueType;
use crate::xlink::{XLinkActuate, XLinkType};
use crate::xmltree::{XmlContent, XmlTag};
use crate::{
    locale, CellStyle, CellStyleRef, HashMap, ValueFormatBoolean, ValueFormatCurrency,
    ValueFormatDateTime, ValueFormatNumber, ValueFormatPercentage, ValueFormatRef, ValueFormatText,
    ValueFormatTimeDuration,
};

/// Book is the main structure for the Spreadsheet.
#[derive(Clone, GetSize)]
pub struct WorkBook {
    /// The data.
    pub(crate) sheets: Vec<Detach<Sheet>>,

    /// ODS Version
    pub(crate) version: String,

    /// FontDecl hold the style:font-face elements
    pub(crate) fonts: HashMap<String, FontFaceDecl>,

    /// Auto-Styles. Maps the prefix to a number.
    pub(crate) autonum: HashMap<String, u32>,

    /// Scripts
    pub(crate) scripts: Vec<Script>,
    pub(crate) event_listener: HashMap<String, EventListener>,

    /// Styles hold the style:style elements.
    pub(crate) tablestyles: HashMap<TableStyleRef, TableStyle>,
    pub(crate) rowstyles: HashMap<RowStyleRef, RowStyle>,
    pub(crate) colstyles: HashMap<ColStyleRef, ColStyle>,
    pub(crate) cellstyles: HashMap<CellStyleRef, CellStyle>,
    pub(crate) paragraphstyles: HashMap<ParagraphStyleRef, ParagraphStyle>,
    pub(crate) textstyles: HashMap<TextStyleRef, TextStyle>,
    pub(crate) rubystyles: HashMap<RubyStyleRef, RubyStyle>,
    pub(crate) graphicstyles: HashMap<GraphicStyleRef, GraphicStyle>,

    /// Value-styles are actual formatting instructions for various datatypes.
    /// Represents the various number:xxx-style elements.
    pub(crate) formats_boolean: HashMap<String, ValueFormatBoolean>,
    pub(crate) formats_number: HashMap<String, ValueFormatNumber>,
    pub(crate) formats_percentage: HashMap<String, ValueFormatPercentage>,
    pub(crate) formats_currency: HashMap<String, ValueFormatCurrency>,
    pub(crate) formats_text: HashMap<String, ValueFormatText>,
    pub(crate) formats_datetime: HashMap<String, ValueFormatDateTime>,
    pub(crate) formats_timeduration: HashMap<String, ValueFormatTimeDuration>,

    /// Default-styles per Type.
    /// This is only used when writing the ods file.
    pub(crate) def_styles: HashMap<ValueType, CellStyleRef>,

    /// Page-layout data.
    pub(crate) pagestyles: HashMap<PageStyleRef, PageStyle>,
    pub(crate) masterpages: HashMap<MasterPageRef, MasterPage>,

    /// Validations.
    pub(crate) validations: HashMap<ValidationRef, Validation>,

    /// Configuration data. Internal cache for all values.
    /// Mapped into WorkBookConfig, SheetConfig.
    pub(crate) config: Detach<Config>,
    /// User modifiable config.
    pub(crate) workbook_config: WorkBookConfig,
    /// Keeps all the namespaces.
    pub(crate) xmlns: HashMap<String, NamespaceMap>,

    /// All extra files contained in the zip manifest are copied here.
    pub(crate) manifest: HashMap<String, Manifest>,

    /// Metadata
    pub(crate) metadata: Metadata,

    /// other stuff ...
    pub(crate) extra: Vec<XmlTag>,
}

impl fmt::Debug for WorkBook {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.version)?;
        for s in self.sheets.iter() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.fonts.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.tablestyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.rowstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.colstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.cellstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.paragraphstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.textstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.rubystyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.graphicstyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_boolean.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_number.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_percentage.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_currency.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_text.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_datetime.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.formats_timeduration.values() {
            writeln!(f, "{:?}", s)?;
        }
        for (t, s) in &self.def_styles {
            writeln!(f, "{:?} -> {:?}", t, s)?;
        }
        for s in self.pagestyles.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.masterpages.values() {
            writeln!(f, "{:?}", s)?;
        }
        for s in self.validations.values() {
            writeln!(f, "{:?}", s)?;
        }
        writeln!(f, "{:?}", &self.workbook_config)?;
        for v in self.manifest.values() {
            writeln!(f, "extras {:?}", v)?;
        }
        writeln!(f, "{:?}", &self.metadata)?;
        for xtr in &self.extra {
            writeln!(f, "extras {:?}", xtr)?;
        }
        Ok(())
    }
}

/// Autogenerate a stylename. Runs a counter with the prefix and
/// checks for existence.
fn auto_style_name2<K, V>(
    autonum: &mut HashMap<String, u32>,
    prefix: &str,
    styles: &HashMap<K, V>,
) -> String
where
    K: Borrow<str> + Hash + Eq,
{
    let mut cnt = if let Some(n) = autonum.get(prefix) {
        n + 1
    } else {
        0
    };

    let style_name = loop {
        let style_name = format!("{}{}", prefix, cnt);
        if !styles.contains_key(&style_name) {
            break style_name;
        }
        cnt += 1;
    };

    autonum.insert(prefix.to_string(), cnt);

    style_name
}

/// Autogenerate a stylename. Runs a counter with the prefix and
/// checks for existence.
fn auto_style_name<T>(
    autonum: &mut HashMap<String, u32>,
    prefix: &str,
    styles: &HashMap<String, T>,
) -> String {
    let mut cnt = if let Some(n) = autonum.get(prefix) {
        n + 1
    } else {
        0
    };

    let style_name = loop {
        let style_name = format!("{}{}", prefix, cnt);
        if !styles.contains_key(&style_name) {
            break style_name;
        }
        cnt += 1;
    };

    autonum.insert(prefix.to_string(), cnt);

    style_name
}

impl Default for WorkBook {
    fn default() -> Self {
        WorkBook::new(locale!("en"))
    }
}

impl WorkBook {
    /// Creates a new, completely empty workbook.
    ///
    /// WorkBook::locale_settings can be used to initialize default styles.
    pub fn new_empty() -> Self {
        WorkBook {
            sheets: Default::default(),
            version: "1.3".to_string(),
            fonts: Default::default(),
            autonum: Default::default(),
            scripts: Default::default(),
            event_listener: Default::default(),
            tablestyles: Default::default(),
            rowstyles: Default::default(),
            colstyles: Default::default(),
            cellstyles: Default::default(),
            paragraphstyles: Default::default(),
            textstyles: Default::default(),
            rubystyles: Default::default(),
            graphicstyles: Default::default(),
            formats_boolean: Default::default(),
            formats_number: Default::default(),
            formats_percentage: Default::default(),
            formats_currency: Default::default(),
            formats_text: Default::default(),
            formats_datetime: Default::default(),
            formats_timeduration: Default::default(),
            def_styles: Default::default(),
            pagestyles: Default::default(),
            masterpages: Default::default(),
            validations: Default::default(),
            config: default_settings(),
            workbook_config: Default::default(),
            extra: vec![],
            manifest: Default::default(),
            metadata: Default::default(),
            xmlns: Default::default(),
        }
    }

    /// Creates a new workbook, and initializes default styles according
    /// to the given locale.
    ///
    /// If the locale is not supported no ValueFormat's are set and all
    /// depends on the application opening the spreadsheet.
    ///
    /// The available locales can be activated via feature-flags.
    pub fn new(locale: Locale) -> Self {
        let mut wb = WorkBook::new_empty();
        wb.locale_settings(locale);
        wb
    }

    /// Creates a set of default formats and styles for every value-type.
    ///
    /// If the locale is not supported no ValueFormat's are set and all
    /// depends on the application opening the spreadsheet.
    ///
    /// The available locales can be activated via feature-flags.
    pub fn locale_settings(&mut self, locale: Locale) {
        if let Some(lf) = locale::localized_format(locale) {
            self.add_boolean_format(lf.boolean_format());
            self.add_number_format(lf.number_format());
            self.add_percentage_format(lf.percentage_format());
            self.add_currency_format(lf.currency_format());
            self.add_datetime_format(lf.date_format());
            self.add_datetime_format(lf.datetime_format());
            self.add_datetime_format(lf.time_of_day_format());
            self.add_timeduration_format(lf.time_interval_format());
        }

        self.add_cellstyle(CellStyle::new(DefaultStyle::bool(), &DefaultFormat::bool()));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::number(),
            &DefaultFormat::number(),
        ));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::percent(),
            &DefaultFormat::percent(),
        ));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::currency(),
            &DefaultFormat::currency(),
        ));
        self.add_cellstyle(CellStyle::new(DefaultStyle::date(), &DefaultFormat::date()));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::datetime(),
            &DefaultFormat::datetime(),
        ));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::time_of_day(),
            &DefaultFormat::time_of_day(),
        ));
        self.add_cellstyle(CellStyle::new(
            DefaultStyle::time_interval(),
            &DefaultFormat::time_interval(),
        ));

        self.add_def_style(ValueType::Boolean, DefaultStyle::bool());
        self.add_def_style(ValueType::Number, DefaultStyle::number());
        self.add_def_style(ValueType::Percentage, DefaultStyle::percent());
        self.add_def_style(ValueType::Currency, DefaultStyle::currency());
        self.add_def_style(ValueType::DateTime, DefaultStyle::date());
        self.add_def_style(ValueType::TimeDuration, DefaultStyle::time_interval());
    }

    /// ODS version. Defaults to 1.3.
    pub fn version(&self) -> &String {
        &self.version
    }

    /// ODS version. Defaults to 1.3.
    /// It's not advised to set another value.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Configuration flags.
    pub fn config(&self) -> &WorkBookConfig {
        &self.workbook_config
    }

    /// Configuration flags.
    pub fn config_mut(&mut self) -> &mut WorkBookConfig {
        &mut self.workbook_config
    }

    /// Number of sheets.
    pub fn num_sheets(&self) -> usize {
        self.sheets.len()
    }

    /// Finds the sheet index by the sheet-name.
    pub fn sheet_idx<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        for (idx, sheet) in self.sheets.iter().enumerate() {
            if sheet.name() == name.as_ref() {
                return Some(idx);
            }
        }
        None
    }

    /// Detaches a sheet.
    /// Useful if you have to make mutating calls to the workbook and
    /// the sheet intermixed.
    ///
    /// Warning
    ///
    /// The sheet has to be re-attached before saving the workbook.
    ///
    /// Panics
    ///
    /// Panics if the sheet has already been detached.
    /// Panics if n is out of bounds.
    pub fn detach_sheet(&mut self, n: usize) -> Detached<usize, Sheet> {
        self.sheets[n].detach(n)
    }

    /// Reattaches the sheet in the place it was before.
    ///
    /// Panics
    ///
    /// Panics if n is out of bounds.
    pub fn attach_sheet(&mut self, sheet: Detached<usize, Sheet>) {
        self.sheets[Detached::key(&sheet)].attach(sheet)
    }

    /// Returns a certain sheet.
    ///
    /// Panics
    ///
    /// Panics if n is out of bounds.
    pub fn sheet(&self, n: usize) -> &Sheet {
        self.sheets[n].as_ref()
    }

    /// Returns a certain sheet.
    ///
    /// Panics
    ///
    /// Panics if n does not exist.
    pub fn sheet_mut(&mut self, n: usize) -> &mut Sheet {
        self.sheets[n].as_mut()
    }

    /// Returns iterator over sheets.
    pub fn iter_sheets(&self) -> impl Iterator<Item = &Sheet> {
        self.sheets.iter().map(|sheet| &**sheet)
    }

    /// Inserts the sheet at the given position.
    pub fn insert_sheet(&mut self, i: usize, sheet: Sheet) {
        self.sheets.insert(i, sheet.into());
    }

    /// Appends a sheet.
    pub fn push_sheet(&mut self, sheet: Sheet) {
        self.sheets.push(sheet.into());
    }

    /// Removes a sheet from the table.
    ///
    /// Panics
    ///
    /// Panics if the sheet was detached.
    pub fn remove_sheet(&mut self, n: usize) -> Sheet {
        self.sheets.remove(n).take()
    }

    /// Scripts.
    pub fn add_script(&mut self, v: Script) {
        self.scripts.push(v);
    }

    /// Scripts.
    pub fn iter_scripts(&self) -> impl Iterator<Item = &Script> {
        self.scripts.iter()
    }

    /// Scripts
    pub fn scripts(&self) -> &Vec<Script> {
        &self.scripts
    }

    /// Scripts
    pub fn scripts_mut(&mut self) -> &mut Vec<Script> {
        &mut self.scripts
    }

    /// Event-Listener
    pub fn add_event_listener(&mut self, e: EventListener) {
        self.event_listener.insert(e.event_name.clone(), e);
    }

    /// Event-Listener
    pub fn remove_event_listener(&mut self, event_name: &str) -> Option<EventListener> {
        self.event_listener.remove(event_name)
    }

    /// Event-Listener
    pub fn iter_event_listeners(&self) -> impl Iterator<Item = &EventListener> {
        self.event_listener.values()
    }

    /// Event-Listener
    pub fn event_listener(&self, event_name: &str) -> Option<&EventListener> {
        self.event_listener.get(event_name)
    }

    /// Event-Listener
    pub fn event_listener_mut(&mut self, event_name: &str) -> Option<&mut EventListener> {
        self.event_listener.get_mut(event_name)
    }

    /// Adds a default-style for all new values.
    /// This information is only used when writing the data to the ODS file.
    pub fn add_def_style(&mut self, value_type: ValueType, style: CellStyleRef) {
        self.def_styles.insert(value_type, style);
    }

    /// Returns the default style name.
    pub fn def_style(&self, value_type: ValueType) -> Option<&CellStyleRef> {
        self.def_styles.get(&value_type)
    }

    /// Adds a font.
    pub fn add_font(&mut self, font: FontFaceDecl) {
        self.fonts.insert(font.name().to_string(), font);
    }

    /// Removes a font.
    pub fn remove_font(&mut self, name: &str) -> Option<FontFaceDecl> {
        self.fonts.remove(name)
    }

    /// Iterates the fonts.
    pub fn iter_fonts(&self) -> impl Iterator<Item = &FontFaceDecl> {
        self.fonts.values()
    }

    /// Returns the FontDecl.
    pub fn font(&self, name: &str) -> Option<&FontFaceDecl> {
        self.fonts.get(name)
    }

    /// Returns a mutable FontDecl.
    pub fn font_mut(&mut self, name: &str) -> Option<&mut FontFaceDecl> {
        self.fonts.get_mut(name)
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_tablestyle(&mut self, mut style: TableStyle) -> TableStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(&mut self.autonum, "ta", &self.tablestyles));
        }
        let sref = style.style_ref();
        self.tablestyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_tablestyle<S: AsRef<str>>(&mut self, name: S) -> Option<TableStyle> {
        self.tablestyles.remove(name.as_ref())
    }

    /// Iterates the table-styles.
    pub fn iter_table_styles(&self) -> impl Iterator<Item = &TableStyle> {
        self.tablestyles.values()
    }

    /// Returns the style.
    pub fn tablestyle<S: AsRef<str>>(&self, name: S) -> Option<&TableStyle> {
        self.tablestyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn tablestyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut TableStyle> {
        self.tablestyles.get_mut(name.as_ref())
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_rowstyle(&mut self, mut style: RowStyle) -> RowStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(&mut self.autonum, "ro", &self.rowstyles));
        }
        let sref = style.style_ref();
        self.rowstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_rowstyle<S: AsRef<str>>(&mut self, name: S) -> Option<RowStyle> {
        self.rowstyles.remove(name.as_ref())
    }

    /// Returns the style.
    pub fn rowstyle<S: AsRef<str>>(&self, name: S) -> Option<&RowStyle> {
        self.rowstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn rowstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut RowStyle> {
        self.rowstyles.get_mut(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_rowstyles(&self) -> impl Iterator<Item = &RowStyle> {
        self.rowstyles.values()
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_colstyle(&mut self, mut style: ColStyle) -> ColStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(&mut self.autonum, "co", &self.colstyles));
        }
        let sref = style.style_ref();
        self.colstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_colstyle<S: AsRef<str>>(&mut self, name: S) -> Option<ColStyle> {
        self.colstyles.remove(name.as_ref())
    }

    /// Returns the style.
    pub fn colstyle<S: AsRef<str>>(&self, name: S) -> Option<&ColStyle> {
        self.colstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn colstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut ColStyle> {
        self.colstyles.get_mut(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_colstyles(&self) -> impl Iterator<Item = &ColStyle> {
        self.colstyles.values()
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_cellstyle(&mut self, mut style: CellStyle) -> CellStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(&mut self.autonum, "ce", &self.cellstyles));
        }
        let sref = style.style_ref();
        self.cellstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_cellstyle<S: AsRef<str>>(&mut self, name: S) -> Option<CellStyle> {
        self.cellstyles.remove(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_cellstyles(&self) -> impl Iterator<Item = &CellStyle> {
        self.cellstyles.values()
    }

    /// Returns the style.
    pub fn cellstyle<S: AsRef<str>>(&self, name: S) -> Option<&CellStyle> {
        self.cellstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn cellstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut CellStyle> {
        self.cellstyles.get_mut(name.as_ref())
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_paragraphstyle(&mut self, mut style: ParagraphStyle) -> ParagraphStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(
                &mut self.autonum,
                "para",
                &self.paragraphstyles,
            ));
        }
        let sref = style.style_ref();
        self.paragraphstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_paragraphstyle<S: AsRef<str>>(&mut self, name: S) -> Option<ParagraphStyle> {
        self.paragraphstyles.remove(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_paragraphstyles(&self) -> impl Iterator<Item = &ParagraphStyle> {
        self.paragraphstyles.values()
    }

    /// Returns the style.
    pub fn paragraphstyle<S: AsRef<str>>(&self, name: S) -> Option<&ParagraphStyle> {
        self.paragraphstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn paragraphstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut ParagraphStyle> {
        self.paragraphstyles.get_mut(name.as_ref())
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_textstyle(&mut self, mut style: TextStyle) -> TextStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(&mut self.autonum, "txt", &self.textstyles));
        }
        let sref = style.style_ref();
        self.textstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_textstyle<S: AsRef<str>>(&mut self, name: S) -> Option<TextStyle> {
        self.textstyles.remove(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_textstyles(&self) -> impl Iterator<Item = &TextStyle> {
        self.textstyles.values()
    }

    /// Returns the style.
    pub fn textstyle<S: AsRef<str>>(&self, name: S) -> Option<&TextStyle> {
        self.textstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn textstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut TextStyle> {
        self.textstyles.get_mut(name.as_ref())
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_rubystyle(&mut self, mut style: RubyStyle) -> RubyStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(
                &mut self.autonum,
                "ruby",
                &self.rubystyles,
            ));
        }
        let sref = style.style_ref();
        self.rubystyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_rubystyle<S: AsRef<str>>(&mut self, name: S) -> Option<RubyStyle> {
        self.rubystyles.remove(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_rubystyles(&self) -> impl Iterator<Item = &RubyStyle> {
        self.rubystyles.values()
    }

    /// Returns the style.
    pub fn rubystyle<S: AsRef<str>>(&self, name: S) -> Option<&RubyStyle> {
        self.rubystyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn rubystyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut RubyStyle> {
        self.rubystyles.get_mut(name.as_ref())
    }

    /// Adds a style.
    /// Unnamed styles will be assigned an automatic name.
    pub fn add_graphicstyle(&mut self, mut style: GraphicStyle) -> GraphicStyleRef {
        if style.name().is_empty() {
            style.set_name(auto_style_name2(
                &mut self.autonum,
                "gr",
                &self.graphicstyles,
            ));
        }
        let sref = style.style_ref();
        self.graphicstyles.insert(style.style_ref(), style);
        sref
    }

    /// Removes a style.
    pub fn remove_graphicstyle<S: AsRef<str>>(&mut self, name: S) -> Option<GraphicStyle> {
        self.graphicstyles.remove(name.as_ref())
    }

    /// Returns iterator over styles.
    pub fn iter_graphicstyles(&self) -> impl Iterator<Item = &GraphicStyle> {
        self.graphicstyles.values()
    }

    /// Returns the style.
    pub fn graphicstyle<S: AsRef<str>>(&self, name: S) -> Option<&GraphicStyle> {
        self.graphicstyles.get(name.as_ref())
    }

    /// Returns the mutable style.
    pub fn graphicstyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut GraphicStyle> {
        self.graphicstyles.get_mut(name.as_ref())
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_boolean_format(&mut self, mut vstyle: ValueFormatBoolean) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(&mut self.autonum, "val_boolean", &self.formats_boolean).as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_boolean
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_boolean_format(&mut self, name: &str) -> Option<ValueFormatBoolean> {
        self.formats_boolean.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_boolean_formats(&self) -> impl Iterator<Item = &ValueFormatBoolean> {
        self.formats_boolean.values()
    }

    /// Returns the format.
    pub fn boolean_format(&self, name: &str) -> Option<&ValueFormatBoolean> {
        self.formats_boolean.get(name)
    }

    /// Returns the mutable format.
    pub fn boolean_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatBoolean> {
        self.formats_boolean.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_number_format(&mut self, mut vstyle: ValueFormatNumber) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(&mut self.autonum, "val_number", &self.formats_number).as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_number
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_number_format(&mut self, name: &str) -> Option<ValueFormatNumber> {
        self.formats_number.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_number_formats(&self) -> impl Iterator<Item = &ValueFormatNumber> {
        self.formats_number.values()
    }

    /// Returns the format.
    pub fn number_format(&self, name: &str) -> Option<&ValueFormatBoolean> {
        self.formats_boolean.get(name)
    }

    /// Returns the mutable format.
    pub fn number_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatBoolean> {
        self.formats_boolean.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_percentage_format(&mut self, mut vstyle: ValueFormatPercentage) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(
                    &mut self.autonum,
                    "val_percentage",
                    &self.formats_percentage,
                )
                .as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_percentage
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_percentage_format(&mut self, name: &str) -> Option<ValueFormatPercentage> {
        self.formats_percentage.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_percentage_formats(&self) -> impl Iterator<Item = &ValueFormatPercentage> {
        self.formats_percentage.values()
    }

    /// Returns the format.
    pub fn percentage_format(&self, name: &str) -> Option<&ValueFormatPercentage> {
        self.formats_percentage.get(name)
    }

    /// Returns the mutable format.
    pub fn percentage_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatPercentage> {
        self.formats_percentage.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_currency_format(&mut self, mut vstyle: ValueFormatCurrency) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(&mut self.autonum, "val_currency", &self.formats_currency).as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_currency
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_currency_format(&mut self, name: &str) -> Option<ValueFormatCurrency> {
        self.formats_currency.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_currency_formats(&self) -> impl Iterator<Item = &ValueFormatCurrency> {
        self.formats_currency.values()
    }

    /// Returns the format.
    pub fn currency_format(&self, name: &str) -> Option<&ValueFormatCurrency> {
        self.formats_currency.get(name)
    }

    /// Returns the mutable format.
    pub fn currency_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatCurrency> {
        self.formats_currency.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_text_format(&mut self, mut vstyle: ValueFormatText) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(&mut self.autonum, "val_text", &self.formats_text).as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_text.insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_text_format(&mut self, name: &str) -> Option<ValueFormatText> {
        self.formats_text.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_text_formats(&self) -> impl Iterator<Item = &ValueFormatText> {
        self.formats_text.values()
    }

    /// Returns the format.
    pub fn text_format(&self, name: &str) -> Option<&ValueFormatText> {
        self.formats_text.get(name)
    }

    /// Returns the mutable format.
    pub fn text_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatText> {
        self.formats_text.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_datetime_format(&mut self, mut vstyle: ValueFormatDateTime) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(&mut self.autonum, "val_datetime", &self.formats_datetime).as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_datetime
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_datetime_format(&mut self, name: &str) -> Option<ValueFormatDateTime> {
        self.formats_datetime.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_datetime_formats(&self) -> impl Iterator<Item = &ValueFormatDateTime> {
        self.formats_datetime.values()
    }

    /// Returns the format.
    pub fn datetime_format(&self, name: &str) -> Option<&ValueFormatDateTime> {
        self.formats_datetime.get(name)
    }

    /// Returns the mutable format.
    pub fn datetime_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatDateTime> {
        self.formats_datetime.get_mut(name)
    }

    /// Adds a value format.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_timeduration_format(
        &mut self,
        mut vstyle: ValueFormatTimeDuration,
    ) -> ValueFormatRef {
        if vstyle.name().is_empty() {
            vstyle.set_name(
                auto_style_name(
                    &mut self.autonum,
                    "val_timeduration",
                    &self.formats_timeduration,
                )
                .as_str(),
            );
        }
        let sref = vstyle.format_ref();
        self.formats_timeduration
            .insert(vstyle.name().to_string(), vstyle);
        sref
    }

    /// Removes the format.
    pub fn remove_timeduration_format(&mut self, name: &str) -> Option<ValueFormatTimeDuration> {
        self.formats_timeduration.remove(name)
    }

    /// Returns iterator over formats.
    pub fn iter_timeduration_formats(&self) -> impl Iterator<Item = &ValueFormatTimeDuration> {
        self.formats_timeduration.values()
    }

    /// Returns the format.
    pub fn timeduration_format(&self, name: &str) -> Option<&ValueFormatTimeDuration> {
        self.formats_timeduration.get(name)
    }

    /// Returns the mutable format.
    pub fn timeduration_format_mut(&mut self, name: &str) -> Option<&mut ValueFormatTimeDuration> {
        self.formats_timeduration.get_mut(name)
    }

    /// Adds a value PageStyle.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_pagestyle(&mut self, mut pstyle: PageStyle) -> PageStyleRef {
        if pstyle.name().is_empty() {
            pstyle.set_name(auto_style_name2(
                &mut self.autonum,
                "page",
                &self.pagestyles,
            ));
        }
        let sref = pstyle.style_ref();
        self.pagestyles.insert(pstyle.style_ref(), pstyle);
        sref
    }

    /// Removes the PageStyle.
    pub fn remove_pagestyle<S: AsRef<str>>(&mut self, name: S) -> Option<PageStyle> {
        self.pagestyles.remove(name.as_ref())
    }

    /// Returns iterator over formats.
    pub fn iter_pagestyles(&self) -> impl Iterator<Item = &PageStyle> {
        self.pagestyles.values()
    }

    /// Returns the PageStyle.
    pub fn pagestyle<S: AsRef<str>>(&self, name: S) -> Option<&PageStyle> {
        self.pagestyles.get(name.as_ref())
    }

    /// Returns the mutable PageStyle.
    pub fn pagestyle_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut PageStyle> {
        self.pagestyles.get_mut(name.as_ref())
    }

    /// Adds a value MasterPage.
    /// Unnamed formats will be assigned an automatic name.
    pub fn add_masterpage(&mut self, mut mpage: MasterPage) -> MasterPageRef {
        if mpage.name().is_empty() {
            mpage.set_name(auto_style_name2(&mut self.autonum, "mp", &self.masterpages));
        }
        let sref = mpage.masterpage_ref();
        self.masterpages.insert(mpage.masterpage_ref(), mpage);
        sref
    }

    /// Removes the MasterPage.
    pub fn remove_masterpage<S: AsRef<str>>(&mut self, name: S) -> Option<MasterPage> {
        self.masterpages.remove(name.as_ref())
    }

    /// Returns iterator over formats.
    pub fn iter_masterpages(&self) -> impl Iterator<Item = &MasterPage> {
        self.masterpages.values()
    }

    /// Returns the MasterPage.
    pub fn masterpage<S: AsRef<str>>(&self, name: S) -> Option<&MasterPage> {
        self.masterpages.get(name.as_ref())
    }

    /// Returns the mutable MasterPage.
    pub fn masterpage_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut MasterPage> {
        self.masterpages.get_mut(name.as_ref())
    }

    /// Adds a Validation.
    /// Nameless validations will be assigned a name.
    pub fn add_validation(&mut self, mut valid: Validation) -> ValidationRef {
        if valid.name().is_empty() {
            valid.set_name(auto_style_name2(
                &mut self.autonum,
                "val",
                &self.validations,
            ));
        }
        let vref = valid.validation_ref();
        self.validations.insert(valid.validation_ref(), valid);
        vref
    }

    /// Removes a Validation.
    pub fn remove_validation<S: AsRef<str>>(&mut self, name: S) -> Option<Validation> {
        self.validations.remove(name.as_ref())
    }

    /// Returns iterator over formats.
    pub fn iter_validations(&self) -> impl Iterator<Item = &Validation> {
        self.validations.values()
    }

    /// Returns the Validation.
    pub fn validation<S: AsRef<str>>(&self, name: S) -> Option<&Validation> {
        self.validations.get(name.as_ref())
    }

    /// Returns a mutable Validation.
    pub fn validation_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut Validation> {
        self.validations.get_mut(name.as_ref())
    }

    /// Adds a manifest entry, replaces an existing one with the same name.
    pub fn add_manifest(&mut self, manifest: Manifest) {
        self.manifest.insert(manifest.full_path.clone(), manifest);
    }

    /// Removes a manifest entry.
    pub fn remove_manifest(&mut self, path: &str) -> Option<Manifest> {
        self.manifest.remove(path)
    }

    /// Iterates the manifest.
    pub fn iter_manifest(&self) -> impl Iterator<Item = &Manifest> {
        self.manifest.values()
    }

    /// Returns the manifest entry for the path
    pub fn manifest(&self, path: &str) -> Option<&Manifest> {
        self.manifest.get(path)
    }

    /// Returns the manifest entry for the path
    pub fn manifest_mut(&mut self, path: &str) -> Option<&mut Manifest> {
        self.manifest.get_mut(path)
    }

    /// Gives access to meta-data.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Gives access to meta-data.
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

/// Subset of the Workbook wide configurations.
#[derive(Clone, Debug, GetSize)]
pub struct WorkBookConfig {
    /// Which table is active when opening.    
    pub active_table: String,
    /// Show grid in general. Per sheet definition take priority.
    pub show_grid: bool,
    /// Show page-breaks.
    pub show_page_breaks: bool,
    /// Are the sheet-tabs shown or not.
    pub has_sheet_tabs: bool,
}

impl Default for WorkBookConfig {
    fn default() -> Self {
        Self {
            active_table: "".to_string(),
            show_grid: true,
            show_page_breaks: false,
            has_sheet_tabs: true,
        }
    }
}

/// Script.
#[derive(Debug, Default, Clone, GetSize)]
pub struct Script {
    pub(crate) script_lang: String,
    pub(crate) script: Vec<XmlContent>,
}

impl Script {
    /// Script
    pub fn new() -> Self {
        Self {
            script_lang: "".to_string(),
            script: Default::default(),
        }
    }

    /// Script language
    pub fn script_lang(&self) -> &str {
        &self.script_lang
    }

    /// Script language
    pub fn set_script_lang(&mut self, script_lang: String) {
        self.script_lang = script_lang
    }

    /// Script
    pub fn script(&self) -> &Vec<XmlContent> {
        &self.script
    }

    /// Script
    pub fn set_script(&mut self, script: Vec<XmlContent>) {
        self.script = script
    }
}

/// Event-Listener.
#[derive(Debug, Clone, GetSize)]
pub struct EventListener {
    pub(crate) event_name: String,
    pub(crate) script_lang: String,
    pub(crate) macro_name: String,
    pub(crate) actuate: XLinkActuate,
    pub(crate) href: String,
    pub(crate) link_type: XLinkType,
}

impl EventListener {
    /// EventListener
    pub fn new() -> Self {
        Self {
            event_name: Default::default(),
            script_lang: Default::default(),
            macro_name: Default::default(),
            actuate: XLinkActuate::OnLoad,
            href: Default::default(),
            link_type: Default::default(),
        }
    }

    /// Name
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    /// Name
    pub fn set_event_name(&mut self, name: String) {
        self.event_name = name;
    }

    /// Script language
    pub fn script_lang(&self) -> &str {
        &self.script_lang
    }

    /// Script language
    pub fn set_script_lang(&mut self, lang: String) {
        self.script_lang = lang
    }

    /// Macro name
    pub fn macro_name(&self) -> &str {
        &self.macro_name
    }

    /// Macro name
    pub fn set_macro_name(&mut self, name: String) {
        self.macro_name = name
    }

    /// Actuate
    pub fn actuate(&self) -> XLinkActuate {
        self.actuate
    }

    /// Actuate
    pub fn set_actuate(&mut self, actuate: XLinkActuate) {
        self.actuate = actuate;
    }

    /// HRef
    pub fn href(&self) -> &str {
        &self.href
    }

    /// HRef
    pub fn set_href(&mut self, href: String) {
        self.href = href;
    }

    /// Link type
    pub fn link_type(&self) -> XLinkType {
        self.link_type
    }

    /// Link type
    pub fn set_link_type(&mut self, link_type: XLinkType) {
        self.link_type = link_type
    }
}

impl Default for EventListener {
    fn default() -> Self {
        Self {
            event_name: Default::default(),
            script_lang: Default::default(),
            macro_name: Default::default(),
            actuate: XLinkActuate::OnRequest,
            href: Default::default(),
            link_type: Default::default(),
        }
    }
}

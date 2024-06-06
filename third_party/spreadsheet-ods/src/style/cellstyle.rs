use crate::attrmap2::AttrMap2;
use crate::color::Rgb;
use crate::format::ValueFormatRef;
use crate::style::stylemap::StyleMap;
use crate::style::units::{
    Angle, Border, CellAlignVertical, CellProtect, FontSize, FontStyle, FontVariant, FontWeight,
    GlyphOrientation, Hyphenation, HyphenationLadderCount, Indent, Length, LetterSpacing,
    LineBreak, LineHeight, LineMode, LineStyle, LineType, LineWidth, Margin, PageBreak, PageNumber,
    ParaAlignVertical, Percent, PunctuationWrap, RotationAlign, TextAlign, TextAlignLast,
    TextAlignSource, TextAutoSpace, TextCombine, TextCondition, TextDisplay, TextEmphasize,
    TextEmphasizePosition, TextKeep, TextPosition, TextRelief, TextTransform, WrapOption,
    WritingDirection, WritingMode,
};
use crate::style::AnyStyleRef;
use crate::style::{
    border_line_width_string, border_string, color_string, shadow_string, text_position,
    StyleOrigin, StyleUse, TextStyleRef,
};
use core::borrow::Borrow;
use get_size::GetSize;
use get_size_derive::GetSize;
use icu_locid::Locale;

style_ref2!(CellStyleRef);

/// Describes the style information for a cell.
///
/// ```
/// use spreadsheet_ods::{pt, Length, CellStyle, WorkBook, Sheet, CellStyleRef};
/// use spreadsheet_ods::defaultstyles::DefaultFormat;
/// use spreadsheet_ods::color::Rgb;
/// use icu_locid::locale;
///
/// let mut book = WorkBook::new(locale!("en_US"));
///
/// let mut st_header = CellStyle::new("header", &DefaultFormat::default());
/// st_header.set_font_bold();
/// st_header.set_color(Rgb::new(255,255,0));
/// st_header.set_font_size(pt!(18));
/// let ref_header = book.add_cellstyle(st_header);
///
/// let mut sheet0 = Sheet::new("sheet 1");
/// sheet0.set_styled_value(0,0, "title", &ref_header);
///
/// // use a style defined later or elsewhere:
/// let ref_some = CellStyleRef::from("some_else");
/// sheet0.set_styled_value(1,0, "some", &ref_some);
///
/// ```
///
#[derive(Debug, Clone, GetSize)]
pub struct CellStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name.
    name: String,
    /// General attributes.
    attr: AttrMap2,
    /// Cell style attributes.
    cellstyle: AttrMap2,
    /// Paragraph style attributes.
    paragraphstyle: AttrMap2,
    /// Text style attributes.
    textstyle: AttrMap2,
    /// Style maps
    stylemaps: Option<Vec<StyleMap>>,
}

styles_styles2!(CellStyle, CellStyleRef);

impl CellStyle {
    /// Creates an empty style.
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            cellstyle: Default::default(),
            paragraphstyle: Default::default(),
            textstyle: Default::default(),
            stylemaps: None,
        }
    }

    /// Creates an empty style with the given name and a reference to a
    /// value format.
    pub fn new<S: AsRef<str>>(name: S, value_format: &ValueFormatRef) -> Self {
        let mut s = Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: String::from(name.as_ref()),
            attr: Default::default(),
            cellstyle: Default::default(),
            paragraphstyle: Default::default(),
            textstyle: Default::default(),
            stylemaps: None,
        };
        s.set_value_format(value_format);
        s
    }

    /// Reference to the value format.
    pub fn value_format(&self) -> Option<&str> {
        self.attr.attr("style:data-style-name")
    }

    /// Reference to the value format.
    pub fn set_value_format(&mut self, name: &ValueFormatRef) {
        self.attr
            .set_attr("style:data-style-name", name.as_ref().to_string());
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Allows access to all cell-style like attributes.
    pub fn cellstyle(&self) -> &AttrMap2 {
        &self.cellstyle
    }

    /// Allows access to all cell-style like attributes.
    pub fn cellstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.cellstyle
    }

    /// Allows access to all paragraph-style like attributes.
    pub fn paragraphstyle(&self) -> &AttrMap2 {
        &self.paragraphstyle
    }

    /// Allows access to all paragraph-style like attributes.
    pub fn paragraphstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.paragraphstyle
    }

    /// Allows access to all text-style like attributes.
    pub fn textstyle(&self) -> &AttrMap2 {
        &self.textstyle
    }

    /// Allows access to all text-style like attributes.
    pub fn textstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.textstyle
    }

    /// Adds a stylemap.
    pub fn push_stylemap(&mut self, stylemap: StyleMap) {
        self.stylemaps.get_or_insert_with(Vec::new).push(stylemap);
    }

    /// Returns the stylemaps
    pub fn stylemaps(&self) -> Option<&Vec<StyleMap>> {
        self.stylemaps.as_ref()
    }

    /// Returns the mutable stylemap.
    pub fn stylemaps_mut(&mut self) -> &mut Vec<StyleMap> {
        self.stylemaps.get_or_insert_with(Vec::new)
    }

    // Cell attributes.
    fo_background_color!(cellstyle);
    fo_border!(cellstyle);
    fo_padding!(cellstyle);
    fo_wrap_option!(cellstyle);
    fo_border_line_width!(cellstyle);
    style_cell_protect!(cellstyle);
    style_decimal_places!(cellstyle);
    style_diagonal!(cellstyle);
    style_direction!(cellstyle);
    style_glyph_orientation_vertical!(cellstyle);
    style_print_content!(cellstyle);
    style_repeat_content!(cellstyle);
    style_rotation_align!(cellstyle);
    style_rotation_angle!(cellstyle);
    style_shadow!(cellstyle);
    style_shrink_to_fit!(cellstyle);
    style_text_align_source!(cellstyle);
    style_vertical_align!(cellstyle);
    style_writing_mode!(cellstyle);

    // Paragraph attributes.

    // NOTE: Some attributes exist as both cell and as paragraph properties.
    //       They can't be mapped this way. On the other hand you cannot set
    //       them via LibreOffice either.

    // fo_background_color!(paragraphstyle);
    // fo_border!(paragraphstyle);
    fo_break!(paragraphstyle);
    fo_hyphenation!(paragraphstyle);
    fo_keep_together!(paragraphstyle);
    fo_keep_with_next!(paragraphstyle);
    fo_line_height!(paragraphstyle);
    fo_margin!(paragraphstyle);
    fo_orphans!(paragraphstyle);
    // fo_padding!(paragraphstyle);
    fo_text_align!(paragraphstyle);
    fo_text_align_last!(paragraphstyle);
    fo_text_indent!(paragraphstyle);
    fo_widows!(paragraphstyle);
    style_auto_text_indent!(paragraphstyle);
    style_background_transparency!(paragraphstyle);
    // fo_border_line_width!(paragraphstyle);
    style_contextual_spacing!(paragraphstyle);
    style_font_independent_line_spacing!(paragraphstyle);
    style_join_border!(paragraphstyle);
    style_justify_single_word!(paragraphstyle);
    style_line_break!(paragraphstyle);
    style_line_height_at_least!(paragraphstyle);
    style_line_spacing!(paragraphstyle);
    style_page_number!(paragraphstyle);
    style_punctuation_wrap!(paragraphstyle);
    style_register_true!(paragraphstyle);
    // style_shadow!(paragraphstyle);
    style_snap_to_layout_grid!(paragraphstyle);
    style_tab_stop_distance!(paragraphstyle);
    style_text_autospace!(paragraphstyle);
    style_vertical_align_para!(paragraphstyle);
    // style_writing_mode!(paragraphstyle);
    style_writing_mode_automatic!(paragraphstyle);
    style_line_number!(paragraphstyle);
    style_number_lines!(paragraphstyle);

    // NOTE: Some attributes exist as both cell and as text properties.
    //       They can't be mapped this way. On the other hand you cannot set
    //       them via LibreOffice either.

    // fo_background_color!(textstyle);
    fo_color!(textstyle);
    fo_locale!(textstyle);
    style_font_name!(textstyle);
    fo_font_size!(textstyle);
    fo_font_size_rel!(textstyle);
    fo_font_style!(textstyle);
    fo_font_weight!(textstyle);
    fo_font_variant!(textstyle);
    fo_font_attr!(textstyle);
    style_locale_asian!(textstyle);
    style_font_name_asian!(textstyle);
    style_font_size_asian!(textstyle);
    style_font_size_rel_asian!(textstyle);
    style_font_style_asian!(textstyle);
    style_font_weight_asian!(textstyle);
    style_font_attr_asian!(textstyle);
    style_locale_complex!(textstyle);
    style_font_name_complex!(textstyle);
    style_font_size_complex!(textstyle);
    style_font_size_rel_complex!(textstyle);
    style_font_style_complex!(textstyle);
    style_font_weight_complex!(textstyle);
    style_font_attr_complex!(textstyle);
    fo_hyphenate!(textstyle);
    fo_hyphenation_push_char_count!(textstyle);
    fo_hyphenation_remain_char_count!(textstyle);
    fo_letter_spacing!(textstyle);
    fo_text_shadow!(textstyle);
    fo_text_transform!(textstyle);
    style_font_relief!(textstyle);
    style_text_position!(textstyle);
    // style_rotation_angle!(textstyle);
    // style_rotation_scale!(textstyle);
    style_letter_kerning!(textstyle);
    style_text_combine!(textstyle);
    style_text_combine_start_char!(textstyle);
    style_text_combine_end_char!(textstyle);
    style_text_emphasize!(textstyle);
    style_text_line_through!(textstyle);
    style_text_outline!(textstyle);
    style_text_overline!(textstyle);
    style_text_underline!(textstyle);
    style_use_window_font_color!(textstyle);
    text_condition!(textstyle);
    text_display!(textstyle);

    // TODO: background image
}

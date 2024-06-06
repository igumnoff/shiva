use crate::attrmap2::AttrMap2;
use crate::color::Rgb;
use crate::style::units::{
    Angle, FontSize, FontStyle, FontVariant, FontWeight, Length, LetterSpacing, LineMode,
    LineStyle, LineType, LineWidth, Percent, RotationScale, TextCombine, TextCondition,
    TextDisplay, TextEmphasize, TextEmphasizePosition, TextPosition, TextRelief, TextTransform,
};
use crate::style::AnyStyleRef;
use crate::style::{color_string, shadow_string, text_position, StyleOrigin, StyleUse};
use core::borrow::Borrow;
use get_size::GetSize;
use get_size_derive::GetSize;
use icu_locid::Locale;

style_ref2!(TextStyleRef);

/// Text style.
/// This is not used for cell-formatting. Use CellStyle instead.
///
#[derive(Debug, Clone, GetSize)]
pub struct TextStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,
    /// General attributes
    attr: AttrMap2,
    /// Specific attributes
    textstyle: AttrMap2,
}

styles_styles2!(TextStyle, TextStyleRef);

impl TextStyle {
    /// Empty.
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            textstyle: Default::default(),
        }
    }

    /// A new named style.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: name.as_ref().to_string(),
            attr: Default::default(),
            textstyle: Default::default(),
        }
    }

    /// General attributes for the style.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// General attributes for the style.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// All text-attributes for the style.
    pub fn textstyle(&self) -> &AttrMap2 {
        &self.textstyle
    }

    /// All text-attributes for the style.
    pub fn textstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.textstyle
    }

    fo_background_color!(textstyle);
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
    style_rotation_angle!(textstyle);
    style_rotation_scale!(textstyle);
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
}

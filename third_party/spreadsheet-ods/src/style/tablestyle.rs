use crate::attrmap2::AttrMap2;
use crate::color::Rgb;
use crate::style::units::{
    Length, Margin, PageBreak, PageNumber, RelativeScale, TableAlign, TableBorderModel, TextKeep,
    WritingMode,
};
use crate::style::AnyStyleRef;
use crate::style::{color_string, shadow_string, MasterPageRef, StyleOrigin, StyleUse};
use core::borrow::Borrow;
use get_size::GetSize;
use get_size_derive::GetSize;

style_ref2!(TableStyleRef);

/// Describes the style information for a table.
///
#[derive(Debug, Clone, GetSize)]
pub struct TableStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,
    /// General attributes
    attr: AttrMap2,
    /// Table style properties
    tablestyle: AttrMap2,
}

styles_styles2!(TableStyle, TableStyleRef);

impl TableStyle {
    /// empty
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            tablestyle: Default::default(),
        }
    }

    /// Creates a new Style.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: String::from(name.as_ref()),
            attr: Default::default(),
            tablestyle: Default::default(),
        }
    }

    style_master_page!(attr);

    /// Access to all stored attributes.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Access to all stored attributes.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Access to all style attributes.
    pub fn tablestyle(&self) -> &AttrMap2 {
        &self.tablestyle
    }

    /// Access to all style attributes.
    pub fn tablestyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.tablestyle
    }

    fo_background_color!(tablestyle);
    fo_break!(tablestyle);
    fo_keep_with_next!(tablestyle);
    fo_margin!(tablestyle);
    style_may_break_between_rows!(tablestyle);
    style_page_number!(tablestyle);
    style_rel_width!(tablestyle);
    style_width!(tablestyle);
    style_shadow!(tablestyle);
    style_writing_mode!(tablestyle);

    table_align!(tablestyle);
    table_border_model!(tablestyle);
    table_display!(tablestyle);
    table_tab_color!(tablestyle);
}

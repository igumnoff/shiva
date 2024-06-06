use crate::color::Rgb;
use get_size::GetSize;

use crate::attrmap2::AttrMap2;
use crate::style::units::{Length, PageBreak, TextKeep};
use crate::style::AnyStyleRef;
use crate::style::ParseStyleAttr;
use crate::style::{color_string, StyleOrigin, StyleUse};
use crate::OdsError;
use get_size_derive::GetSize;
use std::borrow::Borrow;

style_ref2!(RowStyleRef);

/// Describes the style information for a table row.
/// Hardly ever used. It's easier to set the row_height via
/// Sheet::set_row_height.
///
#[derive(Debug, Clone, GetSize)]
pub struct RowStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,
    /// General attributes
    attr: AttrMap2,
    /// Table style properties
    rowstyle: AttrMap2,
}

styles_styles2!(RowStyle, RowStyleRef);

impl RowStyle {
    /// empty
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            rowstyle: Default::default(),
        }
    }

    /// New Style.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: name.as_ref().to_string(),
            attr: Default::default(),
            rowstyle: Default::default(),
        }
    }

    /// General attributes.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// General attributes.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Style attributes.
    pub fn rowstyle(&self) -> &AttrMap2 {
        &self.rowstyle
    }

    /// Style attributes.
    pub fn rowstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.rowstyle
    }

    fo_background_color!(rowstyle);
    fo_break!(rowstyle);
    fo_keep_together!(rowstyle);
    style_min_row_height!(rowstyle);
    style_row_height!(rowstyle);
    style_use_optimal_row_height!(rowstyle);
}

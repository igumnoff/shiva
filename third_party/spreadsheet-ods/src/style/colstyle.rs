use get_size::GetSize;
use get_size_derive::GetSize;

use crate::attrmap2::AttrMap2;
use crate::style::units::{Length, PageBreak};
use crate::style::AnyStyleRef;
use crate::style::ParseStyleAttr;
use crate::style::{rel_width_string, StyleOrigin, StyleUse};
use crate::OdsError;
use std::borrow::Borrow;

style_ref2!(ColStyleRef);

/// Describes the style information for a table column.
/// Hardly ever used. It's easier to set the col_width via
/// Sheet::set_col_width
///
#[derive(Debug, Clone, GetSize)]
pub struct ColStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,

    /// General attributes
    attr: AttrMap2,
    /// Column style properties
    colstyle: AttrMap2,
}

styles_styles2!(ColStyle, ColStyleRef);

impl ColStyle {
    /// empty
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            colstyle: Default::default(),
        }
    }

    /// New Style.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: name.as_ref().to_string(),
            attr: Default::default(),
            colstyle: Default::default(),
        }
    }

    /// Attributes
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Attributes
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Style attributes
    pub fn colstyle(&self) -> &AttrMap2 {
        &self.colstyle
    }

    /// Style attributes
    pub fn colstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.colstyle
    }

    fo_break!(colstyle);
    style_column_width!(colstyle);
    style_rel_column_width!(colstyle);
    style_use_optimal_column_width!(colstyle);
}

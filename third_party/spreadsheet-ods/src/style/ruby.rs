use crate::attrmap2::AttrMap2;
use crate::style::AnyStyleRef;
use crate::style::{StyleOrigin, StyleUse};
use get_size::GetSize;
use get_size_derive::GetSize;
use std::borrow::Borrow;

style_ref2!(RubyStyleRef);

/// Text style.
/// This is not used for cell-formatting. Use CellStyle instead.
///
#[derive(Debug, Clone, GetSize)]
pub struct RubyStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,
    /// General attributes
    attr: AttrMap2,
    /// Specific attributes
    rubystyle: AttrMap2,
}

styles_styles2!(RubyStyle, RubyStyleRef);

impl RubyStyle {
    /// Empty.
    pub fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            rubystyle: Default::default(),
        }
    }

    /// A new named style.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: name.as_ref().to_string(),
            attr: Default::default(),
            rubystyle: Default::default(),
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
    pub fn rubystyle(&self) -> &AttrMap2 {
        &self.rubystyle
    }

    /// All text-attributes for the style.
    pub fn rubystyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.rubystyle
    }
}

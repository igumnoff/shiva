use crate::attrmap2::AttrMap2;
use crate::style::units::{
    FontFamilyGeneric, FontPitch, FontStretch, FontStyle, FontVariant, FontWeight,
};
use crate::style::StyleOrigin;
use get_size::GetSize;
use get_size_derive::GetSize;

/// The <style:font-face> element represents a font face declaration which documents the
/// properties of a font used in a document.
///
/// OpenDocument font face declarations directly correspond to the @font-face font description of
/// CSS2 (see §15.3.1) and the font-face element of SVG (see §20.8.3).
///
/// OpenDocument font face declarations may have an unique name. This name can be used inside
/// styles (as an attribute of <style:text-properties> element) as value of the style:fontname attribute to select a font face declaration. If a font face declaration is referenced by name,
/// the font-matching algorithms for selecting a font declaration based on the font-family, font-style,
/// font-variant, font-weight and font-size descriptors are not used but the referenced font face
/// declaration is used directly. (See §15.5 CSS2)
///
/// Consumers should implement the CSS2 font-matching algorithm with the OpenDocument font
/// face extensions. They may implement variations of the CSS2 font-matching algorithm. They may
/// implement a font-matching based only on the font face declarations, that is, a font-matching that is
/// not applied to every character independently but only once for each font face declaration. (See
/// §15.5 CSS2)
///
/// Font face declarations support the font descriptor attributes and elements described in §20.8.3 of
/// SVG.
#[derive(Clone, Debug, Default, GetSize)]
pub struct FontFaceDecl {
    name: String,
    /// From where did we get this style.
    origin: StyleOrigin,
    /// All other attributes.
    attr: AttrMap2,
}

impl FontFaceDecl {
    /// New, empty.
    pub fn new_empty() -> Self {
        Self {
            name: "".to_string(),
            origin: Default::default(),
            attr: Default::default(),
        }
    }

    /// New, with a name.
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            origin: StyleOrigin::Content,
            attr: Default::default(),
        }
    }

    /// New with a name.
    #[deprecated]
    pub fn new_with_name<S: AsRef<str>>(name: S) -> Self {
        Self::new(name)
    }

    /// Set the name.
    pub fn set_name<V: AsRef<str>>(&mut self, name: V) {
        self.name = name.as_ref().to_string();
    }

    /// Returns the name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Origin of the style
    pub fn set_origin(&mut self, origin: StyleOrigin) {
        self.origin = origin;
    }

    /// Origin of the style
    pub fn origin(&self) -> StyleOrigin {
        self.origin
    }

    /// General attributes.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// General attributes.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    style_font_family_generic!(attr);
    style_font_pitch!(attr);
    svg_font_family!(attr);
    svg_font_stretch!(attr);
    svg_font_style!(attr);
    svg_font_variant!(attr);
    svg_font_weight!(attr);
}

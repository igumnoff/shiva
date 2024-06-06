//!
//! Defines tabstops for paragraph styles.
//!

use crate::attrmap2::AttrMap2;
use crate::color::Rgb;
use crate::style::color_string;
use crate::style::units::{Length, LineStyle, LineType, LineWidth, TabStopType};
use crate::style::TextStyleRef;
use get_size::GetSize;
use get_size_derive::GetSize;

/// The <style:tab-stops> element is a container for <style:tab-stop> elements.
/// If a style contains a <style:tab-stops> element, it overrides the entire <style:tab-stops>
/// element of the parent style such that no <style:tab-stop> children are inherited; otherwise,
/// the style inherits the entire <style:tab-stops> element as specified in section 16.2
/// <style:style>.
#[derive(Clone, Debug, Default, GetSize)]
pub struct TabStop {
    attr: AttrMap2,
}

impl TabStop {
    /// Empty.
    pub fn new() -> Self {
        Self {
            attr: Default::default(),
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

    style_char!(attr);
    style_leader_color!(attr);
    style_leader_style!(attr);
    style_leader_text!(attr);
    style_leader_text_style!(attr);
    style_leader_type!(attr);
    style_leader_width!(attr);
    style_position!(attr);
    style_type!(attr);
}

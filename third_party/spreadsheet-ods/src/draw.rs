//!
//! Draw.
//!

use crate::attrmap2::AttrMap2;
use crate::style::units::RelativeScale;
use crate::style::{GraphicStyleRef, ParagraphStyleRef};
use crate::text::{TextP, TextTag};
use crate::xlink::{XLinkActuate, XLinkShow, XLinkType};
use crate::{CellRef, Length, OdsError};
use base64::Engine;
use chrono::NaiveDateTime;
use get_size::GetSize;
use get_size_derive::GetSize;

/// The <office:annotation> element specifies an OpenDocument annotation. The annotation's
/// text is contained in <text:p> and <text:list> elements.
#[derive(Debug, Clone)]
pub struct Annotation {
    //
    name: String,
    //
    display: bool,
    //
    creator: Option<String>,
    date: Option<NaiveDateTime>,
    text: Vec<TextTag>,
    //
    attr: AttrMap2,
}

impl GetSize for Annotation {
    fn get_heap_size(&self) -> usize {
        self.name.get_heap_size()
            + self.creator.get_heap_size()
            + self.text.get_heap_size()
            + self.attr.get_heap_size()
    }
}

impl Annotation {
    /// New annotation.
    pub fn new_empty() -> Self {
        Self {
            name: Default::default(),
            display: false,
            creator: None,
            date: None,
            text: Default::default(),
            attr: Default::default(),
        }
    }

    /// New annotation.
    pub fn new<S: Into<String>>(annotation: S) -> Self {
        let mut r = Self {
            name: Default::default(),
            display: true,
            creator: None,
            date: None,
            text: Default::default(),
            attr: Default::default(),
        };
        r.push_text(TextP::new().text(annotation).into_xmltag());
        r
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Display
    pub fn display(&self) -> bool {
        self.display
    }

    /// Name
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Creator
    pub fn creator(&self) -> Option<&String> {
        self.creator.as_ref()
    }

    /// Creator
    pub fn set_creator<S: Into<String>>(&mut self, creator: Option<S>) {
        self.creator = creator.map(|v| v.into())
    }

    /// Date of the annotation.
    pub fn date(&self) -> Option<&NaiveDateTime> {
        self.date.as_ref()
    }

    /// Date of the annotation.
    pub fn set_date(&mut self, date: Option<NaiveDateTime>) {
        self.date = date;
    }

    /// Text.
    pub fn text(&self) -> &Vec<TextTag> {
        &self.text
    }

    /// Text.
    pub fn push_text(&mut self, text: TextTag) {
        self.text.push(text);
    }

    /// Text.
    pub fn push_text_str<S: Into<String>>(&mut self, text: S) {
        self.text.push(TextP::new().text(text).into_xmltag());
    }

    /// Text.
    pub fn set_text(&mut self, text: Vec<TextTag>) {
        self.text = text;
    }

    draw_caption_point_x!(attr);
    draw_caption_point_y!(attr);
    draw_class_names!(attr);
    draw_corner_radius!(attr);
    draw_id!(attr);
    draw_layer!(attr);
    draw_style_name!(attr);
    draw_text_style_name!(attr);
    draw_transform!(attr);
    draw_z_index!(attr);
    svg_height!(attr);
    svg_width!(attr);
    svg_x!(attr);
    svg_y!(attr);
    table_end_cell_address!(attr);
    table_end_x!(attr);
    table_end_y!(attr);
    table_table_background!(attr);
    xml_id!(attr);
}

// /// The <draw:rect> element represents a rectangular drawing shape.
// #[derive(Debug, Clone)]
// pub struct DrawRect {
//     ///
//     name: String,
//     ///
//     attr: AttrMap2,
// }
//
// impl DrawRect {
//     pub fn new_empty() -> Self {
//         Self {
//             name: "".to_string(),
//             attr: Default::default(),
//         }
//     }
//
//     pub fn new<S: Into<String>>(name: S) -> Self {
//         Self {
//             name: name.into(),
//             attr: Default::default(),
//         }
//     }
//
//     /// Allows access to all attributes of the style itself.
//     pub fn attrmap(&self) -> &AttrMap2 {
//         &self.attr
//     }
//
//     /// Allows access to all attributes of the style itself.
//     pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
//         &mut self.attr
//     }
//
//     /// Name
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//
//     /// Name
//     pub fn set_name<S: Into<String>>(&mut self, name: S) {
//         self.name = name.into();
//     }
//
//     draw_caption_id!(attr);
//     draw_class_names!(attr);
//     draw_corner_radius!(attr);
//     draw_id!(attr);
//     draw_layer!(attr);
//     draw_style_name!(attr);
//     draw_text_style_name!(attr);
//     draw_transform!(attr);
//     draw_z_index!(attr);
//     svg_height!(attr);
//     svg_width!(attr);
//     svg_rx!(attr);
//     svg_ry!(attr);
//     svg_x!(attr);
//     svg_y!(attr);
//     table_end_cell_address!(attr);
//     table_end_x!(attr);
//     table_end_y!(attr);
//     table_table_background!(attr);
//     xml_id!(attr);
// }

/// The <draw:frame> element represents a frame and serves as the container for elements that
/// may occur in a frame.
/// Frame formatting properties are stored in styles belonging to the graphic family.
#[derive(Debug, Clone, Default, GetSize)]
pub struct DrawFrame {
    /// The <svg:title> element specifies a name for a graphic object.
    title: Option<String>,
    /// The <svg:desc> element specifies a prose description of a graphic object that may be used to
    /// support accessibility. See appendix D.
    desc: Option<String>,
    ///
    attr: AttrMap2,
    ///
    content: Vec<DrawFrameContent>,
}

/// Draw-frame content data.
#[derive(Debug, Clone, GetSize)]
pub enum DrawFrameContent {
    /// Image
    Image(DrawImage),
}

impl DrawFrame {
    /// New.
    pub fn new() -> Self {
        Default::default()
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Desc
    pub fn desc(&self) -> Option<&String> {
        self.desc.as_ref()
    }

    /// Desc
    pub fn set_desc<S: Into<String>>(&mut self, desc: S) {
        self.desc = Some(desc.into())
    }

    /// Desc
    pub fn clear_desc(&mut self) {
        self.desc = None;
    }

    /// Title
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Name
    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = Some(title.into());
    }

    /// Name
    pub fn clear_title(&mut self) {
        self.title = None;
    }

    /// Frame content.
    pub fn set_content(&mut self, content: Vec<DrawFrameContent>) {
        self.content = content;
    }

    /// Frame content.
    pub fn push_content(&mut self, content: DrawFrameContent) {
        self.content.push(content);
    }

    /// Frame content.
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    /// Frame content.
    pub fn content_ref(&self) -> &Vec<DrawFrameContent> {
        &self.content
    }

    /// Frame content.
    pub fn content_mut(&mut self) -> &mut Vec<DrawFrameContent> {
        &mut self.content
    }

    draw_name!(attr);
    draw_caption_id!(attr);
    draw_class_names!(attr);
    draw_corner_radius!(attr);
    draw_copy_of!(attr);
    draw_id!(attr);
    draw_layer!(attr);
    draw_style_name!(attr);
    draw_text_style_name!(attr);
    draw_transform!(attr);
    draw_z_index!(attr);
    style_rel_height!(attr);
    style_rel_width!(attr);
    svg_height!(attr);
    svg_width!(attr);
    svg_x!(attr);
    svg_y!(attr);
    table_end_cell_address!(attr);
    table_end_x!(attr);
    table_end_y!(attr);
    table_table_background!(attr);
    xml_id!(attr);
}

/// The <draw:image> element represents an image. An image can be either:
/// • A link to an external resource
/// or
/// • Embedded in the document
/// The <draw:image> element may have text content. Text content is displayed in addition to the
/// image data.
/// Note: While the image data may have an arbitrary format, vector graphics should
/// be stored in the SVG format and bitmap graphics in the PNG format.
#[derive(Debug, Clone, Default, GetSize)]
pub struct DrawImage {
    attr: AttrMap2,
    binary_data: Option<String>,
    text: Vec<TextTag>,
}

impl DrawImage {
    /// New.
    pub fn new() -> Self {
        Default::default()
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Allows access to all attributes of the style itself.
    pub fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Image binary data.
    pub fn get_binary_base64(&self) -> Option<&String> {
        self.binary_data.as_ref()
    }

    /// Image binary data.
    pub fn set_binary_base64(&mut self, binary: String) {
        self.binary_data = Some(binary);
    }

    /// Image binary data.
    pub fn get_binary(&self) -> Result<Vec<u8>, OdsError> {
        let ng = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::general_purpose::NO_PAD,
        );

        if let Some(binary_data) = &self.binary_data {
            Ok(ng.decode(binary_data)?)
        } else {
            Ok(Default::default())
        }
    }

    /// Image binary data.
    /// Note: While the image data may have an arbitrary format, vector graphics should
    /// be stored in the SVG format and bitmap graphics in the PNG format.
    pub fn set_binary(&mut self, binary: &[u8]) {
        let ng = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::general_purpose::NO_PAD,
        );

        self.binary_data = Some(ng.encode(binary));
    }

    /// Image binary data.
    pub fn clear_binary(&mut self) {
        self.binary_data = None;
    }

    /// Text
    pub fn get_text(&self) -> &Vec<TextTag> {
        &self.text
    }

    /// Text
    pub fn push_text(&mut self, text: TextTag) {
        self.text.push(text);
    }

    /// Text
    pub fn push_text_str<S: Into<String>>(&mut self, text: S) {
        self.text.push(TextP::new().text(text).into_xmltag());
    }

    /// Text
    pub fn set_text(&mut self, text: Vec<TextTag>) {
        self.text = text;
    }

    draw_filter_name!(attr);
    draw_mime_type!(attr);
    xlink_actuate!(attr);
    xlink_href!(attr);
    xlink_show!(attr);
    xlink_type!(attr);
    xml_id!(attr);
}

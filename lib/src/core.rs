use bytes::Bytes;
#[cfg(feature = "json")]
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct Document {
    pub elements: Vec<Element>,
    pub page_width: f32,
    pub page_height: f32,
    pub left_page_indent: f32,
    pub right_page_indent: f32,
    pub top_page_indent: f32,
    pub bottom_page_indent: f32,
    pub page_header: Vec<Element>,
    pub page_footer: Vec<Element>,
}

impl Document {
    pub fn new(elements: Vec<Element>) -> Document {
        Document {
            elements: elements,
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: vec![],
            page_footer: vec![],
        }
    }
}

pub trait TransformerTrait {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document>;
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)>;
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parser error")]
    Common,
}
#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Generator error")]
    Common,
}
#[derive(Debug, Clone)]
pub enum Element {
    Text {
        text: String,
        size: u8,
    },
    Header {
        level: u8,
        text: String,
    },
    Paragraph {
        elements: Vec<Element>,
    },
    Table {
        headers: Vec<TableHeader>,
        rows: Vec<TableRow>,
    },
    List {
        elements: Vec<ListItem>,
        numbered: bool,
    },
    Image {
        bytes: Bytes,
        title: String,
        alt: String,
        image_type: ImageType,
    },
    Hyperlink {
        title: String,
        url: String,
        alt: String,
    },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct ListItem {
    pub element: Element,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct TableHeader {
    pub element: Element,
    pub width: f32,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct TableCell {
    pub element: Element,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub enum ImageType {
    Png,
    Jpeg,
}

use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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

#[derive(Error, Debug, Deserialize)]
pub enum DocumentType {
    Html,
    Markdown,
    Text,
    Pdf,
    Json,
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentType::Html => write!(f, "HTML"),
            DocumentType::Markdown => write!(f, "MarkDown"),
            DocumentType::Text => write!(f, "Text"),
            DocumentType::Pdf => write!(f, "PDF"),
            DocumentType::Json => write!(f, "JSON"),
        }
    }
}

impl Document {
    pub fn new(elements: Vec<Element>) -> Document {
        Document {
            elements,
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
    fn parse(document: &Bytes) -> anyhow::Result<Document>;
    fn generate(document: &Document) -> anyhow::Result<Bytes>;
}



pub trait TransformerWithImageLoaderSaverTrait {
    fn parse_with_loader<F>(document: &Bytes,  image_loader: F) -> anyhow::Result<Document>
        where F: Fn(&str) -> anyhow::Result<Bytes>;
    fn generate_with_saver<F>(document: &Document,  image_saver: F) -> anyhow::Result<Bytes>
        where F: Fn(&Bytes, &str) -> anyhow::Result<()>;
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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
        #[cfg_attr(feature = "json", serde(skip))]
        bytes: Bytes,
        title: String,
        alt: String,
        image_type: ImageType,
    },
    Hyperlink {
        title: String,
        url: String,
        alt: String,
        size: u8,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ListItem {
    pub element: Element,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct TableHeader {
    pub element: Element,
    pub width: f32,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct TableCell {
    pub element: Element,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ImageType {
    Png,
    Jpeg,
}


pub fn disk_image_loader(path: &str) -> impl Fn(&str) -> anyhow::Result<Bytes>  {
    let path = path.to_string();
    let image_loader = move |image: &str| -> anyhow::Result<Bytes> {
        let image_path = format!("{}/{}", path, image);
        let bytes = std::fs::read(image_path)?;
        Ok(Bytes::from(bytes))
    };
    image_loader
}


pub fn disk_image_saver(path: &str) -> impl Fn(&Bytes, &str) -> anyhow::Result<()>  {
    let path = path.to_string();
    let image_saver = move |bytes: &Bytes, image: &str| -> anyhow::Result<()> {
        let image_path = format!("{}/{}", path, image);
        std::fs::write(image_path, bytes)?;
        Ok(())
    };
    image_saver
}
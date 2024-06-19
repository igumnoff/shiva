use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
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
    fn parse_with_loader<F>(document: &Bytes, image_loader: F) -> anyhow::Result<Document>
    where
        F: Fn(&str) -> anyhow::Result<Bytes>;
    fn generate_with_saver<F>(document: &Document, image_saver: F) -> anyhow::Result<Bytes>
    where
        F: Fn(&Bytes, &str) -> anyhow::Result<()>;
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
    Image(ImageData),
    Hyperlink {
        title: String,
        url: String,
        alt: String,
        size: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ImageData {
    #[cfg_attr(feature = "json", serde(skip))]
    bytes: Bytes,
    title: String,
    alt: String,
    image_type: ImageType,
    width: i64,
    height: i64,
}

impl ImageData {
    pub fn new(
        bytes: Bytes,
        title: String,
        alt: String,
        image_type: ImageType,
        width: i64,
        height: i64,
    ) -> ImageData {
        ImageData {
            bytes,
            title,
            alt,
            image_type,
            width,
            height,
        }
    }

    pub fn set_image_type(&mut self, image_type_str: &str) {
        self.image_type = image_type_str.into();
    }

    pub fn set_image_bytes(&mut self, bytes: Bytes) {
        self.bytes = bytes;
    }

    pub fn set_image_alt(&mut self, alt: &str) {
        self.alt = alt.to_string();
    }

    pub fn set_image_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn image_type(&self) -> &ImageType {
        &self.image_type
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }
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

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ImageType {
    #[default]
    Png,
    Jpeg,
}

impl ImageType {
    pub fn to_extension(&self) -> &str {
        match self {
            ImageType::Png => ".png",
            ImageType::Jpeg => ".jpeg",
        }
    }
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageType::Png => write!(f, "png"),
            ImageType::Jpeg => write!(f, "jpeg"),
        }
    }
}

impl From<&str> for ImageType {
    fn from(s: &str) -> ImageType {
        match s.to_lowercase().as_str() {
            "png" => ImageType::Png,
            "jpeg" => ImageType::Jpeg,
            _ => {
                println!("Error: Unsupported image type");
                ImageType::default() // Or handle it as needed
            }
        }
    }
}

pub fn disk_image_loader(path: &str) -> impl Fn(&str) -> anyhow::Result<Bytes> {
    let path = path.to_string();
    let image_loader = move |image: &str| -> anyhow::Result<Bytes> {
        let image_path = format!("{}/{}", path, image);
        println!("Loading image: {}", image_path);
        let bytes = std::fs::read(image_path)?;
        Ok(Bytes::from(bytes))
    };
    image_loader
}

pub fn disk_image_saver(path: &str) -> impl Fn(&Bytes, &str) -> anyhow::Result<()> {
    let path = path.to_string();
    let image_saver = move |bytes: &Bytes, image: &str| -> anyhow::Result<()> {
        let image_path = format!("{}/{}", path, image);
        std::fs::write(image_path, bytes)?;
        Ok(())
    };
    image_saver
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_type() -> anyhow::Result<()> {
        let image_type = ImageType::Png;
        assert_eq!(image_type.to_string(), "png");
        Ok(())
    }
}

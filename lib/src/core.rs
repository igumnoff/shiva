use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;
use strum::{VariantArray, EnumString, Display, IntoStaticStr, EnumCount};

use crate::{csv, docx, html, json, markdown, ods, pdf, rtf, text, xls, xlsx, xml};

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

    pub fn parse(input_bytes: &Bytes, document_type: DocumentType) -> anyhow::Result<Document> {
        let document = match document_type {
            DocumentType::Markdown => markdown::Transformer::parse(input_bytes)?,
            DocumentType::HTML => html::Transformer::parse(input_bytes)?,
            DocumentType::Text => text::Transformer::parse(input_bytes)?,
            DocumentType::PDF => pdf::Transformer::parse(input_bytes)?,
            DocumentType::Json => json::Transformer::parse(input_bytes)?,
            DocumentType::CSV => csv::Transformer::parse(input_bytes)?,
            DocumentType::RTF => rtf::Transformer::parse(input_bytes)?,
            DocumentType::DOCX => docx::Transformer::parse(input_bytes)?,
            DocumentType::XML => xml::Transformer::parse(input_bytes)?,
            DocumentType::XLS => xls::Transformer::parse(input_bytes)?,
            DocumentType::XLSX => xlsx::Transformer::parse(input_bytes)?,
            DocumentType::ODS => ods::Transformer::parse(input_bytes)?,
        };
        Ok(document)
    }

    pub fn generate(&self, document_type: DocumentType) -> anyhow::Result<Bytes> {
        let output = match document_type {
            DocumentType::Markdown => markdown::Transformer::generate(self)?,
            DocumentType::HTML => html::Transformer::generate(self)?,
            DocumentType::Text => text::Transformer::generate(self)?,
            DocumentType::PDF => pdf::Transformer::generate(self)?,
            DocumentType::Json => json::Transformer::generate(self)?,
            DocumentType::CSV => csv::Transformer::generate(self)?,
            DocumentType::RTF => rtf::Transformer::generate(self)?,
            DocumentType::DOCX => docx::Transformer::generate(self)?,
            DocumentType::XML => xml::Transformer::generate(self)?,
            DocumentType::XLS => xls::Transformer::generate(self)?,
            DocumentType::XLSX => xlsx::Transformer::generate(self)?,
            DocumentType::ODS => ods::Transformer::generate(self)?,
        };
        Ok(output)
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
    Image(ImageData),
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

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ImageData {
    #[cfg_attr(feature = "json", serde(skip))]
    bytes: Bytes,
    title: String,
    alt: String,
    image_type: ImageType,
    align: ImageAlignment,
    size: ImageDimension
}

/**
 * ImageData methods
 */
impl ImageData {
    pub fn new(
        bytes: Bytes,
        title: String,
        alt: String,
        src_or_type: String,
        alignment: String,
        size: ImageDimension,
    ) -> ImageData {
        let mut image_data = ImageData {
            bytes,
            title,
            alt,
            image_type: ImageType::default(),
            align: ImageAlignment::default(),
            size
        };
        image_data.set_image_type(&src_or_type);
        image_data.set_image_alignment(&alignment);
        image_data
    }

    pub fn set_image_type(&mut self, image_type_str: &str) {
        let image_type_str = image_type_str
            .split('.')
            .last()
            .unwrap_or("")
            .trim()
            .to_lowercase();

        if image_type_str.trim().is_empty() {
            self.image_type = ImageType::default();
            return;
        }

        match ImageType::from_str(&image_type_str) {
            Ok(image_type) => self.image_type = image_type,
            Err(_) => panic!("Invalid image type: {}", image_type_str),
        }
    }

    pub fn set_image_alignment(&mut self, alignment_str: &str) {
        if alignment_str.trim().is_empty() {
            self.align = ImageAlignment::default();
            return;
        }
        match ImageAlignment::from_str(alignment_str) {
            Ok(alignment) => self.align = alignment,
            Err(_) => panic!("Invalid image alignment: {}", alignment_str),
        }
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

    pub fn set_image_size(&mut self, size: ImageDimension) {
        self.size = size;
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

    pub fn align(&self) -> &ImageAlignment {
        &self.align
    }

    pub fn size(&self) -> &ImageDimension{
        &self.size
    }

}

#[derive(Debug, Clone, PartialEq, Default, Display, EnumString, VariantArray)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[strum(serialize_all = "lowercase")]
pub enum ImageType {
    #[default]
    Png,
    Jpeg,
    Gif,
    SVG
}

impl ImageType {
    pub fn to_extension(&self) -> &str {
        match self {
            ImageType::Png => ".png",
            ImageType::Jpeg => ".jpeg",
            ImageType::Gif => ".gif",
            ImageType::SVG => ".svg",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, EnumString, Display, VariantArray)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[strum(serialize_all = "lowercase")]
pub enum ImageAlignment {
    Left,
    Center,
    Right,
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ImageDimension {
    pub width: Option<String>,
    pub height: Option<String>,
}


pub fn disk_image_loader(path: &str) -> impl Fn(&str) -> anyhow::Result<Bytes>  {
    let path = path.to_string();
    let image_loader = move |image: &str| -> anyhow::Result<Bytes> {
        let image_path = format!("{}/{}", path, image);
        println!("Loading image: {}", image_path);
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


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Eq, EnumString, Display, VariantArray, IntoStaticStr, EnumCount)]
#[strum(serialize_all = "lowercase")]
pub enum DocumentType {
    HTML = 0,
    Markdown = 1,
    Text = 2,
    PDF = 3,
    Json = 4,
    CSV = 5,
    RTF = 6,
    DOCX = 7,
    XML = 8,
    XLS = 9,
    XLSX = 10,
    ODS = 11,
}

impl DocumentType {
    pub fn variants() -> &'static [DocumentType] {
        DocumentType::VARIANTS
    }

    pub fn variants_as_str() -> Vec<&'static str> {
        DocumentType::VARIANTS.iter().map(|v| v.into()).collect()
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    const VARIANTS: &[DocumentType] = &[
        DocumentType::HTML, DocumentType::Markdown, DocumentType::Text,
        DocumentType::PDF, DocumentType::Json, DocumentType::CSV,
        DocumentType::RTF, DocumentType::DOCX, DocumentType::XML,
        DocumentType::XLS, DocumentType::XLSX, DocumentType::ODS];

    #[test]
    fn test_document_type_count() {
        assert_eq!(VARIANTS.len(), DocumentType::COUNT);
    }

    #[test]
    fn test_document_type_as_list() {
        assert_eq!(DocumentType::VARIANTS, VARIANTS);
        assert!(DocumentType::VARIANTS.contains(&DocumentType::HTML));
        assert!(DocumentType::VARIANTS.contains(&DocumentType::RTF));
    }

    #[test]
    fn test_serialize_all_lower_case() {
        assert_eq!("csv", DocumentType::CSV.to_string());
        assert_eq!(DocumentType::PDF, DocumentType::from_str("pdf").unwrap());
        assert_eq!("json", <&'static str>::from(DocumentType::Json));
    }

    #[test]
    fn test_variants_as_str() {
        let variants = DocumentType::variants_as_str();
        assert_eq!(variants.len(), DocumentType::COUNT);
        assert!(variants.contains(&"html"));
        assert!(variants.contains(&"docx"));
    }

    #[test]
    fn test_as_repr() {
        assert_eq!(DocumentType::HTML as u8, 0);
        assert_eq!(DocumentType::XLSX as u8, 10);
    }

    #[test]
    fn test_image_type_from_str() {
        assert_eq!(ImageType::from_str("png").unwrap(), ImageType::Png);
        assert_eq!(ImageType::from_str("jpeg").unwrap(), ImageType::Jpeg);
        assert_eq!(ImageType::from_str("gif").unwrap(), ImageType::Gif);
    }

    #[test]
    fn test_image_new() {
        let bytes = Bytes::from("image".as_bytes());
        let image = ImageData::new(bytes.clone(), "title".to_string(), "alt".to_string(), "/name/image.png".to_string(), "center".to_string(), ImageDimension { width: Some("50%".to_string()), height: Some("200".to_string())});
        assert_eq!(image.bytes(), &bytes);
        assert_eq!(image.title(), "title");
        assert_eq!(image.alt(), "alt");
        assert_eq!(image.image_type(), &ImageType::Png);
    }

    #[test]
    fn test_image_type_extension() {
        assert_eq!(ImageType::Png.to_extension(), ".png");
        assert_eq!(ImageType::Jpeg.to_extension(), ".jpeg");
    }

    #[test]
    fn test_image_type_defaut() {
        assert_eq!(ImageType::Png, ImageType::default());
    }

    #[test]
    fn test_image_type_display() {
        assert_eq!("png", ImageType::Png.to_string());
        assert_eq!("jpeg", ImageType::Jpeg.to_string());
    }

    #[test]
    fn test_image_alignment() {
        assert_eq!(ImageAlignment::Left, ImageAlignment::from_str("left").unwrap());
        assert_eq!(ImageAlignment::Center, ImageAlignment::from_str("center").unwrap());
        assert_eq!(ImageAlignment::Right, ImageAlignment::from_str("right").unwrap());
    }



}
use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::Debug;
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

}
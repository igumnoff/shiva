use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::Debug;
use thiserror::Error;
use strum::{VariantArray, EnumString, Display, IntoStaticStr, EnumCount};

#[cfg(feature = "csv")]
use crate::csv;
#[cfg(feature = "docx")]
use crate::docx;
#[cfg(feature = "html")]
use crate::html;
#[cfg(feature = "json")]
use crate::json;
#[cfg(feature = "markdown")]
use crate::markdown;
#[cfg(feature = "ods")]
use crate::ods;
#[cfg(feature = "pdf")]
use crate::pdf;
#[cfg(feature = "rtf")]
use crate::rtf;
#[cfg(feature = "text")]
use crate::text;
#[cfg(feature = "xls")]
use crate::xls;
#[cfg(feature = "xlsx")]
use crate::xlsx;
#[cfg(feature = "xml")]
use crate::xml;

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
            #[cfg(feature = "markdown")]
            DocumentType::Markdown => markdown::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "markdown"))]
            DocumentType::Markdown => return Err(anyhow::anyhow!("Markdown feature is not enabled")),
            #[cfg(feature = "html")]
            DocumentType::HTML => html::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "html"))]
            DocumentType::HTML => return Err(anyhow::anyhow!("HTML feature is not enabled")),
            #[cfg(feature = "text")]
            DocumentType::Text => text::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "text"))]
            DocumentType::Text => return Err(anyhow::anyhow!("Text feature is not enabled")),
            #[cfg(feature = "pdf")]
            DocumentType::PDF => pdf::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "pdf"))]
            DocumentType::PDF => return Err(anyhow::anyhow!("PDF feature is not enabled")),
            #[cfg(feature = "json")]
            DocumentType::Json => json::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "json"))]
            DocumentType::Json => return Err(anyhow::anyhow!("Json feature is not enabled")),
            #[cfg(feature = "csv")]
            DocumentType::CSV => csv::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "csv"))]
            DocumentType::CSV => return Err(anyhow::anyhow!("CSV feature is not enabled")),
            #[cfg(feature = "rtf")]
            DocumentType::RTF => rtf::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "rtf"))]
            DocumentType::RTF => return Err(anyhow::anyhow!("RTF feature is not enabled")),
            #[cfg(feature = "docx")]
            DocumentType::DOCX => docx::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "docx"))]
            DocumentType::DOCX => return Err(anyhow::anyhow!("DOCX feature is not enabled")),
            #[cfg(feature = "xml")]
            DocumentType::XML => xml::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "xml"))]
            DocumentType::XML => return Err(anyhow::anyhow!("XML feature is not enabled")),
            #[cfg(feature = "xls")]
            DocumentType::XLS => xls::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "xls"))]
            DocumentType::XLS => return Err(anyhow::anyhow!("XLS feature is not enabled")),
            #[cfg(feature = "xlsx")]
            DocumentType::XLSX => xlsx::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "xlsx"))]
            DocumentType::XLSX => return Err(anyhow::anyhow!("XLSX feature is not enabled")),
            #[cfg(feature = "ods")]
            DocumentType::ODS => ods::Transformer::parse(input_bytes)?,
            #[cfg(not(feature = "ods"))]
            DocumentType::ODS => return Err(anyhow::anyhow!("ODS feature is not enabled")),
        };
        Ok(document)
    }

    pub fn generate(&self, document_type: DocumentType) -> anyhow::Result<Bytes> {
        let output = match document_type {
            #[cfg(feature = "markdown")]
            DocumentType::Markdown => markdown::Transformer::generate(self)?,
            #[cfg(not(feature = "markdown"))]
            DocumentType::Markdown => return Err(anyhow::anyhow!("Markdown feature is not enabled")),
            #[cfg(feature = "html")]
            DocumentType::HTML => html::Transformer::generate(self)?,
            #[cfg(not(feature = "html"))]
            DocumentType::HTML => return Err(anyhow::anyhow!("HTML feature is not enabled")),
            #[cfg(feature = "text")]
            DocumentType::Text => text::Transformer::generate(self)?,
            #[cfg(not(feature = "text"))]
            DocumentType::Text => return Err(anyhow::anyhow!("Text feature is not enabled")),
            #[cfg(feature = "pdf")]
            DocumentType::PDF => pdf::Transformer::generate(self)?,
            #[cfg(not(feature = "pdf"))]
            DocumentType::PDF => return Err(anyhow::anyhow!("PDF feature is not enabled")),
            #[cfg(feature = "json")]
            DocumentType::Json => json::Transformer::generate(self)?,
            #[cfg(not(feature = "json"))]
            DocumentType::Json => return Err(anyhow::anyhow!("Json feature is not enabled")),
            #[cfg(feature = "csv")]
            DocumentType::CSV => csv::Transformer::generate(self)?,
            #[cfg(not(feature = "csv"))]
            DocumentType::CSV => return Err(anyhow::anyhow!("CSV feature is not enabled")),
            #[cfg(feature = "rtf")]
            DocumentType::RTF => rtf::Transformer::generate(self)?,
            #[cfg(not(feature = "rtf"))]
            DocumentType::RTF => return Err(anyhow::anyhow!("RTF feature is not enabled")),
            #[cfg(feature = "docx")]
            DocumentType::DOCX => docx::Transformer::generate(self)?,
            #[cfg(not(feature = "docx"))]
            DocumentType::DOCX => return Err(anyhow::anyhow!("DOCX feature is not enabled")),
            #[cfg(feature = "xml")]
            DocumentType::XML => xml::Transformer::generate(self)?,
            #[cfg(not(feature = "xml"))]
            DocumentType::XML => return Err(anyhow::anyhow!("XML feature is not enabled")),
            #[cfg(feature = "xls")]
            DocumentType::XLS => xls::Transformer::generate(self)?,
            #[cfg(not(feature = "xls"))]
            DocumentType::XLS => return Err(anyhow::anyhow!("XLS feature is not enabled")),
            #[cfg(feature = "xlsx")]
            DocumentType::XLSX => xlsx::Transformer::generate(self)?,
            #[cfg(not(feature = "xlsx"))]
            DocumentType::XLSX => return Err(anyhow::anyhow!("XLSX feature is not enabled")),
            #[cfg(feature = "ods")]
            DocumentType::ODS => ods::Transformer::generate(self)?,
            #[cfg(not(feature = "ods"))]
            DocumentType::ODS => return Err(anyhow::anyhow!("ODS feature is not enabled")),
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

// Opinion (JohnScience): The variants of this enum should be enabled/disabled based on the features.
// This way, the user will get compile-time errors if they work. However, this would be a breaking change.

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
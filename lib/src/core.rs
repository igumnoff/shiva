use bytes::Bytes;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use std::fmt::Debug;
use std::str::FromStr;
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

// Opinion (JohnScience): The variants of this enum should be enabled/disabled based on the features.
// This way, the user will get compile-time errors if they work. However, this would be a breaking change.

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, VariantArray, IntoStaticStr, EnumCount)]
#[cfg_attr(feature = "json", derive(Serialize))]
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
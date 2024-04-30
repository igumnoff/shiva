use crate::core::{Document, TransformerTrait};
use anyhow::Ok;
use bytes::Bytes;
use serde_xml_rs::{from_str, to_string};
use std::collections::HashMap;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let doc_string = String::from_utf8(document.to_vec())?;
        let doc: Document = from_str(&doc_string)?;
        return Ok(doc);
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let result = to_string(document)?;
        Ok((Bytes::from(result), HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::TransformerTrait;
    use crate::xml;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
        // println!("{:?}", document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        images.insert("test/data/picture.png".to_string(), image_bytes);
        let parsed = xml::Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = xml::Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);
        Ok(())
    }
}

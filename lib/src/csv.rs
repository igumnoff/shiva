use crate::core::{Document, TransformerTrait};
use bytes::Bytes;
use std::collections::HashMap;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(_document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        todo!()
    }
    fn generate(_document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::core::TransformerTrait;
    use crate::json;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"StudentID,Name,Math,Science,English
1,John Doe,88,92,85
2,Jane Smith,94,95,91
3,Emily Johnson,78,88,83"#;
        let images = HashMap::new();
        let parsed = json::Transformer::parse(&document.as_bytes().into(), &images);
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        Ok(())
    }
}
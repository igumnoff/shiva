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
    use bytes::Bytes;
    use crate::core::TransformerTrait;
    use crate::{docx};

    #[test]
    fn test() -> anyhow::Result<()> {
        // read from document.docx file from disk
        let document = std::fs::read("test/data/document.docx")?;
        let bytes = Bytes::from(document);
        let images = HashMap::new();
        let parsed = docx::Transformer::parse(&bytes, &images);
        assert!(parsed.is_ok());
        Ok(())
    }
}

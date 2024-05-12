use crate::core::{Document, TransformerTrait};
use untex::latex::parse;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(_document: &bytes::Bytes, _images: &std::collections::HashMap<String, bytes::Bytes>) -> anyhow::Result<Document> {
        todo!()
    }
    fn generate(_document: &Document) -> anyhow::Result<(bytes::Bytes, std::collections::HashMap<String, bytes::Bytes>)> {
        todo!()
    }
}

#[cfg test]
mod tests {
    use std::collections::HashMap;
    use bytes::Bytes;
    use crate::core::TransformerTrait;
    use crate::{docx};

    #[test]
    fn test() -> anyhow::Result<()> {
        // read from document.docx file from disk
        let document = std::fs::read("test/data/document.latex")?;
        let bytes = Bytes::from(document);
        let images = HashMap::new();
        let parsed = docx::Transformer::parse(&bytes, &images);
        assert!(parsed.is_ok());
        Ok(())
    }
}
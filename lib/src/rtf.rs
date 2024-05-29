use crate::core::Element;
use crate::core::{
    Document,
    Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text},
    TableCell, TableHeader, TableRow, TransformerTrait,
};

use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;



pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(
        document: &bytes::Bytes,
        imagesizes: &std::collections::HashMap<String, bytes::Bytes>,
    ) -> anyhow::Result<Document> {
        let data_str = std::str::from_utf8(document).unwrap();
        let tokens = Lexer::scan(&data_str).unwrap();
    
        // keeping the document in a box since it might contain huge data and also
        // for easy manipulation
        let mut document: Document = Document::new(vec![]);
        // initializing header levels
        let mut level = 1;
        for styleblock in Parser::new(tokens).parse().unwrap().body.as_slice() {
            if styleblock.painter.font_size >= 30 && styleblock.painter.bold == true {
                document.elements.push(Header {
                    level: level,
                    text: styleblock.text.to_owned(),
                });
                level += 1
            } else {
                {
                    document.elements.push(Paragraph {
                        elements: vec![Text {
                            text: styleblock.text.to_owned(),
                            size: styleblock.painter.font_size as u8,
                        }],
                    })
                }
            }   
    }
    Ok(document)
    }
    fn generate(
        document: &Document,
    ) -> anyhow::Result<(
        bytes::Bytes,
        std::collections::HashMap<String, bytes::Bytes>,
    )> {
        todo!();
    }
}

#[cfg(test)]

mod test {
    use bytes::Bytes;
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.rtf")?;
        let documents_bytes = Bytes::from(document);
        let parsed = Transformer::parse(&documents_bytes, &HashMap::new())?;
        let generated_result = crate::pdf::Transformer::generate(&parsed)?;
        std::fs::write("test/data/document_from_rtf.pdf", generated_result.0)?;

        Ok(())
    }
}

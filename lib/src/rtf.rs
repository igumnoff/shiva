use std::collections::HashMap;
use crate::core::{Document, Element::{Header, Paragraph, Text}, TransformerTrait};

use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;




pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(
        document: &bytes::Bytes,
        _images: &std::collections::HashMap<String, bytes::Bytes>,
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
        let mut rtf_content = String::new();

        rtf_content.push_str("{\\rtf1\\ansi\\deff0"); //the standard title of an RTF document, which indicates that it is an RTF document using ANSI characters and the default font
        for element in &document.elements {
            match element {

                Header { level, text} => {
                    let header_size = 30 + (*level as i32 * 2);

                    //formatting the string RTF
                    rtf_content.push_str(&format!(
                        "{{\\fs{}\\b {} \\b0}}\\par ",
                        header_size * 2,
                        text
                    ));
                }

                Paragraph { elements } => {
                    for elem in elements {
                        if let Text { text, size } = elem {
                            rtf_content.push_str(&format!(
                                "{{\\fs{} {}}}",
                                *size as i32 * 2,
                                text
                            ));
                        }
                    }
                    rtf_content.push_str("\\par ");
                }
                _ => {
                    eprintln!("Unknown element");
                }
            }
        }

        rtf_content.push_str("}");

        Ok((bytes::Bytes::from(rtf_content.into_bytes()), HashMap::new()))

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

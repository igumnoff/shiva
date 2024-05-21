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
        Ok(doc)
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let result = to_string(document)?;
        Ok((Bytes::from(result), HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Document, Element, ImageType, TransformerTrait};
    use crate::xml;
    #[test]
    fn test() -> anyhow::Result<()> {
        let elements = vec![
            Element::Text {
                text: "Hello, world!".to_string(),
                size: 12,
            },
            Element::Header {
                level: 1,
                text: "123".to_string(),
            },
            Element::Image {
                bytes: Default::default(),
                title: "".to_string(),
                alt: "".to_string(),
                image_type: ImageType::Png,
            },
            Element::Paragraph {
                elements: vec![Element::Text {
                    text: "Hello, world!".to_string(),
                    size: 12,
                }],
            },
            // Element::Table {
            //     headers: vec![
            //         TableHeader {
            //             element: Element::Text {
            //                 text: "Syntax".to_string(),
            //                 size: 8,
            //             },
            //             width: 10.0,
            //         },
            //         TableHeader {
            //             element: Element::Text {
            //                 text: "Description".to_string(),
            //                 size: 8,
            //             },
            //             width: 10.0,
            //         },
            //     ],
            //     rows: vec![
            //         TableRow {
            //             cells: vec![
            //                 TableCell {
            //                     element: Element::Text {
            //                         text: "Header".to_string(),
            //                         size: 8,
            //                     },
            //                 },
            //                 TableCell {
            //                     element: Element::Text {
            //                         text: "Title".to_string(),
            //                         size: 8,
            //                     },
            //                 },
            //             ],
            //         },
            //         TableRow {
            //             cells: vec![
            //                 TableCell {
            //                     element: Element::Text {
            //                         text: "Paragraph".to_string(),
            //                         size: 8,
            //                     },
            //                 },
            //                 TableCell {
            //                     element: Element::Text {
            //                         text: "Text".to_string(),
            //                         size: 8,
            //                     },
            //                 },
            //             ],
            //         },
            //     ],
            // },
            // Element::List {
            //     elements: vec![
            //         crate::core::ListItem {
            //             element: Element::Text {
            //                 text: "List item 1".to_string(),
            //                 size: 8,
            //             },
            //         },
            //     ],
            //     numbered: false,
            // }
        ];
        let doc = Document::new(elements);
        let generated_result = xml::Transformer::generate(&doc);
        assert!(generated_result.is_ok());
        println!("{:?}", generated_result);
        println!("==========================");
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);
        Ok(())
    }
}

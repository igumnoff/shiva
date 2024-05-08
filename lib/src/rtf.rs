use std::env::var;

use crate::core::Element;
use crate::core::{
    Document,
    Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text},
    TableCell, TableHeader, TableRow, TransformerTrait,
};
use anyhow::Ok;
use regex::bytes::CaptureMatches;
use regex::Regex;
use ttf_parser::head;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(
        document: &bytes::Bytes,
        imagesizes: &std::collections::HashMap<String, bytes::Bytes>,
    ) -> anyhow::Result<Document> {
        let document_str = std::str::from_utf8(document)?;
        // let mut elements: Vec<Element> = Vec::new();
        let mut headers: Vec<Element> = Vec::new();
        let mut paragraphs: Vec<Element> = Vec::new();
        let mut hyperlinks: Vec<Element> = Vec::new();
        let mut images: Vec<Element> = Vec::new();
        let mut lists: Vec<String> = Vec::new();

        let mut table_header: Vec<TableHeader> = Vec::new();
        let mut table: Vec<TableRow> = Vec::new();
        let mut table_cell: Vec<TableCell> = Vec::new();

        let header_re = Regex::new(r#"\\header\s+(.*?)\}"#).unwrap();
        let table_re = Regex::new(r#"\\trowd(.*?)\\row"#).unwrap();
        let paragraph_re = Regex::new(r#"\\pard(.*?)\\par"#).unwrap();
        let image_re = Regex::new(r#"\\pict(.*?)}"#).unwrap();
        let hyperlink_re = Regex::new(r#"\\field\\fldedit\\*\s*\\fldinst\s*HYPERLINK\s*"(.+?)("\s*)(.+?)("\\fldrslt\s*")(.+?)"#).unwrap();
        let list_re = Regex::new(r#"\pntext(.*?)}"#).unwrap();
        let footer_re = Regex::new(r#"\\footer\s+(.*?)\}"#).unwrap();

        let pard_match = paragraph_re.captures_iter(document_str);
        let header_match = header_re.captures_iter(document_str);
        let tab_match: regex::CaptureMatches = table_re.captures_iter(document_str);
        let img_match = image_re.captures_iter(document_str);
        let hyper_match = hyperlink_re.captures_iter(document_str);
        let list_match = list_re.captures_iter(document_str);
        let foot_match = footer_re.captures_iter(document_str);
        fn getcapture<'a>(m: regex::CaptureMatches<'a, 'a>) -> Vec<&'a str> {
            m.map(|capture| capture.get(0).unwrap().as_str()).collect()
        }

        let [pard_vec, tab_vec, img_vec, hyper_vec, list_vec, foot_vec, header_vec] = [
            pard_match,
            tab_match,
            img_match,
            hyper_match,
            list_match,
            foot_match,
            header_match,
        ]
        .map(getcapture);

        for (_, text) in pard_vec.into_iter().enumerate() {
            paragraphs.push(Paragraph {
                elements: vec![Text {
                    text: text.to_string(),
                    size: 8,
                }],
            })
        }
        // since we want to collect just the first row as ow table header
        for (index, text) in tab_vec.into_iter().enumerate() {
            if let 0 = index {
                table_header.push(TableHeader {
                    element: Text {
                        text: text.to_string(),
                        size: 8,
                    },
                    width: 16.9,
                });
            } else {
                table_cell.push(TableCell {
                    element: Text {
                        text: text.to_string(),
                        size: 8,
                    },
                })
            }
            table.push(TableRow {
                cells: table_cell.to_owned(),
            })
        }

        for capture in hyperlink_re.captures_iter(document_str) {
            let url = capture.get(1).map_or("", |m| m.as_str().trim());
            let title = capture.get(0).map_or("", |m| m.as_str().trim());
            let alt_text = capture.get(2).map_or("", |m| m.as_str().trim());

            hyperlinks.push(Hyperlink {
                title: title.to_string(),
                url: url.to_string(),
                alt: alt_text.to_string(),
                size: 8,
            })
        }
        for (_, headnote) in header_vec.into_iter().enumerate() {
            headers.push(Header {
                level: 8,
                text: headnote.to_string(),
            })
        }
        // for (index, list) in list_vec.into_iter().enumerate() {
        //   list.push(List { elements: {Text { text: list.to_string(), size: 8 }} , numbered: false });
        // }
        // for (_, image) in img_vec.iter().enumerate() {
        //         // images.push(Image { bytes: (), title: (), alt: (), image_type: () })
        // }

        Ok(Document::new(paragraphs))
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
    use std::collections::HashMap;
    use bytes::Bytes;

    use super::*;
    #[test]
    fn mytest() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.rtf")?;
        let documents_bytes = Bytes::from(document);
        let parsed = Transformer::parse(&documents_bytes, &HashMap::new())?;
        let generated_result = crate::pdf::Transformer::generate(&parsed)?;
        std::fs::write("test/data/document_from_rtf.pdf", generated_result.0)?;

        Ok(())
    }
}

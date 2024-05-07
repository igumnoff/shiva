use crate::core::Element;
use crate::core::{
    Document,
    Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text},
    TableCell, TableHeader, TableRow, TransformerTrait,
};
use anyhow::Ok;
use regex::bytes::CaptureMatches;
use regex::Regex;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(
        document: &bytes::Bytes,
        imagesizes: &std::collections::HashMap<String, bytes::Bytes>,
    ) -> anyhow::Result<Document> {
        let document_str = std::str::from_utf8(document)?;
        let mut elements: Vec<Element> = Vec::new();
        let mut headers: Vec<Element> = Vec::new();
        let mut paragraphs: Vec<Element> = Vec::new();
        let mut hyperlinks: Vec<Element> = Vec::new();
        let mut images: Vec<Element> = Vec::new();
        let mut lists: Vec<String> = Vec::new();
        let mut footers: Vec<String> = Vec::new();

        let mut table_header: Vec<TableHeader> = Vec::new();
        let mut table_row: Vec<TableRow> = Vec::new();
        let mut table_cell: Vec<TableCell> = Vec::new();

        let table_re = Regex::new(r#"\trowd(.*?)\row"#).unwrap();
        let paragraph_re = Regex::new(r#"\\pard(.*?)\\par"#).unwrap();
        let image_re = Regex::new(r#"\pict(.*?)}"#).unwrap();
        let hyperlink_re = Regex::new(r#"HYPERLINK "(.+?)""#).unwrap();
        let list_re = Regex::new(r#"\pntext(.*?)}"#).unwrap();
        let footer_re = Regex::new(r#"\footnote(.*?)}"#).unwrap();

        let pard_match = paragraph_re.captures_iter(document_str);
        let tab_match: regex::CaptureMatches = table_re.captures_iter(document_str);
        let img_match = image_re.captures_iter(document_str);
        let hyper_match = hyperlink_re.captures_iter(document_str);
        let list_match = list_re.captures_iter(document_str);

        fn getcapture<'a>(m: regex::CaptureMatches<'a, 'a>) -> Vec<&'a str> {
            m.map(|capture| capture.get(0).unwrap().as_str()).collect()
        }
        let [pard_vec, tab_vec, img_vec, hyper_vec, list_vec] =
            [pard_match, tab_match, img_match, hyper_match, list_match].map(getcapture);

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
            table_row.push(TableRow {
                cells: table_cell.clone(),
            })
        }
        for (_, link) in hyper_vec.iter().enumerate() {
            //     // hyperlinks.push(Hyperlink { title: (), url: (), alt: (), size: 8 })
        }
        for (index, list) in list_vec.into_iter().enumerate() {
            //     // lists.push(List {elements: vec![Text { text: list.to_string(), size: 8 }]}, numbered: false })
        }

        for (_, image) in img_vec.iter().enumerate() {
            //     // images.push(Image { bytes: (), title: (), alt: (), image_type: () })
        }
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

    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"{\rtf1\ansi\deff0 {\fonttbl {\f0 Times;}}\fs24
        {\pard
     this is a first paragrap
        {\pard
 this is a second paragraph
        \par}
        }"#;
        let parsed = Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        assert!(parsed.is_ok());
        let parsed = parsed?;
       Ok(())
    }
}

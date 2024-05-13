use crate::core::Element::{Table, Text};
use crate::core::{
    Document, Element, ListItem, TableCell, TableRow,
    TransformerTrait,
};

use bytes::Bytes;
use docx_rs::{read_docx, NumberingProperty, ParagraphStyle, RunChild, TableRowChild};
use std::
    collections::HashMap
;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        fn extract_text(doc_element: &docx_rs::Paragraph) -> String {
            for c in &doc_element.children {
                match c {
                    docx_rs::ParagraphChild::Run(run) => {
                        println!("run.children - {:?} - ", run.children);
                        if run.children.len() == 0 {
                            return "".to_string();
                        }
                        if let RunChild::Text(t) = &run.children[0] {
                            return t.text.to_string();
                        }
                    }
                    _ => {}
                }
            }
            return "".to_string();
        }

        fn push_list_item(
            result: &mut Vec<Element>,
            is_collecting_list: &mut bool,
            last_list_indent: &mut usize,
            par: &docx_rs::Paragraph,
            numbering_property: &NumberingProperty,
        ) {
            let numbered = numbering_property
                .id
                .as_ref()
                .expect("No number id in list item")
                .id
                == 3;
            let list_indent = numbering_property
                .level
                .as_ref()
                .expect("Expect indent level to be Some")
                .val;

            if !*is_collecting_list {
                *is_collecting_list = true;

                let text = extract_text(&par);
                let list_item = ListItem {
                    element: Text { text, size: 16 },
                };
                let element = Element::List {
                    elements: vec![list_item],
                    numbered,
                };

                result.push(element);

                *last_list_indent = list_indent;

            } else {
                if *last_list_indent == list_indent {
                    let text = extract_text(&par);
                    let list_item = ListItem {
                        element: Text { text, size: 16 },
                    };
                    let element = Element::List {
                        elements: vec![list_item],
                        numbered,
                    };
    
                    result.push(element);
    
                    *last_list_indent = list_indent;
                }

            }
        }

        let docx = read_docx(&document)?;
        const HEADING1: &str = "Heading1";
        const HEADING2: &str = "Heading2";
        const NORMAL: &str = "Normal";
        const BODY_TEXT: &str = "BodyText";
        let mut result: Vec<Element> = vec![];
        let mut is_collecting_list = false;
        let mut last_list_indent: usize = 0;
        for ch in docx.document.children {
            println!("ch - {:?}", ch);

            if let docx_rs::DocumentChild::Paragraph(par) = ch {
                if let Some(numbering_property) = &par.property.numbering_property {
                    push_list_item(
                        &mut result,
                        &mut is_collecting_list,
                        &mut last_list_indent,
                        &par,
                        numbering_property,
                    );
                } else {
                    if is_collecting_list {

                    }
                    is_collecting_list = false;
                    match &par.property.style {
                        Some(ParagraphStyle { val }) => match val.as_str() {
                            HEADING1 => {
                                let text = extract_text(&par);
                                let element = Element::Header {
                                    level: 1,
                                    text: text,
                                };

                                result.push(element);
                            }
                            HEADING2 => {
                                let text = extract_text(&par);
                                let element = Element::Header {
                                    level: 2,
                                    text: text,
                                };

                                result.push(element);
                            }

                            BODY_TEXT => {
                                let text = extract_text(&par);
                                let element = Element::Text { text, size: 16 };

                                result.push(element);
                            }

                            NORMAL => {
                                let text = extract_text(&par);
                                let element = Element::Text { text, size: 16 };

                                result.push(element);
                            }

                            _ => {}
                        },
                        _ => {
                            unimplemented!("Should implement");
                        }
                    }
                }
            } else {
                match ch {
                    docx_rs::DocumentChild::Table(table) => {
                        is_collecting_list = false;
                        let mut rows = vec![];
                        for row in &table.rows {
                            let docx_rs::TableChild::TableRow(tr) = row;
                            // println!("tr - {:?}", tr);
                            let mut cells = TableRow { cells: vec![] };
    
                            for table_cell in &tr.cells {
                                let TableRowChild::TableCell(tc) = table_cell;
                                println!("tc - {:?}", tc);
                                for ch in &tc.children {
                                    match ch {
                                        docx_rs::TableCellContent::Paragraph(par) => {
                                            let text = extract_text(&par);
                                            cells.cells.push(TableCell {
                                                element: Element::Text { text, size: 16 },
                                            });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            rows.push(cells);
                        }
    
                        result.push(Table {
                            headers: vec![],
                            rows,
                        });
                    }
                    _ => {}
                }
            }
      
        }

        Ok(Document::new(result))
    }
    fn generate(_document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::TransformerTrait;
    use crate::docx;
    use bytes::Bytes;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        // read from document.docx file from disk
        let document = std::fs::read("test/data/document.docx")?;
        let bytes = Bytes::from(document);
        let images = HashMap::new();
        let parsed = docx::Transformer::parse(&bytes, &images);
        assert!(parsed.is_ok());
        println!(
            "--------------------------------------------\n parsed - {:?}",
            parsed
        );

        Ok(())
    }
}

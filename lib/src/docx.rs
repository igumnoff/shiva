use crate::core::Element::Table;
use crate::core::{Document, Element, ListItem, TableCell, TableRow, TransformerTrait};

use bytes::Bytes;
use docx_rs::{read_docx, ParagraphStyle, RunChild, TableRowChild, Docx, Paragraph, Run};
use std::collections::HashMap;
use std::io::Cursor;

pub struct Transformer;


impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        fn extract_text(doc_element: &docx_rs::Paragraph) -> String {
            for c in &doc_element.children {
                match c {
                    docx_rs::ParagraphChild::Run(run) => {
                        if run.children.is_empty() {
                            return "".to_string();
                        }
                        if let RunChild::Text(t) = &run.children[0] {
                            return t.text.to_string();
                        }
                    }
                    _ => {}
                }
            }
            "".to_string()
        }

  
        let docx = read_docx(document)?;
        const HEADING1: &str = "Heading1";
        const HEADING2: &str = "Heading2";
        const NORMAL: &str = "Normal";
        const BODY_TEXT: &str = "BodyText";
        let mut result: Vec<Element> = vec![];

        let mut is_list_numbered = false;

        let mut current_list: Option<(usize, Vec<ListItem>)> = None;

        for ch in docx.document.children {

            if let docx_rs::DocumentChild::Paragraph(par) = ch {
                if let Some(numbering_property) = &par.property.numbering_property {
                    let num_id = numbering_property
                        .id
                        .as_ref()
                        .expect("No number id in list item")
                        .id;
                    if num_id == 3 || num_id == 2 {

                        let list_text = extract_text(&par);

                        let list_item = ListItem {
                            element: Element::Text {
                                text: list_text,
                                size: 12,
                            },
                        };

                        let numbered = numbering_property
                            .id
                            .as_ref()
                            .expect("No number id in list item")
                            .id
                            == 3;
                        let level = numbering_property
                            .level
                            .as_ref()
                            .expect("Expect indent level to be Some")
                            .val;
                        if let Some((last_level, ref mut list_items)) = current_list {
                            if level > last_level {
                                let nested_list = Element::List {
                                    elements: vec![list_item],
                                    numbered,
                                };
                                list_items.push(ListItem {
                                    element: nested_list,
                                });
                            } else if level < last_level {
                                // Finish the current list and start a new one
                                result.push(Element::List {
                                    elements: list_items.clone(),
                                    numbered,
                                });
                                current_list = Some((level, vec![list_item]));
                            } else {
                                list_items.push(list_item);
                            }
                        } else {
                            current_list = Some((level, vec![list_item]));
                            is_list_numbered = numbered;
                        }
                    } else {
                        if let Some((_, list_items)) = current_list.take() {
                            result.push(Element::List {
                                elements: list_items,
                                numbered: is_list_numbered,
                            });
                        }
        
                        match &par.property.style {
                            Some(ParagraphStyle { val }) => match val.as_str() {
                                HEADING1 => {
                                    let text = extract_text(&par);
                                    let element = Element::Header {
                                        level: 1,
                                        text,
                                    };

                                    result.push(element);
                                }
                                HEADING2 => {
                                    let text = extract_text(&par);
                                    let element = Element::Header {
                                        level: 2,
                                        text,
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
                    if let Some((_, list_items)) = current_list.take() {
                        result.push(Element::List {
                            elements: list_items,
                            numbered: is_list_numbered,
                        });
                    }
                    match &par.property.style {
                        Some(ParagraphStyle { val }) => match val.as_str() {
                            HEADING1 => {
                                let text = extract_text(&par);
                                let element = Element::Header {
                                    level: 1,
                                    text,
                                };

                                result.push(element);
                            }
                            HEADING2 => {
                                let text = extract_text(&par);
                                let element = Element::Header {
                                    level: 2,
                                    text,
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
                if let Some((_, list_items)) = current_list.take() {
                    result.push(Element::List {
                        elements: list_items,
                        numbered: is_list_numbered,
                    });
                }
                match ch {
                    docx_rs::DocumentChild::Table(table) => {
                        let mut rows = vec![];
                        for row in &table.rows {
                            let docx_rs::TableChild::TableRow(tr) = row;
                            let mut cells = TableRow { cells: vec![] };

                            for table_cell in &tr.cells {
                                let TableRowChild::TableCell(tc) = table_cell;
                                for ch in &tc.children {
                                    match ch {
                                        docx_rs::TableCellContent::Paragraph(par) => {
                                            let text = extract_text(par);
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

        if let Some((_, list_items)) = current_list.take() {
            result.push(Element::List {
                elements: list_items,
                numbered: is_list_numbered,
            });
        }


        Ok(Document::new(result))
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {

        let mut doc = Docx::new();

        for element in &document.elements {
            match element {
                Element::Header { level, text } => {
                    let size = match level {
                        1 => 32,
                        2 => 28,
                        3 => 24,
                        4 => 20,
                        5 => 16,
                        6 => 12,
                        _ => 10,
                    };
                    doc = doc.add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(text).bold().size(size))
                    );
                }

                Element::Paragraph { elements } => {
                    let mut paragraph = Paragraph::new();
                    for elem in elements {
                        if let Element::Text { text, size } = elem {
                            paragraph = paragraph.add_run(Run::new().add_text(text).size((*size * 2) as usize));
                        }
                    }

                    doc = doc.add_paragraph(paragraph);
                }

                _ => {
                    eprintln!("Unknown element");
                }
            }
        }

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        doc.build().pack(&mut cursor)?;
        let buffer = cursor.into_inner();

        Ok((bytes::Bytes::from(buffer), HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::TransformerTrait;
    use crate::{docx, html};
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
            "--------------------------------------------\n parsed - {:#?}",
            parsed
        );

        let result = html::Transformer::generate(&parsed?)?;
        //write to file
        std::fs::write("test/data/document_from_docx.html", result.0)?;

        Ok(())
    }
}

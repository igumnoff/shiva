use crate::core::Element::{Table, Text};
use crate::core::{Document, Element, ListItem, TableCell, TableRow, TransformerTrait};

use bytes::Bytes;
use docx_rs::{read_docx, NumberingProperty, ParagraphStyle, RunChild, TableRowChild};
use std::collections::HashMap;

pub struct Transformer;

#[derive(Debug)]
struct ParsedListEntity {
    numbered: bool,
    text: String,
}

#[derive(Debug)]
enum ParseList {
    Item(ParsedListEntity),
    NestedList(Vec<ParseList>),
}

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

        // fn extract_nested_list(nl: &mut ParseList) -> &mut ParseList {
        //     if let ParseList::NestedList(nested_list) = nl {
        //         return extract_nested_list(&mut nested_list.pop().unwrap());
        //     }

        //     return nl;
        // }

        // fn push_list_item(
        //     result: &mut Option<Vec<ParseList>>,
        //     is_collecting_list: &mut bool,
        //     last_list_indent: &mut usize,
        //     par: &docx_rs::Paragraph,
        //     numbering_property: &NumberingProperty,
        // ) {
        //     println!("Result in push_list_item - {:?}", result);
        //     let numbered = numbering_property
        //         .id
        //         .as_ref()
        //         .expect("No number id in list item")
        //         .id
        //         == 3;
        //     let list_indent = numbering_property
        //         .level
        //         .as_ref()
        //         .expect("Expect indent level to be Some")
        //         .val;
        //     let text = extract_text(&par);

        //     if !*is_collecting_list {
        //         *is_collecting_list = true;

        //         // let text = extract_text(&par);
        //         // let list_item = ListItem {
        //         //     element: Text { text, size: 16 },
        //         // };
        //         // let element = Element::List {
        //         //     elements: vec![list_item],
        //         //     numbered,
        //         // };

        //         let li = ParsedListEntity { numbered, text };
        //         let list_item = ParseList::Item(li);

        //         *result = Some(vec![list_item]);

        //         *last_list_indent = list_indent;
        //     } else {
        //         if list_indent == *last_list_indent {
        //             // let vec_parse_list = result.as_mut().unwrap();

        //             // let li = ParseList::Item(ParsedListEntity { numbered, text });

        //             // vec_parse_list.push(li);
        //             if list_indent != 0 {

        //                 let mut vec_parse_list = result.as_mut().unwrap().pop();
        //                  let mut vec_parse_list = vec_parse_list.as_mut().unwrap();

        //                 while let ParseList::NestedList(nested_list) =  vec_parse_list {
        //                     if let Some(mut reg inner) = nested_list.pop() {
        //                         vec_parse_list = inner;
        //                     } else {
        //                         break;
        //                     }
        //                 }
        //                 let mut li = ParseList::Item(ParsedListEntity { numbered, text });

        //                 for _ in 0..list_indent {
        //                     li = ParseList::NestedList(vec![li]);
        //                 }

        //                 // vec_parse_list.push(li);
        //             }
        //         } else {
        //             let mut li = ParseList::Item(ParsedListEntity { numbered, text });
        //             let vec_parse_list = result.as_mut().unwrap();

        //             for _ in 0..list_indent {
        //                 li = ParseList::NestedList(vec![li]);
        //             }
        //             vec_parse_list.push(li);

        //             *last_list_indent = list_indent;
        //         }
        //         // let mut li = ParseList::Text(ParsedListEntity { numbered, text });
        //         // for i in 0..list_indent {
        //         //     li = ParseList::NestedList(Box::new(li));
        //         // }

        //         // result.as_mut().unwrap().push(li);

        //         // if *last_list_indent == list_indent {
        //         //     if let Some(parse_list)  = result {
        //         //         let li = ParsedListEntity {
        //         //             numbered,
        //         //             text
        //         //         };
        //         //         parse_list.push(ParseList::Text(li));
        //         //     }
        //         //     // let list_item = ListItem {
        //         //     //     element: Text { text, size: 16 },
        //         //     // };
        //         //     // let element = Element::List {
        //         //     //     elements: vec![list_item],
        //         //     //     numbered,
        //         //     // };

        //         //     // result.push(element);

        //         //     // *last_list_indent = list_indent;

        //         // }else {

        //         //     let li = ParsedListEntity {
        //         //         numbered,
        //         //         text
        //         //     };
        //         //     let mut parsed_list_item = ParseList::Text(li);

        //         //     for i in 0..l
        //         // }
        //     }
        // }

        // fn parse_li(item: ParseList) -> Element

        fn parse_list(list_items: Vec<ParseList>) -> Element {
            let result_elements = vec![];

            for li in list_items {}

            return Element::List {
                elements: result_elements,
                numbered: true,
            };
        }

        let docx = read_docx(&document)?;
        const HEADING1: &str = "Heading1";
        const HEADING2: &str = "Heading2";
        const NORMAL: &str = "Normal";
        const BODY_TEXT: &str = "BodyText";
        let mut result: Vec<Element> = vec![];
        let mut is_collecting_list = false;
        let mut last_list_indent: usize = 0;

        let mut list_items: Option<Vec<ParseList>> = None;

        let mut current_list:Option<(usize, Vec<ListItem>)> = None;

        for ch in docx.document.children {
            println!("ch - {:?}", ch);

            if let docx_rs::DocumentChild::Paragraph(par) = ch {
                if let Some(numbering_property) = &par.property.numbering_property {
                    let num_id = numbering_property
                        .id
                        .as_ref()
                        .expect("No number id in list item")
                        .id;
                    if num_id == 3 || num_id == 2 {
                        // push_list_item(
                        //     &mut list_items,
                        //     &mut is_collecting_list,
                        //     &mut last_list_indent,
                        //     &par,
                        //     numbering_property,
                        // );

                        let list_text = extract_text(&par);
    
                    let list_item = ListItem {
                        element: Element::Text { text: list_text, size: 12 },
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
                            list_items.push(ListItem { element: nested_list });
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
                    }
                    } else {
                        if is_collecting_list {
                            println!("list_items - {:?}", list_items);
                            let elements = parse_list(list_items.unwrap());
                            list_items = None;
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
                    if is_collecting_list {
                        println!("list_items - {:?}", list_items);
                        let elements = parse_list(list_items.unwrap());

                        list_items = None;
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
                if is_collecting_list {
                    println!("list_items - {:?}", list_items);
                    let elements = parse_list(list_items.unwrap());

                    list_items = None;
                }
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

        if let Some(lists) = list_items {
            println!("lists - {:?}", lists);

            let elements = parse_list(lists);
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

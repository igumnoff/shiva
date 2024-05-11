use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use crate::core::{
    Document, Element, ImageType, ListItem, ParserError, TableCell, TableHeader, TableRow,
    TransformerTrait,
};

use bytes::{Buf, Bytes};
use quick_xml::events::BytesEnd;
use quick_xml::name::QName;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::borrow::Cow;
use std::{
    collections::HashMap,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};
use zip::ZipArchive;

enum ParagraphStyle {
    Heading1,
    Heading2,
    Normal,
    BodyText,
}
#[derive(Debug)]
struct ElementBuilder {
    element: Option<Element>,
    table_grid_count: usize,
}

impl ElementBuilder {
    fn new(tag_name: QName) -> Self {
        println!("tag_name - {:?}", tag_name);
        match tag_name {
            QName(b"w:p") => {
                return Self {
                    element: Some(Paragraph { elements: vec![] }),
                    table_grid_count: 0,
                };
            }
            QName(b"w:tbl") => {
                return Self {
                    element: Some(Table {
                        headers: vec![],
                        rows: vec![],
                    }),
                    table_grid_count: 0,
                };
            }
            _ => {
                return Self {
                    table_grid_count: 0,
                    element: None,
                };
            }
        }
    }

    fn add_empty_tag(&mut self, e: BytesStart) {
        match e.name().0 {
            b"w:pStyle" => {
                let attrs = e
                    .attributes()
                    .find(|e| {
                        let a = e.as_ref().unwrap();
                        return a.key == QName(b"w:val");
                    })
                    .unwrap()
                    .unwrap();

                let el = self.element.as_mut();

                match el {
                    Some(Element::Paragraph { elements }) => {
                        let value = attrs.value.into_owned();

                        match &value[..] {
                            b"Heading1" => elements.push(Header {
                                level: 1,
                                text: "".to_string(),
                            }),
                            b"Heading2" => elements.push(Header {
                                level: 2,
                                text: "".to_string(),
                            }),
                            b"Normal" | b"BodyText" => elements.push(Text {
                                text: "".to_string(),
                                size: 16,
                            }),
                            _ => {}
                        }
                    }

                    Some(Element::Table { headers, rows }) => {
                        let value = attrs.value.into_owned();
                        println!("value - {:?}", String::from_utf8_lossy(&value[..]));
                    }

                    None => {}
                    _ => {}
                }
            }
            b"w:gridCol" => {
                self.table_grid_count += 1;
            }
            _ => {}
        }
    }

    fn add_text(&mut self, txt: String) {
        println!("mut self - {:?}", self.element);

        let el = self.element.as_mut();
        println!("add_text txt - {:?}", txt);
        match el {
            Some(Element::Paragraph { elements }) => {
                let mut last_el = elements.pop();
                let last_el = last_el.as_mut();

                match last_el {
                    Some(Element::Text { text, size }) => {
                        text.push_str(&txt);
                        elements.push(last_el.unwrap().clone());
                    }
                    Some(Element::Header { level, text }) => {
                        text.push_str(&txt);
                        elements.push(last_el.unwrap().clone());
                    }
                    _ => {}
                }
            }
            Some(Element::Table { headers, rows }) => {
                if headers.len() < self.table_grid_count {
                    headers.push(TableHeader {
                        element: Element::Text {
                            text: txt,
                            size: 16,
                        },
                        width: 30.,
                    })
                } else {
                    let last_index;
                    if rows.len() == 0 {
                        last_index = 0;
                    } else {
                        last_index = rows.len() - 1;
                    }
                    match rows.get_mut(last_index) {
                        Some(TableRow { cells }) => {
                            if cells.len() < self.table_grid_count {
                                cells.push(TableCell {
                                    element: Element::Text {
                                        text: txt,
                                        size: 16,
                                    },
                                })
                            }
                        }
                        None => rows.push(TableRow {
                            cells: vec![TableCell {
                                element: Element::Text {
                                    text: txt,
                                    size: 16,
                                },
                            }],
                        }),
                    }
                }
            }
            None => {}
            _ => {}
        }
    }

    fn finish_element(&mut self, e: BytesEnd) -> bool {
        let el = self.element.as_mut();
        match el {
            Some(Element::Paragraph { elements }) => {
                if e.name() == QName(b"w:p") {
                    return true;
                }
            }
            Some(Element::Table { headers, rows }) => {
                if e.name() == QName(b"w:tbl") {
                    return true;
                }
            }
            None => {}
            _ => {}
        }
        return false;
    }
}

impl Default for ElementBuilder {
    fn default() -> Self {
        Self {
            element: None,
            table_grid_count: 0,
        }
    }
}
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        fn parse_docx_xml(content: Vec<u8>) {
            let mut xml = Reader::from_reader(content.reader());

            let mut buf = Vec::new();

            let mut element: ElementBuilder = ElementBuilder::default();
            let mut result: Vec<Element> = vec![];
            loop {
                let read_data = xml.read_event_into(&mut buf);

                match read_data {
                    Ok(Event::Start(e)) => {
                        println!("Event::Start: {:?}", e);
                        match e.name() {
                            QName(b"w:p") => {
                                if element.element.is_none() {
                                    element = ElementBuilder::new(e.name());
                                }
                            }
                            QName(b"w:tbl") => {
                                element = ElementBuilder::new(e.name());
                                println!("element - :tbl{:?}", element);
                            }

                            _ => {}
                        }
                    }
                    Ok(Event::Empty(e)) => element.add_empty_tag(e),

                    Ok(Event::Text(e)) => {
                        println!("Event::Text(e) - {:?}", e);
                        element.add_text(e.unescape().unwrap().into_owned());
                    }
                    Ok(Event::End(e)) => {
                        if element.finish_element(e) {
                            println!("element End - {:?}", element);
                            result.push(element.element.take().unwrap());
                        }
                    }

                    Ok(Event::Eof) => break,

                    _ => {}
                }

                buf.clear();
            }

            println!("result - {:?}", result);
        }

        let cursor = Cursor::new(document);
        let mut zip = ZipArchive::new(cursor).expect("Wasn't able to read document");
        let mut main_document = zip
            .by_name("word/document.xml")
            .expect("Didn't find document xml file in docx");

        let mut content = vec![];

        main_document
            .read_to_end(&mut content)
            .expect("Couldn't read content of document into buffer");

        parse_docx_xml(content);

        todo!()
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
        Ok(())
    }
}

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE as BS64, Engine as _};
use bytes::Bytes;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Reader, Writer,
};
use std::collections::HashMap;
use std::str::from_utf8;

use crate::core::{Document, Element, ImageType, TransformerTrait};

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> Result<Document> {
        let xml_data = from_utf8(document)?;
        let mut reader = Reader::from_str(xml_data);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut elements = Vec::new();
        let mut stack = Vec::new();
        let mut temp_attributes = HashMap::new();

        let page_width = 210.0;
        let page_height = 297.0;
        let left_page_indent = 10.0;
        let right_page_indent = 10.0;
        let top_page_indent = 10.0;
        let bottom_page_indent = 10.0;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => {
                    let tag_name = e.name();
                    let QName(name) = tag_name;

                    match name {
                        b"paragraph" => {
                            stack.push(Element::Paragraph {
                                elements: Vec::new(),
                            });
                        }
                        b"text" => {
                            let attrs = e
                                .attributes()
                                .filter_map(|attr| attr.ok())
                                .map(|attr| {
                                    (
                                        from_utf8(attr.key.into_inner())
                                            .unwrap_or_default()
                                            .to_string(),
                                        from_utf8(&attr.value).unwrap_or_default().to_string(),
                                    )
                                })
                                .collect::<HashMap<_, _>>();
                            let size = attrs
                                .get("size")
                                .and_then(|s| s.parse::<u8>().ok())
                                .unwrap_or(12);
                            temp_attributes.insert("size".to_string(), size.to_string());
                        }
                        b"header" => {
                            let attrs = e
                                .attributes()
                                .filter_map(|attr| attr.ok())
                                .map(|attr| {
                                    (
                                        from_utf8(attr.key.into_inner())
                                            .unwrap_or_default()
                                            .to_string(),
                                        from_utf8(&attr.value).unwrap_or_default().to_string(),
                                    )
                                })
                                .collect::<HashMap<_, _>>();
                            temp_attributes = attrs;
                        }
                        b"image" => {
                            let attrs = e
                                .attributes()
                                .filter_map(|attr| attr.ok())
                                .map(|attr| {
                                    (
                                        from_utf8(attr.key.into_inner())
                                            .unwrap_or_default()
                                            .to_string(),
                                        from_utf8(&attr.value).unwrap_or_default().to_string(),
                                    )
                                })
                                .collect::<HashMap<_, _>>();
                            let title = attrs.get("title").cloned().unwrap_or_default();
                            let alt = attrs.get("alt").cloned().unwrap_or_default();
                            let image_type = attrs
                                .get("type")
                                .map(|t| {
                                    match t.as_str() {
                                        "png" => ImageType::Png,
                                        "jpeg" => ImageType::Jpeg,
                                        _ => ImageType::Png, // default or fallback
                                    }
                                })
                                .unwrap_or(ImageType::Png);
                            let bytes = attrs
                                .get("data")
                                .and_then(|d| BS64.decode(d.as_bytes()).ok())
                                .unwrap_or_default();
                            stack.push(Element::Image {
                                bytes: Bytes::from(bytes),
                                title,
                                alt,
                                image_type,
                            });
                        }
                        // Add cases for `list` and `table` with similar logic
                        _ => {}
                    }
                }

                Event::Text(ref e) => {
                    let text = e.unescape()?.to_string();

                    if let Some(Element::Paragraph { ref mut elements }) = stack.last_mut() {
                        elements.push(Element::Text { text, size: 12 });
                    } else {
                        elements.push(Element::Text { text, size: 12 });
                    }
                }
                Event::End(ref e) => {
                    let tag_name = e.name();
                    let QName(name) = tag_name;

                    match name {
                        b"document" => {}
                        b"paragraph" => {
                            if let Some(Element::Paragraph {
                                elements: sub_elements,
                            }) = stack.pop()
                            {
                                elements.push(Element::Paragraph {
                                    elements: sub_elements,
                                });
                            }
                        }
                        b"text" | b"header" | b"list" | b"table" => {
                            if name == b"header" {
                                if let Some(level) = temp_attributes
                                    .get("level")
                                    .and_then(|l| l.parse::<u8>().ok())
                                {
                                    elements.push(Element::Header {
                                        level,
                                        text: temp_attributes
                                            .get("text")
                                            .cloned()
                                            .unwrap_or_default(),
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(Document {
            elements,
            page_width,
            page_height,
            left_page_indent,
            right_page_indent,
            top_page_indent,
            bottom_page_indent,
            page_header: vec![],
            page_footer: vec![],
        })
    }

    fn generate(document: &Document) -> Result<(Bytes, HashMap<String, Bytes>)> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new(&mut buffer);

        writer.write_event(Event::Start(BytesStart::new("document")))?;

        fn serialize_element(element: &Element, writer: &mut Writer<&mut Vec<u8>>) -> Result<()> {
            match element {
                Element::Paragraph { elements } => {
                    writer.write_event(Event::Start(BytesStart::new("paragraph")))?;
                    for sub_element in elements {
                        serialize_element(sub_element, writer)?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("paragraph")))?;
                }
                Element::Text { text, size } => {
                    let mut text_start = BytesStart::new("text");
                    text_start.push_attribute(("size", size.to_string().as_str()));
                    writer.write_event(Event::Start(text_start))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("text")))?;
                }
                Element::Hyperlink {
                    title,
                    url,
                    alt,
                    size,
                } => {
                    let mut header_start = BytesStart::new("header");
                    header_start.push_attribute(("size", size.to_string().as_str()));
                    header_start.push_attribute(("url", url.to_string().as_str()));
                    header_start.push_attribute(("alt", alt.to_string().as_str()));

                    writer.write_event(Event::Start(header_start))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(title)))?;
                    writer.write_event(Event::End(BytesEnd::new("header")))?;
                }
                Element::Header { level, text } => {
                    let mut header_start = BytesStart::new("header");
                    header_start.push_attribute(("level", level.to_string().as_str()));
                    writer.write_event(Event::Start(header_start))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("header")))?;
                }
                Element::List { elements, numbered } => {
                    let mut list_start = BytesStart::new("list");
                    list_start
                        .push_attribute(("type", if *numbered { "numbered" } else { "bulleted" }));
                    writer.write_event(Event::Start(list_start))?;
                    for element in elements {
                        serialize_element(&element.element, writer)?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("list")))?;
                }
                Element::Table { rows, headers } => {
                    let table_start = BytesStart::new("table");
                    writer.write_event(Event::Start(table_start))?;

                    writer.write_event(Event::Start(BytesStart::new("thead")))?;
                    for header in headers {
                        let mut header_start = BytesStart::new("th");
                        header_start.push_attribute(("width", header.width.to_string().as_str()));
                        writer.write_event(Event::Start(header_start))?;
                        serialize_element(&header.element, writer)?;
                        writer.write_event(Event::End(BytesEnd::new("th")))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("thead")))?;

                    writer.write_event(Event::Start(BytesStart::new("tbody")))?;
                    for row in rows {
                        writer.write_event(Event::Start(BytesStart::new("tr")))?;
                        for cell in &row.cells {
                            writer.write_event(Event::Start(BytesStart::new("td")))?;
                            serialize_element(&cell.element, writer)?;
                            writer.write_event(Event::End(BytesEnd::new("td")))?;
                        }
                        writer.write_event(Event::End(BytesEnd::new("tr")))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("tbody")))?;
                    writer.write_event(Event::End(BytesEnd::new("table")))?;
                }
                Element::Image {
                    bytes,
                    title,
                    alt,
                    image_type,
                } => {
                    let mut image_start = BytesStart::new("image");
                    image_start.push_attribute(("title", title.as_str()));
                    image_start.push_attribute(("alt", alt.as_str()));
                    image_start.push_attribute((
                        "type",
                        match image_type {
                            ImageType::Png => "png",
                            ImageType::Jpeg => "jpeg",
                        },
                    ));
                    writer.write_event(Event::Start(image_start))?;
                    writer
                        .write_event(Event::Text(BytesText::from_escaped(&BS64.encode(bytes))))?;
                    writer.write_event(Event::End(BytesEnd::new("image")))?;
                }
            }
            Ok(())
        }

        for element in &document.elements {
            serialize_element(element, &mut writer)?;
        }

        writer.write_event(Event::End(BytesEnd::new("document")))?;

        Ok((Bytes::from(buffer), HashMap::new()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        Document, Element, ImageType, ListItem, TableCell, TableHeader, TableRow, TransformerTrait,
    };
    use bytes::Bytes;

    #[test]
    fn test_serialize_parse_round_trip() {
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
                bytes: Bytes::from_static(b""),
                title: "Sample Image".to_string(),
                alt: "An empty image".to_string(),
                image_type: ImageType::Png,
            },
            Element::Paragraph {
                elements: vec![Element::Text {
                    text: "Hello, world!".to_string(),
                    size: 12,
                }],
            },
            Element::Table {
                headers: vec![
                    TableHeader {
                        element: Element::Text {
                            text: "Syntax".to_string(),
                            size: 8,
                        },
                        width: 10.0,
                    },
                    TableHeader {
                        element: Element::Text {
                            text: "Description".to_string(),
                            size: 8,
                        },
                        width: 10.0,
                    },
                ],
                rows: vec![
                    TableRow {
                        cells: vec![
                            TableCell {
                                element: Element::Text {
                                    text: "Header".to_string(),
                                    size: 8,
                                },
                            },
                            TableCell {
                                element: Element::Text {
                                    text: "Title".to_string(),
                                    size: 8,
                                },
                            },
                        ],
                    },
                    TableRow {
                        cells: vec![
                            TableCell {
                                element: Element::Text {
                                    text: "Paragraph".to_string(),
                                    size: 8,
                                },
                            },
                            TableCell {
                                element: Element::Text {
                                    text: "Text".to_string(),
                                    size: 8,
                                },
                            },
                        ],
                    },
                ],
            },
            Element::List {
                elements: vec![ListItem {
                    element: Element::Text {
                        text: "List item 1".to_string(),
                        size: 8,
                    },
                }],
                numbered: false,
            },
        ];

        let document = Document {
            elements: elements.clone(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: vec![],
            page_footer: vec![],
        };

        let (bytes, _images) = Transformer::generate(&document).unwrap();
        println!("{:}", std::str::from_utf8(&bytes).unwrap());

        let parsed_document = Transformer::parse(&bytes, &_images).unwrap();

        assert_eq!(elements, parsed_document.elements);
    }
}

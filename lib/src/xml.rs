use anyhow::Result;
use bytes::Bytes;
use quick_xml::{ events::{ BytesDecl, BytesEnd, BytesStart, BytesText, Event }, Reader, Writer };
use std::str::{from_utf8};

use crate::core::{
    ImageDimension, Document, Element, ImageAlignment, ImageData, ImageType, ListItem, TableCell, TableHeader, TableRow, TransformerTrait
};

use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    key: String,
    value: String,
}

impl Node {
    fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let mut stack: Vec<Node> = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut current_node = None;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut attributes = Vec::new();
                    for attr  in e.attributes() {
                        let attr = attr?;
                        attributes.push(Attribute {
                            key: from_utf8(attr.key.as_ref())?.to_string(),
                            value: attr.decode_and_unescape_value(&reader)?.into_owned(),
                        });
                    }

                    let new_node = Node {
                        name,
                        attributes,
                        children: Vec::new(),
                        text: None,
                    };

                    if let Some(node) = current_node.take() {
                        stack.push(node);
                    }
                    current_node = Some(new_node);
                }
                Event::End(ref e) => {
                    if String::from_utf8_lossy(e.as_ref()).to_string() == "end".to_string() {
                    }
                    if let Some(node) = current_node.take() {
                        if let Some(mut parent) = stack.pop() {
                            parent.children.push(node);
                            current_node = Some(parent);
                        } else {
                            nodes.push(node);
                        }
                    }
                }
                Event::Text(e) => {
                    if String::from_utf8_lossy(e.as_ref()).to_string() == "end".to_string() {
                    }
                    if let Some(node) = &mut current_node {
                        if let Ok(text) = e.unescape() {
                            node.text = Some(text.into_owned());
                        }
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(nodes)
    }
}

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> Result<Document> {
        let xml_data = from_utf8(&document)?;
        let mut reader = Reader::from_str(xml_data);
        reader.trim_text(true);

        let mut element_data: Option<&Node> = None;

        let tree = Node::from_xml(&mut reader);
        for node in &tree {
            for child in node {
                element_data = Some(child);
            }
        }

        let mut elements = Vec::new();

        for child in element_data.unwrap().children.iter() {
            match child.name.as_str() {
                "elements" => {
                    elements = parse_element(child)?;
                }
                _ => {}
            }
        }

        fn parse_element(element_data: &Node) -> anyhow::Result<Vec<Element>> {
            let mut elements = Vec::new();
            for element in element_data.children.iter() {
                match element.name.as_str() {
                    "Paragraph" => {
                        let sub_elements = parse_element(element)?;
                        elements.push(Element::Paragraph { elements: sub_elements });
                    }
                    "List" => {
                        let mut numbered = false;
                        let mut sub_elements: Vec<ListItem> = vec![];
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "elements" => {
                                    sub_elements = list_parse_element(child)?;
                                }
                                "numbered" => {
                                    if let Some(value) = &child.text {
                                        numbered = value == "true";
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        elements.push(Element::List { elements: sub_elements, numbered: numbered });
                    }
                    "elements" => {
                        elements = parse_element(element)?;
                    }
                    "Text" => {
                        let mut text = "_";
                        let mut size = 10;
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "size" => {
                                    if let Some(value) = &child.text {
                                        size = value.parse()?;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "text" => {
                                    if let Some(value) = &child.text {
                                        text = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        elements.push(Element::Text { text: text.to_string(), size: size });
                    }
                    "Image" => {
                        let mut image_bytes = Bytes::new();
                        let mut alt = "_";
                        let mut title = "_";
                        let mut image_type = ImageType::default().to_string();
                        let mut align = ImageAlignment::default().to_string();
                        let mut width = None;
                        let mut height = None;
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "image_type" => {
                                    if let Some(value) = &child.text {
                                        image_type = value.to_string();
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "bytes" => {
                                    if let Some(value) = &child.text {
                                        image_bytes = Bytes::from(value.to_string());
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "alt" => {
                                    if let Some(value) = &child.text {
                                        alt = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "title" => {
                                    if let Some(value) = &child.text {
                                        title = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                },
                                "align" => {
                                    if let Some(value) = &child.text {
                                        align = value.to_string();
                                    } else {
                                        println!("Error: No value");
                                    }
                                },
                                "width" => {
                                    if let Some(value) = &child.text {
                                        width = Some(value.parse()?);
                                    } else {
                                        println!("Error: No value");
                                    }
                                },
                                "height" => {
                                    if let Some(value) = &child.text {
                                        height = Some(value.parse()?);
                                    } else {
                                        println!("Error: No value");
                                    }
                                },
                                _ => {}
                            }
                        }
                        elements.push(Element::Image(ImageData::new(
                            image_bytes,
                            title.to_string(),
                            alt.to_string(),
                            image_type,
                            align,
                            ImageDimension {width,height},
                        )));
                    }
                    "Hyperlink" => {
                        let mut url = "_";
                        let mut alt = "_";
                        let mut title = "_";
                        let mut size = 10;
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "size" => {
                                    if let Some(value) = &child.text {
                                        size = value.parse()?;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "url" => {
                                    if let Some(value) = &child.text {
                                        url = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "alt" => {
                                    if let Some(value) = &child.text {
                                        alt = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "title" => {
                                    if let Some(value) = &child.text {
                                        title = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        elements.push(Element::Hyperlink {
                            title: title.to_string(),
                            url: url.to_string(),
                            alt: alt.to_string(),
                            size: size,
                        });
                    }
                    "Header" => {
                        let mut text = "_";
                        let mut level = 0;
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "level" => {
                                    if let Some(value) = &child.text {
                                        level = value.parse()?;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "text" => {
                                    if let Some(value) = &child.text {
                                        text = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        elements.push(Element::Header { text: text.to_string(), level: level });
                    }
                    "Table" => {
                        let mut headers: Vec<TableHeader> = vec![];
                        let mut rows: Vec<TableRow> = vec![];
                        for table_element in element.children.iter() {
                            match table_element.name.as_str() {
                                "headers" => {
                                    for header in table_element.children.iter() {
                                        let mut header_content: TableHeader = {
                                            TableHeader {
                                                element: Element::Text {
                                                    text: "_".to_string(),
                                                    size: 10,
                                                },
                                                width: 8.0,
                                            }
                                        };
                                        match header.name.as_str() {
                                            "TableHeader" => {
                                                let mut text = "_";
                                                let mut size = 10;
                                                let mut width = 8.0;
                                                for table_header_element in header.children.iter() {
                                                    for table_header_element_group in table_header_element.children.iter() {
                                                        match
                                                            table_header_element_group.name.as_str()
                                                        {
                                                            "Text" => {
                                                                for table_header_element_sub in table_header_element_group.children.iter() {
                                                                    match
                                                                        table_header_element_sub.name.as_str()
                                                                    {
                                                                        "size" => {
                                                                            if
                                                                                let Some(value) =
                                                                                    &table_header_element_sub.text
                                                                            {
                                                                                size =
                                                                                    value.parse()?;
                                                                            } else {
                                                                                println!(
                                                                                    "Error: No value"
                                                                                );
                                                                            }
                                                                        }
                                                                        "text" => {
                                                                            if
                                                                                let Some(value) =
                                                                                    &table_header_element_sub.text
                                                                            {
                                                                                text = value;
                                                                            } else {
                                                                                println!(
                                                                                    "Error: No value"
                                                                                );
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                            }
                                                            "width" => {
                                                                if
                                                                    let Some(value) =
                                                                        &table_header_element_group.text
                                                                {
                                                                    width = value.parse()?;
                                                                } else {
                                                                    println!("Error: No value");
                                                                }
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                }
                                                header_content = TableHeader {
                                                    element: {
                                                        Element::Text {
                                                            text: text.to_string(),
                                                            size: size,
                                                        }
                                                    },
                                                    width: width,
                                                };
                                            }
                                            _ => {}
                                        }
                                        headers.push(header_content);
                                    }
                                }
                                "rows" => {
                                    for table_row in table_element.children.iter() {
                                        let mut row_content: TableRow = {
                                            TableRow { cells: vec![] }
                                        };
                                        for cells in table_row.children.iter() {
                                            let mut cells_content = vec![];
                                            for table_cell in cells.children.iter() {
                                                match table_cell.name.as_str() {
                                                    "TableCell" => {
                                                        let mut cell_content: TableCell =
                                                            TableCell {
                                                                element: Element::Text {
                                                                    text: "_".to_string(),
                                                                    size: 10,
                                                                },
                                                            };
                                                        for cell in table_cell.children.iter() {
                                                            for cell_element_sub in cell.children.iter() {
                                                                match
                                                                    cell_element_sub.name.as_str()
                                                                {
                                                                    "Text" => {
                                                                        let mut text = "_";
                                                                        let mut size = 10;
                                                                        for cell_element_item in cell_element_sub.children.iter() {
                                                                            match
                                                                                cell_element_item.name.as_str()
                                                                            {
                                                                                "size" => {
                                                                                    if
                                                                                        let Some(
                                                                                            value,
                                                                                        ) =
                                                                                            &cell_element_item.text
                                                                                    {
                                                                                        size =
                                                                                            value.parse()?;
                                                                                    } else {
                                                                                        println!(
                                                                                            "Error: No value"
                                                                                        );
                                                                                    }
                                                                                }
                                                                                "text" => {
                                                                                    if
                                                                                        let Some(
                                                                                            value,
                                                                                        ) =
                                                                                            &cell_element_item.text
                                                                                    {
                                                                                        text =
                                                                                            value;
                                                                                    } else {
                                                                                        println!(
                                                                                            "Error: No value"
                                                                                        );
                                                                                    }
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                        }
                                                                        cell_content = TableCell {
                                                                            element: Element::Text {
                                                                                text: text.to_string(),
                                                                                size: size,
                                                                            },
                                                                        };
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                        cells_content.push(cell_content);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            row_content = TableRow { cells: cells_content };
                                        }
                                        rows.push(row_content);
                                    }
                                }
                                _ => {}
                            }
                        }
                        elements.push(Element::Table { headers: headers, rows: rows });
                    }
                    "element" => {
                        elements = parse_element(element)?;
                    }
                    _ => {}
                }
            }
            Ok(elements)
        }

        fn list_parse_element(element_data: &Node) -> anyhow::Result<Vec<ListItem>> {
            let mut elements: Vec<ListItem> = vec![];
            for element in element_data.children.iter() {
                match element.name.as_str() {
                    "ListItem" => {
                        for child in element.children.iter() {
                            match child.name.as_str() {
                                "elements" => {
                                    for sub_child in child.children.iter() {
                                        match sub_child.name.as_str() {
                                            "Text" => {
                                                let mut text = "_";
                                                let mut size = 10;
                                                for child in sub_child.children.iter() {
                                                    match child.name.as_str() {
                                                        "size" => {
                                                            if let Some(value) = &child.text {
                                                                size = value.parse()?;
                                                            } else {
                                                                println!("Error: No value");
                                                            }
                                                        }
                                                        "text" => {
                                                            if let Some(value) = &child.text {
                                                                text = value;
                                                            } else {
                                                                println!("Error: No value");
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                let sub_element = Element::Text {
                                                    text: text.to_string(),
                                                    size: size,
                                                };
                                                elements.push(ListItem { element: sub_element });
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                "List" => {
                                    let mut numbered = false;
                                    let mut sub_elements: Vec<ListItem> = vec![];
                                    for sub_child in child.children.iter() {
                                        match sub_child.name.as_str() {
                                            "elements" => {
                                                sub_elements = list_parse_element(sub_child)?;
                                            }
                                            "numbered" => {
                                                if let Some(value) = &sub_child.text {
                                                    numbered = value == "true";
                                                } else {
                                                    println!("Error: No value");
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    elements.push(ListItem {
                                        element: Element::List {
                                            elements: sub_elements,
                                            numbered: numbered,
                                        },
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(elements)
        }

        let mut page_width = 210.0;
        let mut page_height = 297.0;
        let mut left_page_indent = 10.0;
        let mut right_page_indent = 10.0;
        let mut top_page_indent = 10.0;
        let mut bottom_page_indent = 10.0;
        let mut page_header: Vec<Element> = vec![];
        let mut page_footer: Vec<Element> = vec![];

        for child in element_data.unwrap().children.iter() {
            match child.name.as_str() {
                "page_width" => {
                    if let Some(value) = &child.text {
                        page_width = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "page_height" => {
                    if let Some(value) = &child.text {
                        page_height = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "left_page_indent" => {
                    if let Some(value) = &child.text {
                        left_page_indent = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "right_page_indent" => {
                    if let Some(value) = &child.text {
                        right_page_indent = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "top_page_indent" => {
                    if let Some(value) = &child.text {
                        top_page_indent = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "bottom_page_indent" => {
                    if let Some(value) = &child.text {
                        bottom_page_indent = value.parse()?;
                    } else {
                        println!("Error: No value");
                    }
                }
                "page_header" => {
                    for child_element in child.children.iter() {
                        let mut text = "_";
                        let mut size = 10;
                        for sub_child in child_element.children.iter() {
                            match sub_child.name.as_str() {
                                "size" => {
                                    if let Some(value) = &sub_child.text {
                                        size = value.parse()?;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "text" => {
                                    if let Some(value) = &sub_child.text {
                                        text = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        page_header.push(Element::Text { text: text.to_string(), size: size });
                    }
                }
                "page_footer" => {
                    for child_element in child.children.iter() {
                        let mut text = "_";
                        let mut size = 10;
                        for sub_child in child_element.children.iter() {
                            match sub_child.name.as_str() {
                                "size" => {
                                    if let Some(value) = &sub_child.text {
                                        size = value.parse()?;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                "text" => {
                                    if let Some(value) = &sub_child.text {
                                        text = value;
                                    } else {
                                        println!("Error: No value");
                                    }
                                }
                                _ => {}
                            }
                        }
                        page_footer.push(Element::Text { text: text.to_string(), size: size });
                    }
                }
                _ => {}
            }
        }

        Ok(Document {
            elements,
            page_width,
            page_height,
            left_page_indent,
            right_page_indent,
            top_page_indent,
            bottom_page_indent,
            page_header: page_header.clone(),
            page_footer: page_footer.clone(),
        })
    }

    fn generate(document: &Document) -> Result<Bytes> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new(&mut buffer);
        writer.write_event(
            Event::Decl(
                BytesDecl::from_start(
                    BytesStart::from_content("xml version=\"1.0\" encoding=\"UTF-8\"", 0)
                )
            )
        )?;
        writer.write_event(Event::Start(BytesStart::new("Document")))?;
        writer.write_event(Event::Start(BytesStart::new("elements")))?;

        fn serialize_element(element: &Element, writer: &mut Writer<&mut Vec<u8>>) -> Result<()> {
            match element {
                Element::Header { level, text } => {
                    writer.write_event(Event::Start(BytesStart::new("Header")))?;
                    writer.write_event(Event::Start(BytesStart::new("text")))?;
                    writer.write_event(Event::Text(BytesText::new(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("text")))?;
                    writer.write_event(Event::Start(BytesStart::new("level")))?;
                    writer.write_event(Event::Text(BytesText::new(&level.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("level")))?;
                    writer.write_event(Event::End(BytesEnd::new("Header")))?;
                }
                Element::Paragraph { elements } => {
                    writer.write_event(Event::Start(BytesStart::new("Paragraph")))?;
                    writer.write_event(Event::Start(BytesStart::new("elements")))?;
                    for sub_element in elements {
                        serialize_element(sub_element, writer)?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("elements")))?;
                    writer.write_event(Event::End(BytesEnd::new("Paragraph")))?;
                }
                Element::Text { text, size } => {
                    writer.write_event(Event::Start(BytesStart::new("Text")))?;
                    writer.write_event(Event::Start(BytesStart::new("text")))?;
                    writer.write_event(Event::Text(BytesText::new(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("text")))?;
                    writer.write_event(Event::Start(BytesStart::new("size")))?;
                    writer.write_event(Event::Text(BytesText::new(&size.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("size")))?;
                    writer.write_event(Event::End(BytesEnd::new("Text")))?;
                }
                Element::Image(image) => {
                    writer.write_event(Event::Start(BytesStart::new("Image")))?;
                    writer.write_event(Event::Start(BytesStart::new("title")))?;
                    writer.write_event(Event::Text(BytesText::new(image.title())))?;
                    writer.write_event(Event::End(BytesEnd::new("title")))?;
                    writer.write_event(Event::Start(BytesStart::new("alt")))?;
                    writer.write_event(Event::Text(BytesText::new(image.alt())))?;
                    writer.write_event(Event::End(BytesEnd::new("alt")))?;
                    writer.write_event(Event::Start(BytesStart::new("bytes")))?;
                    writer.write_event(
                        Event::Text(
                            BytesText::new(&String::from_utf8(image.bytes().to_vec()).unwrap().to_string())
                        )
                    )?;
                    writer.write_event(Event::End(BytesEnd::new("bytes")))?;
                    writer.write_event(Event::Start(BytesStart::new("image_type")))?;
                    writer.write_event(Event::Text(BytesText::new(&image.image_type().to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("image_type")))?;
                    writer.write_event(Event::End(BytesEnd::new("Image")))?;
                }
                Element::Hyperlink { title, url, alt, size } => {
                    writer.write_event(Event::Start(BytesStart::new("Hyperlink")))?;
                    writer.write_event(Event::Start(BytesStart::new("url")))?;
                    writer.write_event(Event::Text(BytesText::new(url)))?;
                    writer.write_event(Event::End(BytesEnd::new("url")))?;
                    writer.write_event(Event::Start(BytesStart::new("title")))?;
                    writer.write_event(Event::Text(BytesText::new(title)))?;
                    writer.write_event(Event::End(BytesEnd::new("title")))?;
                    writer.write_event(Event::Start(BytesStart::new("alt")))?;
                    writer.write_event(Event::Text(BytesText::new(alt)))?;
                    writer.write_event(Event::End(BytesEnd::new("alt")))?;
                    writer.write_event(Event::Start(BytesStart::new("size")))?;
                    writer.write_event(Event::Text(BytesText::new(&size.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("size")))?;
                    writer.write_event(Event::End(BytesEnd::new("Hyperlink")))?;
                }
                Element::List { elements, numbered } => {
                    writer.write_event(Event::Start(BytesStart::new("List")))?;
                    writer.write_event(Event::Start(BytesStart::new("elements")))?;
                    for sub_element in elements {
                        list_serialize_element(sub_element, writer)?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("elements")))?;
                    writer.write_event(Event::Start(BytesStart::new("numbered")))?;
                    writer.write_event(Event::Text(BytesText::new(&numbered.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("numbered")))?;
                    writer.write_event(Event::End(BytesEnd::new("List")))?;
                }
                Element::Table { headers, rows } => {
                    writer.write_event(Event::Start(BytesStart::new("Table")))?;
                    writer.write_event(Event::Start(BytesStart::new("headers")))?;
                    for header in headers {
                        writer.write_event(Event::Start(BytesStart::new("TableHeader")))?;
                        writer.write_event(Event::Start(BytesStart::new("element")))?;
                        match &header.element {
                            Element::Text { text, size } => {
                                writer.write_event(Event::Start(BytesStart::new("Text")))?;
                                writer.write_event(Event::Text(BytesText::new(&text)))?;
                                writer.write_event(Event::End(BytesEnd::new("Text")))?;
                                writer.write_event(Event::Start(BytesStart::new("Text")))?;
                                writer.write_event(Event::Text(BytesText::new(&size.to_string())))?;
                                writer.write_event(Event::End(BytesEnd::new("Text")))?;
                            }
                            _ => {}
                        }
                        writer.write_event(Event::End(BytesEnd::new("element")))?;
                        writer.write_event(Event::Start(BytesStart::new("width")))?;
                        writer.write_event(Event::Text(BytesText::new(&header.width.to_string())))?;
                        writer.write_event(Event::End(BytesEnd::new("width")))?;
                        writer.write_event(Event::End(BytesEnd::new("TableHeader")))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("headers")))?;
                    writer.write_event(Event::Start(BytesStart::new("rows")))?;
                    for row in rows {
                        writer.write_event(Event::Start(BytesStart::new("TableRow")))?;
                        writer.write_event(Event::Start(BytesStart::new("cells")))?;
                        for cell in &row.cells {
                            match cell {
                                TableCell { element } => {
                                    writer.write_event(Event::Start(BytesStart::new("TableCell")))?;
                                    writer.write_event(Event::Start(BytesStart::new("element")))?;
                                    match &element {
                                        Element::Text { text, size } => {
                                            writer.write_event(
                                                Event::Start(BytesStart::new("Text"))
                                            )?;
                                            writer.write_event(Event::Text(BytesText::new(&text)))?;
                                            writer.write_event(Event::End(BytesEnd::new("Text")))?;
                                            writer.write_event(
                                                Event::Start(BytesStart::new("Text"))
                                            )?;
                                            writer.write_event(
                                                Event::Text(BytesText::new(&size.to_string()))
                                            )?;
                                            writer.write_event(Event::End(BytesEnd::new("Text")))?;
                                        }
                                        _ => {}
                                    }
                                    writer.write_event(Event::End(BytesEnd::new("element")))?;
                                    writer.write_event(Event::End(BytesEnd::new("TableCell")))?;
                                }
                            }
                        }
                        writer.write_event(Event::End(BytesEnd::new("cells")))?;
                        writer.write_event(Event::End(BytesEnd::new("TableRow")))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("rows")))?;
                    writer.write_event(Event::End(BytesEnd::new("Table")))?;
                }
            }
            Ok(())
        }

        fn list_serialize_element(
            element: &ListItem,
            writer: &mut Writer<&mut Vec<u8>>
        ) -> Result<()> {
            match element {
                ListItem { element } => {
                    writer.write_event(Event::Start(BytesStart::new("ListItem")))?;
                    writer.write_event(Event::Start(BytesStart::new("element")))?;
                    serialize_element(element, writer)?;
                    writer.write_event(Event::End(BytesEnd::new("element")))?;
                    writer.write_event(Event::End(BytesEnd::new("ListItem")))?;
                }
            }
            Ok(())
        }

        for element in &document.elements {
            serialize_element(element, &mut writer)?;
        }
        writer.write_event(Event::End(BytesEnd::new("elements")))?;

        writer.write_event(Event::Start(BytesStart::new("page_width")))?;
        writer.write_event(Event::Text(BytesText::new(&document.page_width.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("page_width")))?;
        writer.write_event(Event::Start(BytesStart::new("page_height")))?;
        writer.write_event(Event::Text(BytesText::new(&document.page_height.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("page_height")))?;
        writer.write_event(Event::Start(BytesStart::new("left_page_indent")))?;
        writer.write_event(Event::Text(BytesText::new(&document.left_page_indent.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("left_page_indent")))?;
        writer.write_event(Event::Start(BytesStart::new("right_page_indent")))?;
        writer.write_event(Event::Text(BytesText::new(&document.right_page_indent.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("right_page_indent")))?;
        writer.write_event(Event::Start(BytesStart::new("top_page_indent")))?;
        writer.write_event(Event::Text(BytesText::new(&document.top_page_indent.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("top_page_indent")))?;
        writer.write_event(Event::Start(BytesStart::new("bottom_page_indent")))?;
        writer.write_event(Event::Text(BytesText::new(&document.bottom_page_indent.to_string())))?;
        writer.write_event(Event::End(BytesEnd::new("bottom_page_indent")))?;

        writer.write_event(Event::Start(BytesStart::new("page_header")))?;
        for page_header_element in document.page_header.iter() {
            match page_header_element {
                Element::Text { text, size } => {
                    writer.write_event(Event::Start(BytesStart::new("Text")))?;
                    writer.write_event(Event::Start(BytesStart::new("text")))?;
                    writer.write_event(Event::Text(BytesText::new(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("text")))?;
                    writer.write_event(Event::Start(BytesStart::new("size")))?;
                    writer.write_event(Event::Text(BytesText::new(&size.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("size")))?;
                    writer.write_event(Event::End(BytesEnd::new("Text")))?;
                }
                _ => {}
            }
        }
        writer.write_event(Event::End(BytesEnd::new("page_header")))?;

        writer.write_event(Event::Start(BytesStart::new("page_footer")))?;
        for page_footer_element in document.page_footer.iter() {
            match page_footer_element {
                Element::Text { text, size } => {
                    writer.write_event(Event::Start(BytesStart::new("Text")))?;
                    writer.write_event(Event::Start(BytesStart::new("text")))?;
                    writer.write_event(Event::Text(BytesText::new(text)))?;
                    writer.write_event(Event::End(BytesEnd::new("text")))?;
                    writer.write_event(Event::Start(BytesStart::new("size")))?;
                    writer.write_event(Event::Text(BytesText::new(&size.to_string())))?;
                    writer.write_event(Event::End(BytesEnd::new("size")))?;
                    writer.write_event(Event::End(BytesEnd::new("Text")))?;
                }
                _ => {}
            }
        }
        writer.write_event(Event::End(BytesEnd::new("page_footer")))?;
        writer.write_event(Event::End(BytesEnd::new("Document")))?;

        Ok(Bytes::from(buffer))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use bytes::Bytes;
    use crate::markdown;
    use crate::xml::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let path = "test/data/document.xml";
        let mut file = File::open(path).expect("Cannot open xml file");

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let bytes = Bytes::from(buffer);
        let parsed = Transformer::parse(&bytes)?;
        println!("{:#?}", parsed);
        let generated = markdown::Transformer::generate(&parsed)?;
        let mut file = File::create("test/data/generated.md")?;
        file.write_all(&generated)?;
        Ok(())
    }

    #[test]
    fn test_generate() -> anyhow::Result<()> {
        let path = "test/data/document.xml";
        let mut file = File::open(path).expect("Cannot open xml file");

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let bytes = Bytes::from(buffer);
        let parsed = Transformer::parse(&bytes)?;
        let generated = Transformer::generate(&parsed)?;
        println!("{:#?}", generated);
        // write to file
        let mut file = File::create("test/data/generated.xml")?;
        file.write_all(&generated)?;

        Ok(())
    }
}

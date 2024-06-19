use crate::core::Element::{Image, Paragraph, Table};
use crate::core::*;
use bytes::Bytes;
use std::collections::HashMap;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document>
    where
        Self: Sized,
    {
        let mut elements: Vec<Element> = vec![];
        let document: &str = std::str::from_utf8(document.as_ref())?;
        let lines = document.lines();
        let lines_vec: Vec<&str> = lines.collect();
        let mut i = 0;
        while i < lines_vec.len() {
            let line = lines_vec[i].to_string();
            elements.push(Element::Text {
                text: line,
                size: 8,
            });
            elements.push(Element::Text {
                text: "\n".to_string(),
                size: 8,
            });
            i += 1;
        }
        let new_paragraph = Paragraph { elements };
        Ok(Document::new(vec![new_paragraph]))
    }

    fn generate(document: &Document) -> anyhow::Result<Bytes>
    where
        Self: Sized,
    {
        let mut images: HashMap<String, Bytes> = HashMap::new();
        let mut image_num: i32 = 0;

        let mut markdown = String::new();
        fn generate_element(
            element: &Element,
            markdown: &mut String,
            list_depth: usize,
            list_counters: &mut Vec<usize>,
            list_types: &mut Vec<bool>,
            images: &mut HashMap<String, Bytes>,
            image_num: &mut i32,
        ) -> anyhow::Result<()> {
            fn generate_list_item(
                element: &ListItem,
                markdown: &mut String,
                list_depth: usize,
                list_counters: &mut Vec<usize>,
                list_types: &mut Vec<bool>,
                images: &mut HashMap<String, Bytes>,
                image_num: &mut i32,
            ) -> anyhow::Result<()> {
                let prefix = if *list_types.last().unwrap() {
                    let counter = list_counters.last_mut().unwrap();
                    if let Element::Text { .. } = element.element {
                        *counter += 1;
                    }
                    format!("{}. ", counter)
                } else {
                    "- ".to_string()
                };
                // println!("list depth: {}", list_depth);
                markdown.push_str(&"  ".repeat(list_depth - 1));
                if let Element::Text { .. } = element.element {
                    markdown.push_str(&prefix);
                }
                generate_element(
                    &element.element,
                    markdown,
                    list_depth,
                    list_counters,
                    list_types,
                    images,
                    image_num,
                )?;
                if let Element::Text { .. } = element.element {
                    markdown.push('\n');
                }
                Ok(())
            }

            match element {
                Element::Header { level: _, text } => {
                    markdown.push_str(text);
                    markdown.push('\n');
                    markdown.push('\n');
                }
                Element::Paragraph { elements } => {
                    for child in elements {
                        generate_element(
                            child,
                            markdown,
                            list_depth,
                            list_counters,
                            list_types,
                            images,
                            image_num,
                        )?;
                    }
                    markdown.push('\n');
                    markdown.push('\n');
                }
                Element::List { elements, numbered } => {
                    list_counters.push(0);
                    list_types.push(*numbered);
                    for item in elements {
                        generate_list_item(
                            item,
                            markdown,
                            list_depth + 1,
                            list_counters,
                            list_types,
                            images,
                            image_num,
                        )?;
                    }
                    list_counters.pop();
                    list_types.pop();

                    if list_counters.is_empty() {
                        markdown.push('\n');
                    }
                }
                Element::Text { text, size: _ } => {
                    markdown.push_str(text);
                    if !text.ends_with(' ') {
                        markdown.push(' ');
                    }
                }
                Element::Hyperlink {
                    title, url, alt, ..
                } => {
                    if url == alt && alt == url {
                        markdown.push_str(&url.to_string());
                    } else {
                        markdown.push_str(&format!("[{}]({} \"{}\")", title, url, alt));
                    }
                }
                Image(image) => {
                    let image_path = format!("image{}.png", image_num);
                    markdown.push_str(&format!(
                        "![{}]({} \"{}\") ({}x{})",
                        image.alt(),
                        image_path,
                        image.title(),
                        image.width(),
                        image.height()
                    ));
                    images.insert(image_path.to_string(), image.bytes().clone());
                    *image_num += 1;
                }
                Table { headers, rows } => {
                    let mut max_lengths: Vec<usize> = Vec::new();

                    for header in headers {
                        if let Element::Text { text, size: _ } = header.element.clone() {
                            max_lengths.push(text.len());
                        }
                    }
                    for row in rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            if let Element::Text { text, size: _ } = cell.element.clone() {
                                if cell_index < max_lengths.len() {
                                    max_lengths[cell_index] =
                                        max_lengths[cell_index].max(text.len());
                                }
                            }
                        }
                    }

                    for (index, header) in headers.iter().enumerate() {
                        if let Element::Text { text, size: _ } = header.element.clone() {
                            let padding = max_lengths[index] - text.len();
                            markdown.push_str("| ");
                            markdown.push_str(text.as_str());
                            markdown.push_str(&" ".repeat(padding));
                            markdown.push(' ');
                        }
                    }
                    markdown.push_str("|\n");

                    for max_length in &max_lengths {
                        markdown.push('|');
                        markdown.push_str(&"-".repeat(*max_length + 2));
                    }
                    markdown.push_str("|\n");

                    for row in rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            if let Element::Text { text, size: _ } = cell.element.clone() {
                                let padding = max_lengths[cell_index] - text.len();
                                markdown.push_str("| ");
                                markdown.push_str(text.as_str());
                                markdown.push_str(&" ".repeat(padding));
                                markdown.push(' ');
                            }
                        }
                        markdown.push_str("|\n");
                    }
                    markdown.push('\n');
                }
            }
            Ok(())
        }

        let mut list_counters: Vec<usize> = Vec::new();
        let mut list_types: Vec<bool> = Vec::new();

        for element in &document.page_header {
            generate_element(
                element,
                &mut markdown,
                0,
                &mut list_counters,
                &mut list_types,
                &mut images,
                &mut image_num,
            )?;
        }

        for element in &document.elements {
            generate_element(
                element,
                &mut markdown,
                0,
                &mut list_counters,
                &mut list_types,
                &mut images,
                &mut image_num,
            )?;
        }

        for element in &document.page_footer {
            generate_element(
                element,
                &mut markdown,
                0,
                &mut list_counters,
                &mut list_types,
                &mut images,
                &mut image_num,
            )?;
        }

        Ok(Bytes::from(markdown))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Element::Header;
    use crate::core::*;
    use crate::text::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"First header

1. List item 1
2. List item 2
3. List item 3

Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla

Second header

+-----------------+-----------------+
| Header 1        | Header 2        |
+-----------------+-----------------+
| Row 1, Column 1 | Row 1, Column 2 |
| Row 2, Column 1 | Row 2, Column 2 |
+-----------------+-----------------+"#;
        // println!("{:?}", document);
        let parsed = Transformer::parse(&document.as_bytes().into());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let mut parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let mut footer_elements = Vec::new();
        let mut header_elements = Vec::new();
        let header = Header {
            level: 0,
            text: std::string::String::from("page header string"),
        };
        let footer = Header {
            level: 0,
            text: std::string::String::from("page footer string"),
        };
        footer_elements.push(footer);
        header_elements.push(header);
        parsed_document.page_footer = footer_elements;
        parsed_document.page_header = header_elements;
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes)?;
        println!("{}", generated_text);
        Ok(())
    }
}

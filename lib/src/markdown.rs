use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use crate::core::*;
use bytes::Bytes;
use regex::Regex;
use std::collections::HashMap;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document>
    where
        Self: Sized,
    {
        let document_str = std::str::from_utf8(document)?;
        let mut elements: Vec<Element> = Vec::new();
        let lines = document_str.lines();
        let mut current_paragraph_elements: Vec<Element> = Vec::new();
        let mut in_table = false;
        let mut table_rows: Vec<TableRow> = Vec::new();
        let mut table_headers: Vec<TableHeader> = Vec::new();
        let mut current_list_stack: Vec<Vec<ListItem>> = Vec::new();
        let mut current_list_type_stack: Vec<bool> = Vec::new();
        let img_regex = Regex::new("!\\[.*?\\]\\(.*? \".*?\"\\)").unwrap();
        for line in lines {
            let indent_level = line.chars().take_while(|c| c.is_whitespace()).count() / 2;
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with('-')
                || trimmed_line.starts_with('+')
                || trimmed_line.starts_with('*')
            {
                let list_item_text = trimmed_line[1..].trim();
                let list_item = ListItem {
                    element: Text {
                        text: list_item_text.to_string(),
                        size: 8,
                    },
                };
                if current_list_stack.len() <= indent_level {
                    while current_list_stack.len() <= indent_level {
                        current_list_stack.push(vec![]);
                        current_list_type_stack.push(false);
                    }
                } else {
                    while current_list_stack.len() > indent_level + 1 {
                        let nested_items = current_list_stack.pop().unwrap();
                        let nested_list = List {
                            elements: nested_items,
                            numbered: current_list_type_stack.pop().unwrap(),
                        };
                        if let Some(parent_list) = current_list_stack.last_mut() {
                            parent_list.push(ListItem {
                                element: nested_list,
                            });
                        }
                    }
                }
                current_list_stack[indent_level].push(list_item);
                while current_list_stack.len() > indent_level + 1 {
                    let nested_items = current_list_stack.pop().unwrap();
                    let nested_list = List {
                        elements: nested_items,
                        numbered: current_list_type_stack.pop().unwrap(),
                    };
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItem {
                            element: nested_list,
                        });
                    }
                }
            } else if trimmed_line.chars().next().unwrap_or(' ').is_digit(10)
                && trimmed_line.chars().nth(1).unwrap_or(' ') == '.'
            {
                let list_item_text = trimmed_line.splitn(2, '.').nth(1).unwrap_or("").trim();
                let list_item = ListItem {
                    element: Text {
                        text: list_item_text.to_string(),
                        size: 8,
                    },
                };
                if current_list_stack.len() <= indent_level {
                    while current_list_stack.len() <= indent_level {
                        current_list_stack.push(vec![]);
                        current_list_type_stack
                            .push(trimmed_line.chars().next().unwrap().is_digit(10));
                    }
                } else {
                    while current_list_stack.len() > indent_level + 1 {
                        let nested_items = current_list_stack.pop().unwrap();
                        let nested_list = List {
                            elements: nested_items,
                            numbered: current_list_type_stack.pop().unwrap(),
                        };
                        if let Some(parent_list) = current_list_stack.last_mut() {
                            parent_list.push(ListItem {
                                element: nested_list,
                            });
                        }
                    }
                }
                current_list_stack[indent_level].push(list_item);
                while current_list_stack.len() > indent_level + 1 {
                    let nested_items = current_list_stack.pop().unwrap();
                    let nested_list = List {
                        elements: nested_items,
                        numbered: current_list_type_stack.pop().unwrap(),
                    };
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItem {
                            element: nested_list,
                        });
                    }
                }
            } else {
                if line.starts_with('|') && line.ends_with('|') {
                    if !in_table {
                        // Start a new table
                        in_table = true;
                        table_headers = line
                            .split('|')
                            .filter(|x| !x.trim().is_empty() && !x.contains('-'))
                            .map(|header| TableHeader {
                                element: Text {
                                    text: header.trim().to_string(),
                                    size: 8,
                                },
                                width: 10.0,
                            })
                            .collect();
                    } else if line.contains("---") {
                        continue;
                    } else if line.starts_with('|') && in_table {
                        let cells = line
                            .split('|')
                            .filter(|x| !x.trim().is_empty())
                            .map(|cell| TableCell {
                                element: Text {
                                    text: cell.trim().to_string(),
                                    size: 8,
                                },
                            })
                            .collect();
                        table_rows.push(TableRow { cells });
                    }
                } else {
                    if in_table {
                        elements.push(Table {
                            headers: table_headers.clone(),
                            rows: table_rows.clone(),
                        });
                        table_rows.clear();
                        table_headers.clear();
                        in_table = false;
                    }

                    if line.starts_with('#') {
                        let level = line.chars().take_while(|&c| c == '#').count() as u8;
                        let text = line.trim_start_matches('#').trim();
                        let header = Header {
                            text: text.to_string(),
                            level,
                        };
                        if !current_paragraph_elements.is_empty() {
                            elements.push(Paragraph {
                                elements: current_paragraph_elements.clone(),
                            });
                            current_paragraph_elements.clear();
                        }
                        elements.push(header);
                    } else if !line.trim().is_empty() {
                        fn parser_text(
                            text: &str,
                            current_paragraph_elements: &mut Vec<Element>,
                        ) -> anyhow::Result<()> {
                            let mut start = 0;
                            let mut captures = 0;
                            let hyperlink_regex = Regex::new("(?:\\[([^\\]]+)\\])?(?:\\(|)(http[s]?:\\/\\/[^\\s\\)]+)(?:\\s\"([^\"]+)\")?\\)?").unwrap();
                            for cap in hyperlink_regex.captures_iter(text) {
                                captures += 1;
                                let (hyperlink_start, end) =
                                    (cap.get(0).unwrap().start(), cap.get(0).unwrap().end());
                                let markdown = &text[hyperlink_start..end];
                                if hyperlink_start > start {
                                    parser_text(
                                        &text[start..hyperlink_start],
                                        current_paragraph_elements,
                                    )?;
                                }
                                if markdown.starts_with("h") {
                                    current_paragraph_elements.push(Hyperlink {
                                        title: markdown.to_string(),
                                        url: markdown.to_string(),
                                        alt: markdown.to_string(),
                                    });
                                } else if markdown.starts_with("[") && markdown.ends_with("\")") {
                                    let start_alt_text = markdown.find("[").unwrap() + 1;
                                    let end_alt_text = markdown.find("]").unwrap();
                                    let start_url_path = markdown.find("(").unwrap() + 1;
                                    let end_url_path = markdown.find(" ").unwrap();
                                    let start_title = markdown.find("\"").unwrap() + 1;
                                    let end_title = markdown.rfind("\"").unwrap();
                                    let alt_text = &markdown[start_alt_text..end_alt_text];
                                    let url = &markdown[start_url_path..end_url_path];
                                    let title = &markdown[start_title..end_title];
                                    current_paragraph_elements.push(Hyperlink {
                                        title: alt_text.to_string(),
                                        url: url.to_string(),
                                        alt: title.to_string(),
                                    });
                                } else {
                                    let start_alt_text = markdown.find("[").unwrap() + 1;
                                    let end_alt_text = markdown.find("]").unwrap();
                                    let start_url_path = markdown.find("(").unwrap() + 1;
                                    let alt_text = &markdown[start_alt_text..end_alt_text];
                                    let url = &markdown[start_url_path..markdown.len() - 1];
                                    current_paragraph_elements.push(Hyperlink {
                                        title: alt_text.to_string(),
                                        url: url.to_string(),
                                        alt: alt_text.to_string(),
                                    });
                                }
                                start = end;
                            }
                            if captures == 0 {
                                current_paragraph_elements.push(Text {
                                    text: text.to_string(),
                                    size: 8,
                                });
                            } else {
                                if start < text.len() {
                                    parser_text(&text[start..], current_paragraph_elements)?;
                                }
                            }
                            Ok(())
                        }

                        let mut captures = 0;
                        let mut start = 0;

                        for cap in img_regex.captures_iter(line) {
                            captures += 1;
                            let (img_start, img_end) =
                                (cap.get(0).unwrap().start(), cap.get(0).unwrap().end());
                            let markdown = &line[img_start..img_end];
                            let start_alt_text = markdown.find("[").unwrap() + 1;
                            let end_alt_text = markdown.find("]").unwrap();
                            let start_file_path = markdown.find("(").unwrap() + 1;
                            let start_title = markdown.find("\"").unwrap() + 1;
                            let end_title = markdown.rfind("\"").unwrap();
                            let alt_text = &markdown[start_alt_text..end_alt_text];
                            let file_path = &markdown[start_file_path..end_title - 1];
                            let title = &markdown[start_title..end_title];

                            if img_start > start {
                                parser_text(
                                    &line[start..img_start],
                                    &mut current_paragraph_elements,
                                )?;
                            }
                            let image_empty = Bytes::new();
                            let image_bytes = images.get(file_path).map_or(&image_empty, |x| x);
                            current_paragraph_elements.push(Image {
                                bytes: image_bytes.clone(),
                                title: title.to_string(),
                                alt: alt_text.to_string(),
                                image_type: ImageType::Png,
                            });

                            start = img_end;
                        }

                        if captures == 0 {
                            parser_text(&line, &mut current_paragraph_elements)?;
                        } else {
                            if start < line.len() {
                                parser_text(&line[start..], &mut current_paragraph_elements)?;
                            }
                        }
                    } else if !current_paragraph_elements.is_empty() {
                        elements.push(Paragraph {
                            elements: current_paragraph_elements.clone(),
                        });
                        current_paragraph_elements.clear();
                    }
                }
                while !current_list_stack.is_empty() {
                    let items = current_list_stack.pop().unwrap();
                    let list = List {
                        elements: items,
                        numbered: current_list_type_stack.pop().unwrap(),
                    };
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItem { element: list });
                    } else {
                        elements.push(list);
                    }
                }
            }
        }
        while !current_list_stack.is_empty() {
            let items = current_list_stack.pop().unwrap();
            let list = List {
                elements: items,
                numbered: current_list_type_stack.pop().unwrap(),
            };
            if let Some(parent_list) = current_list_stack.last_mut() {
                parent_list.push(ListItem { element: list });
            } else {
                elements.push(list);
            }
        }

        if !current_paragraph_elements.is_empty() {
            elements.push(Paragraph {
                elements: current_paragraph_elements,
            });
        }

        if in_table {
            elements.push(Table {
                headers: table_headers,
                rows: table_rows,
            });
        }

        Ok(Document::new(elements))
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)>
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

                    if let Text { .. } = element.element {
                        *counter += 1;
                    }
                    format!("{}. ", counter)
                } else {
                    "- ".to_string()
                };
                // println!("list depth: {}", list_depth);
                markdown.push_str(&"  ".repeat(list_depth - 1));
                if let Text { .. } = element.element {
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
                if let Text { .. } = element.element {
                    markdown.push('\n');
                }
                Ok(())
            }

            match element {
                Header { level, text } => {
                    markdown.push_str(&"#".repeat(level.clone() as usize));
                    markdown.push(' ');
                    markdown.push_str(text);
                    markdown.push('\n');
                    markdown.push('\n');
                }
                Paragraph { elements } => {
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
                List { elements, numbered } => {
                    list_counters.push(0);
                    list_types.push(*numbered);
                    for item in elements {
                        generate_list_item(
                            &item,
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

                    if list_counters.len() == 0 {
                        markdown.push('\n');
                    }
                }
                Text { text, size: _ } => {
                    let re = Regex::new(
                        r#"^(\n)*\s+$?"#,
                    )?;
                    if !re.is_match(&text) {
                        markdown.push_str("\n");
                        markdown.push_str(text);
                        if !text.ends_with(" ") {
                            markdown.push_str(" ");
                        }
                        markdown.push_str("\n");
                    }
                }
                Hyperlink { title, url, alt } => {
                    if url == alt && alt == url {
                        markdown.push_str(&format!("{}", url));
                    } else {
                        markdown.push_str(&format!("[{}]({} \"{}\")", title, url, alt));
                    }
                }
                Image {
                    bytes,
                    title,
                    alt,
                    image_type: _,
                } => {
                    let image_path = format!("image{}.png", image_num);
                    markdown.push_str(&format!("![{}]({} \"{}\")", alt, image_path, title));
                    images.insert(image_path.to_string(), bytes.clone());
                    *image_num += 1;
                }
                Table { headers, rows } => {
                    let mut max_lengths: Vec<usize> = Vec::new();

                    for header in headers {
                        if let Text { text, .. } = header.element.clone() {
                            max_lengths.push(text.len());
                        }
                    }
                    for row in rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            if let Text { text, .. } = cell.element.clone() {
                                if cell_index < max_lengths.len() {
                                    max_lengths[cell_index] =
                                        max_lengths[cell_index].max(text.len());
                                }
                            }
                        }
                    }

                    for (index, header) in headers.iter().enumerate() {
                        // let header_text = header.element.text_as_ref()?;
                        if let Text { text, .. } = header.element.clone() {
                            let padding = max_lengths[index] - text.len();
                            markdown.push_str("| ");
                            markdown.push_str(text.as_str());
                            markdown.push_str(&" ".repeat(padding));
                            markdown.push(' ');
                        }
                    }
                    markdown.push_str("|\n");

                    for max_length in &max_lengths {
                        markdown.push_str("|");
                        markdown.push_str(&"-".repeat(*max_length + 2));
                    }
                    markdown.push_str("|\n");

                    for row in rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            if let Text { text, .. } = cell.element.clone() {
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
        let all_elements: Vec<Element> = document
            .page_header
            .iter()
            .cloned()
            .chain(document.elements.iter().cloned())
            .chain(document.page_footer.iter().cloned())
            .collect();
        for element in &all_elements {
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

        Ok((Bytes::from(markdown), HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::markdown::*;
    use crate::text;
    use crate::pdf;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"# First header

Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla

1. List item 1
2. List item 2
3. List item 3
   1. List item secode level 1
   2. List item secode level 2
4. List item 4
   1. List item secode level 3
   2. List item secode level 4
5. List item 5
   1. List item secode level 5

- List item one
  - List item two
- List item three
  - List item four
  - List item five
    - List item zzz
- List item six
  - List item seven

![Picture alt1](test/data/picture.png "Picture title1")


Bla bla bla ![Picture alt2](test/data/picture.png "Picture title2") bla. http://example.com  [Example](http://example.com) [Example](http://example.com "Example tooltip")


## Second header

| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |

Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
        // println!("{:?}", document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        images.insert("test/data/picture.png".to_string(), image_bytes);
        let parsed = Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);
        println!("==========================");
        let generated_result = text::Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);

        let generated_result = pdf::Transformer::generate(&parsed_document)?;
        std::fs::write("test/data/generated.pdf",generated_result.0)?;

        Ok(())
    }
}

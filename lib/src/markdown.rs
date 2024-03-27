use std::collections::HashMap;
use bytes::Bytes;
use regex::Regex;
use crate::core::*;



pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document> where Self: Sized {
        let document_str = std::str::from_utf8(document)?;
        let mut elements: Vec<Box<dyn Element>> = Vec::new();
        let lines = document_str.lines();
        let mut current_paragraph_elements: Vec<Box<dyn Element>> = Vec::new();
        let mut in_table = false;
        let mut table_rows: Vec<TableRowElement> = Vec::new();
        let mut table_headers: Vec<TableHeaderElement> = Vec::new();
        let mut current_list_stack: Vec<Vec<ListItemElement>> = Vec::new();
        let mut current_list_type_stack: Vec<bool> = Vec::new();
        let img_regex = Regex::new("!\\[.*?\\]\\(.*? \".*?\"\\)").unwrap();
        for line in lines {
            let indent_level = line.chars().take_while(|c| c.is_whitespace()).count() / 2;
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with('-') || trimmed_line.starts_with('+') || trimmed_line.starts_with('*') {
                let list_item_text = trimmed_line[1..].trim();
                let list_item = ListItemElement::new(&TextElement::new(list_item_text,8)?).unwrap();
                if current_list_stack.len() <= indent_level {
                    while current_list_stack.len() <= indent_level {
                        current_list_stack.push(vec![]);
                        current_list_type_stack.push(false);
                    }
                } else {
                    while current_list_stack.len() > indent_level + 1 {
                        let nested_items = current_list_stack.pop().unwrap();
                        let nested_list = ListElement::new(&nested_items, current_list_type_stack.pop().unwrap())?;
                        if let Some(parent_list) = current_list_stack.last_mut() {
                            parent_list.push(ListItemElement::new(&nested_list)?);
                        }
                    }
                }
                current_list_stack[indent_level].push(list_item);
                while current_list_stack.len() > indent_level + 1 {
                    let nested_items = current_list_stack.pop().unwrap();
                    let nested_list = ListElement::new(&nested_items, current_list_type_stack.pop().unwrap())?;
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItemElement::new(&nested_list)?);
                    }
                }
            } else if trimmed_line.chars().next().unwrap_or(' ').is_digit(10) && trimmed_line.chars().nth(1).unwrap_or(' ') == '.' {
                let list_item_text = trimmed_line.splitn(2, '.').nth(1).unwrap_or("").trim();
                let list_item = ListItemElement::new(&TextElement::new(list_item_text, 8)?).unwrap();
                if current_list_stack.len() <= indent_level {
                    while current_list_stack.len() <= indent_level {
                        current_list_stack.push(vec![]);
                        current_list_type_stack.push(trimmed_line.chars().next().unwrap().is_digit(10));
                    }
                } else {
                    while current_list_stack.len() > indent_level + 1 {
                        let nested_items = current_list_stack.pop().unwrap();
                        let nested_list = ListElement::new(&nested_items, current_list_type_stack.pop().unwrap())?;
                        if let Some(parent_list) = current_list_stack.last_mut() {
                            parent_list.push(ListItemElement::new(&nested_list)?);
                        }
                    }
                }
                current_list_stack[indent_level].push(list_item);
                while current_list_stack.len() > indent_level + 1 {
                    let nested_items = current_list_stack.pop().unwrap();
                    let nested_list = ListElement::new(&nested_items, current_list_type_stack.pop().unwrap())?;
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItemElement::new(&nested_list)?);
                    }
                }

            } else {
                    if line.starts_with('|')  && line.ends_with('|') {
                        if !in_table {
                            // Start a new table
                            in_table = true;
                            table_headers = line.split('|')
                                .filter(|x| !x.trim().is_empty() && !x.contains('-'))
                                .map(|header| TableHeaderElement::new(&TextElement::new(header.trim(),8).unwrap()).unwrap())
                                .collect();
                        } else if line.contains("---") {
                            continue;
                        } else if line.starts_with('|') && in_table {
                            let cells = line.split('|')
                                .filter(|x| !x.trim().is_empty())
                                .map(|cell| TableCellElement::new(&TextElement::new(cell.trim(),8).unwrap()).unwrap())
                                .collect();
                            table_rows.push(TableRowElement::new(&cells).unwrap());
                        }
                    }  else {
                        if in_table {
                            elements.push(TableElement::new(&table_headers, &table_rows)?);
                            table_rows.clear();
                            table_headers.clear();
                            in_table = false;
                        }

                        if line.starts_with('#') {
                            let level = line.chars().take_while(|&c| c == '#').count() as u8;
                            let text = line.trim_start_matches('#').trim();
                            let header = HeaderElement::new(text, level)?;
                            if !current_paragraph_elements.is_empty() {
                                elements.push(ParagraphElement::new(&current_paragraph_elements)?);
                                current_paragraph_elements.clear();
                            }
                            elements.push(header);
                        } else if !line.trim().is_empty() {
                            fn parser_text(text: &str, current_paragraph_elements: &mut Vec<Box<dyn Element>>) ->  anyhow::Result<()> {
                                let mut start = 0;
                                let mut captures = 0;
                                let hyperlink_regex = Regex::new("(?:\\[([^\\]]+)\\])?(?:\\(|)(http[s]?:\\/\\/[^\\s\\)]+)(?:\\s\"([^\"]+)\")?\\)?").unwrap();
                                for cap in hyperlink_regex.captures_iter(text) {
                                    captures += 1;
                                    let (hyperlink_start, end) = (cap.get(0).unwrap().start(), cap.get(0).unwrap().end());
                                    let markdown = &text[hyperlink_start..end];
                                    if hyperlink_start > start {
                                        parser_text(&text[start..hyperlink_start], current_paragraph_elements)?;
                                    }
                                    if markdown.starts_with("h") {
                                        current_paragraph_elements.push(HyperlinkElement::new(markdown, markdown, markdown)?);
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
                                        current_paragraph_elements.push(HyperlinkElement::new(alt_text, url, title)?);
                                    } else {
                                        let start_alt_text = markdown.find("[").unwrap() + 1;
                                        let end_alt_text = markdown.find("]").unwrap();
                                        let start_url_path = markdown.find("(").unwrap() + 1;
                                        let alt_text = &markdown[start_alt_text..end_alt_text];
                                        let url = &markdown[start_url_path..markdown.len()-1];
                                        current_paragraph_elements.push(HyperlinkElement::new(alt_text, url, alt_text)?);

                                    }
                                    start = end;
                                }
                                if captures == 0 {
                                    current_paragraph_elements.push(TextElement::new(text,8)?);
                                } else {
                                    if start < text.len() {
                                        parser_text(&text[start..], current_paragraph_elements)?;
                                    }
                                }
                                Ok(())
                            }

                            let  mut captures= 0;
                            let mut start = 0;

                            for cap in img_regex.captures_iter(line) {
                                captures += 1;
                                let (img_start, img_end) = (cap.get(0).unwrap().start(), cap.get(0).unwrap().end());
                                let markdown = &line[img_start..img_end];
                                let start_alt_text = markdown.find("[").unwrap() + 1;
                                let end_alt_text = markdown.find("]").unwrap();
                                let start_file_path = markdown.find("(").unwrap() + 1;
                                let start_title = markdown.find("\"").unwrap() + 1;
                                let end_title = markdown.rfind("\"").unwrap();
                                let alt_text = &markdown[start_alt_text..end_alt_text];
                                let file_path = &markdown[start_file_path..end_title-1];
                                let title = &markdown[start_title..end_title];

                                if img_start > start {
                                    parser_text(&line[start..img_start], &mut current_paragraph_elements)?;
                                }
                                let image_empty = Bytes::new();
                                let image_bytes = images.get(file_path).map_or(&image_empty, |x| x);
                                current_paragraph_elements.push(ImageElement::new(image_bytes, &title, &alt_text, ImageType::Png)?);

                                start = img_end;
                            }

                            if captures ==0 {
                                parser_text(&line, &mut current_paragraph_elements)?;

                            } else {
                                if start < line.len() {
                                    parser_text(&line[start..], &mut current_paragraph_elements)?;
                                }
                            }


                        } else if !current_paragraph_elements.is_empty() {
                            elements.push(ParagraphElement::new(&current_paragraph_elements)?);
                            current_paragraph_elements.clear();
                        }
                    }
                while !current_list_stack.is_empty() {
                    let items = current_list_stack.pop().unwrap();
                    let list = ListElement::new(&items, current_list_type_stack.pop().unwrap())?;
                    if let Some(parent_list) = current_list_stack.last_mut() {
                        parent_list.push(ListItemElement::new(&list)?);
                    } else {
                        elements.push(list);
                    }
                }
            }
        }
        while !current_list_stack.is_empty() {
            let items = current_list_stack.pop().unwrap();
            let list = ListElement::new(&items, current_list_type_stack.pop().unwrap())?;
            if let Some(parent_list) = current_list_stack.last_mut() {
                parent_list.push(ListItemElement::new(&list)?);
            } else {
                elements.push(list);
            }
        }

        if !current_paragraph_elements.is_empty() {
            elements.push(ParagraphElement::new(&current_paragraph_elements)?);
        }

        if in_table {
            elements.push(TableElement::new(&table_headers, &table_rows)?);
        }

        Ok(Document { elements })
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> where Self: Sized {

        let mut images:HashMap<String, Bytes> = HashMap::new();
        let mut image_num:i32 = 0;

        let mut markdown = String::new();
        fn generate_element(element: &Box<dyn Element>, markdown: &mut String, list_depth: usize, list_counters: &mut Vec<usize>, list_types: &mut Vec<bool>, images: &mut HashMap<String, Bytes>, image_num: &mut i32) -> anyhow::Result<()> {
            fn generate_list_item(element:&ListItemElement, markdown: &mut String, list_depth: usize, list_counters: &mut Vec<usize>, list_types: &mut Vec<bool>,  images: &mut HashMap<String, Bytes>, image_num: &mut i32) -> anyhow::Result<()> {
                let prefix = if *list_types.last().unwrap() {
                    let counter = list_counters.last_mut().unwrap();
                    if &element.element.element_type() == &ElementType::Text {
                        *counter += 1;
                    }
                    format!("{}. ", counter)
                } else {
                    "- ".to_string()
                };
                // println!("list depth: {}", list_depth);
                markdown.push_str(&"  ".repeat(list_depth-1));
                if &element.element.element_type() == &ElementType::Text {
                    markdown.push_str(&prefix);
                }
                generate_element(&element.element, markdown, list_depth, list_counters, list_types, images, image_num)?;
                if &element.element.element_type() == &ElementType::Text{
                    markdown.push('\n');
                }
                Ok(())
            }

            match element.element_type() {
                ElementType::Header => {
                    let header = HeaderElement::from(element)?;
                    markdown.push_str(&"#".repeat(header.level as usize));
                    markdown.push(' ');
                    markdown.push_str(&header.text);
                    markdown.push('\n');
                    markdown.push('\n');
                },
                ElementType::Paragraph => {
                    let paragraph = ParagraphElement::from(element)?;
                    for child in &paragraph.elements {
                        generate_element(child, markdown, list_depth, list_counters, list_types, images, image_num)?;
                    }
                    markdown.push('\n');
                    markdown.push('\n');
                },
                ElementType::List => {
                    let list = ListElement::from(element)?;
                    // println!("{:?}", list);

                    list_counters.push(0);
                    list_types.push(list.numbered);
                    for item in &list.elements {
                        generate_list_item(&item, markdown, list_depth + 1, list_counters, list_types, images, image_num)?;
                    }
                    list_counters.pop();
                    list_types.pop();

                    if list_counters.len() == 0 {
                        markdown.push('\n');
                    }

                },
                ElementType::Text => {
                    let text = TextElement::from(element)?;
                    markdown.push_str(&text.text);
                    if !text.text.ends_with(" ") {
                        markdown.push_str(" ");
                    }
                },
                ElementType::Hyperlink => {
                    let hyperlink = HyperlinkElement::from(element)?;
                    if hyperlink.url ==hyperlink.alt && hyperlink.alt == hyperlink.url {
                        markdown.push_str(&format!("{}", hyperlink.url));
                    } else {
                        markdown.push_str(&format!("[{}]({} \"{}\")", hyperlink.title, hyperlink.url, hyperlink.alt));
                    }

                },
                ElementType::Image => {
                    let image = ImageElement::from(element)?;
                    let image_path = format!("image{}.png", image_num);
                    markdown.push_str(&format!(
                        "![{}]({} \"{}\")",
                        image.alt, image_path, image.title
                    ));
                    images.insert(image_path.to_string(), image.bytes.clone());
                    *image_num += 1;
                }
                ElementType::Table => {
                    let table = TableElement::from(element)?;

                    let mut max_lengths: Vec<usize> = Vec::new();

                    for header in &table.headers {
                        let header_text = TextElement::from(&header.element)?;
                        max_lengths.push(header_text.text.len());
                    }
                    for row in &table.rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            let cell_text = TextElement::from(&cell.element)?;
                            if cell_index < max_lengths.len() {
                                max_lengths[cell_index] = max_lengths[cell_index].max(cell_text.text.len());
                            }
                        }
                    }

                    for (index, header) in table.headers.iter().enumerate() {
                        let header_text = TextElement::from(&header.element)?;
                        let padding = max_lengths[index] - header_text.text.len();
                        markdown.push_str("| ");
                        markdown.push_str(&header_text.text);
                        markdown.push_str(&" ".repeat(padding));
                        markdown.push(' ');
                    }
                    markdown.push_str("|\n");

                    for max_length in &max_lengths {
                        markdown.push_str("|");
                        markdown.push_str(&"-".repeat(*max_length + 2));
                    }
                    markdown.push_str("|\n");

                    for row in &table.rows {
                        for (cell_index, cell) in row.cells.iter().enumerate() {
                            let cell_text = TextElement::from(&cell.element)?;
                            let padding = max_lengths[cell_index] - cell_text.text.len();
                            markdown.push_str("| ");
                            markdown.push_str(&cell_text.text);
                            markdown.push_str(&" ".repeat(padding));
                            markdown.push(' ');
                        }
                        markdown.push_str("|\n");
                    }
                    markdown.push('\n');
                },
                _ =>  {

                },
            }
            Ok(())
        }

        let mut list_counters: Vec<usize> = Vec::new();
        let mut list_types: Vec<bool> = Vec::new();
        for element in &document.elements {
            generate_element(element, &mut markdown, 0, &mut list_counters,&mut list_types, &mut images, &mut image_num)?;
        }


        Ok((Bytes::from(markdown), HashMap::new()))
    }
}




#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::markdown::*;
    use crate::text;

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
        let parsed =  Transformer::parse(&document.as_bytes().into(), &HashMap::new());
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

        Ok(())

    }
}

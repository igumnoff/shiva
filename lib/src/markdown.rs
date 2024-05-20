use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use crate::core::*;
use bytes::Bytes;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, TextMergeStream};
use regex::Regex;
use std::collections::HashMap;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document>
    where
        Self: Sized,
    {
    
        fn process_element_creation(
            current_element: &mut Option<Element>,
            el: Element,
            list_depth: i32,
        ) {
            match current_element {
                Some(element) => match element {
                    Element::List { elements, numbered } => {
                        let mut li_vec_to_insert = elements;
                        for _ in 1..list_depth {
                            let last_index = li_vec_to_insert.len() - 1;
                            if let Element::List {
                                elements: ref mut inner_els,
                                numbered,
                            } = li_vec_to_insert[last_index].element
                            {
                                li_vec_to_insert = inner_els;
                            } else {
                                panic!("Expected a nested list structure at the specified depth");
                            }
                        }

                        match &el {
                            Hyperlink {
                                alt,
                                size,
                                title,
                                url,
                            } => {
                                if let Some(ListItem { element }) = li_vec_to_insert.last() {
                                    if let Text { text, size } = element {
                                        li_vec_to_insert.pop();
                                    }
                                }
                            }
                            _ => {}
                        }

                        let li = ListItem { element: el };
                        li_vec_to_insert.push(li);
                    }
                    _ => {}
                },
                None => {
                    *current_element = Some(el);
                }
            }
        }

        let document_str = std::str::from_utf8(document)?;
        let mut elements: Vec<Element> = Vec::new();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        options.insert(Options::ENABLE_MATH);
        options.insert(Options::ENABLE_GFM);

        let parser = Parser::new_ext(document_str, options);
        let md_iterator = TextMergeStream::new(parser);

        let mut list_depth = 0;
        let mut current_element: Option<Element> = None;

        let mut table_element: Option<(bool, Element)> = None;
        for event in md_iterator {
            // println!("Event - {:?}", event);
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {
                            process_element_creation(
                                &mut current_element,
                                Element::Paragraph { elements: vec![] },
                                list_depth,
                            );
                        }
                        Tag::Heading {
                            level,
                            id,
                            classes,
                            attrs,
                        } => {
                            let level = match level {
                                HeadingLevel::H1 => 1,
                                HeadingLevel::H2 => 2,
                                HeadingLevel::H3 => 3,
                                HeadingLevel::H4 => 4,
                                HeadingLevel::H5 => 5,
                                HeadingLevel::H6 => 6,
                            };
                            process_element_creation(
                                &mut current_element,
                                Element::Header {
                                    level,
                                    text: "".to_string(),
                                },
                                list_depth,
                            );
                        }
                        Tag::List(numbered) => {
                            let numbered = match numbered {
                                Some(_) => true,
                                None => false,
                            };

                            let list_el = List {
                                elements: vec![],
                                numbered,
                            };

                            process_element_creation(&mut current_element, list_el, list_depth);
                            list_depth += 1;
                        }
                        Tag::Item => {
                            let list_li = Text {
                                text: "".to_string(),
                                size: 14,
                            };

                            process_element_creation(&mut current_element, list_li, list_depth);
                        }
                        Tag::Table(_) => {
                            let table_el = Table {
                                headers: vec![],
                                rows: vec![],
                            };

                            table_element = Some((false, table_el));
                        }
                        Tag::TableHead => {
                            if let Some(table) = table_element.as_mut() {
                                table.0 = true;
                            }
                        }
                        Tag::Image {
                            link_type,
                            dest_url,
                            title,
                            id,
                        } => {
                            let img_type = dest_url.to_string();
                            let img_type = img_type.split('.').last().unwrap();
                            let image_type = match img_type {
                                "png" => ImageType::Png,
                                "jpeg" => ImageType::Jpeg,
                                _ => ImageType::Png,
                            };

                            //TODO: better to write helper to determine whether to download image or read locally base on dest_url
                            let bytes = images.get(&dest_url.to_string()).unwrap().clone();
                            let img_el = Element::Image {
                                bytes,
                                title: title.to_string(),
                                alt: title.to_string(),
                                image_type,
                            };
                            // Before image there is paragraph tag (likely because alt text is in paragraph )
                            current_element = None;
                            process_element_creation(&mut current_element, img_el, list_depth);
                        }
                        Tag::Link {
                            link_type,
                            dest_url,
                            title,
                            id,
                        } => {
                            let link_element = Hyperlink {
                                title: title.to_string(),
                                url: dest_url.to_string(),
                                alt: "alt".to_string(),
                                size: 14,
                            };
                            process_element_creation(
                                &mut current_element,
                                link_element,
                                list_depth,
                            );
                        }

                        rest => {
                            // println!("The tag parsing is not implemented {:#?}", rest);
                        }
                    }
                }
                Event::Text(text) => {
                    if let Some(curr_el) = current_element.as_mut() {
                        match curr_el {
                            Element::Paragraph { ref mut elements } => {
                                elements.push(Element::Text {
                                    text: text.to_string(),
                                    size: 14,
                                })
                            }
                            Element::Header {
                                level,
                                text: el_text,
                            } => {
                                el_text.push_str(&text);
                            }
                            Element::List { elements, numbered } => {
                                let mut li_vec_to_insert = elements;

                                for _ in 1..list_depth {
                                    let last_index = li_vec_to_insert.len() - 1;
                                    if let Element::List {
                                        elements: ref mut inner_els,
                                        numbered,
                                    } = li_vec_to_insert[last_index].element
                                    {
                                        li_vec_to_insert = inner_els;
                                    } else {
                                        panic!("Expected a nested list structure at the specified depth");
                                    }
                                }

                                let li = li_vec_to_insert.last_mut().unwrap();

                                match &mut li.element {
                                    Text {
                                        text: element_text,
                                        size,
                                    } => {
                                        element_text.push_str(&text);
                                    }
                                    Hyperlink {
                                        title,
                                        url,
                                        alt,
                                        size,
                                    } => {
                                        *title = text.to_string();
                                    }
                                    _ => {}
                                }
                            }
                            Element::Image {
                                bytes,
                                title,
                                alt,
                                image_type,
                            } => {
                                *alt = text.to_string();
                            }
                            Element::Hyperlink {
                                title,
                                url,
                                alt,
                                size,
                            } => {
                                *alt = alt.to_string();
                            }
                            _ => {}
                        }
                    }
                    match table_element {
                        Some(ref mut t_el) => {
                            if let (is_header, Element::Table { headers, rows }) = t_el {
                                if *is_header {
                                    headers.push(TableHeader {
                                        element: Text {
                                            text: text.to_string(),
                                            size: 14,
                                        },
                                        width: 30.,
                                    })
                                } else {
                                    let last_row = rows.last_mut();

                                    match last_row {
                                        Some(tr) => {
                                            if tr.cells.len() == headers.len() {
                                                rows.push(TableRow {
                                                    cells: vec![TableCell {
                                                        element: Text {
                                                            text: text.to_string(),
                                                            size: 14,
                                                        },
                                                    }],
                                                });
                                            } else {
                                                tr.cells.push(TableCell {
                                                    element: Text {
                                                        text: text.to_string(),
                                                        size: 14,
                                                    },
                                                });
                                            }
                                        }
                                        None => {
                                            rows.push(TableRow {
                                                cells: vec![TableCell {
                                                    element: Text {
                                                        text: text.to_string(),
                                                        size: 14,
                                                    },
                                                }],
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                Event::End(tag) => match tag {
                    TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::Link | TagEnd::Image => {
                        let curr_el = current_element.take();
                        if let Some(curr_el) = curr_el {
                            elements.push(curr_el);
                        }
                    }
                    TagEnd::List(_) => {
                        list_depth -= 1;

                        if list_depth == 0 {
                            let curr_el = current_element.take();
                            if let Some(curr_el) = curr_el {
                                elements.push(curr_el);
                            }
                        }
                    }
                    TagEnd::TableHead => {
                        if let Some((is_header, t_el)) = &mut table_element {
                            *is_header = false;
                        }
                    }
                    TagEnd::Table => {
                        if let Some((_, t_el)) = table_element.take() {
                            elements.push(t_el);
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
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
                    markdown.push_str(&"#".repeat(*level as usize));
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
                Text { text, size: _ } => {
                    let re = Regex::new(r#"^(\n)*\s+$?"#)?;
                    if !re.is_match(text) {
                        markdown.push('\n');
                        markdown.push_str(text);
                        if !text.ends_with(' ') {
                            markdown.push(' ');
                        }
                        markdown.push('\n');
                    }
                }
                Hyperlink {
                    title, url, alt, ..
                } => {
                    if url == alt && alt == url {
                        markdown.push_str(&url.to_string());
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
                        markdown.push('|');
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
    use crate::pdf;
    use crate::text;

    //     #[test]
    //     fn test() -> anyhow::Result<()> {
    //         let document = r#"# First header

    // Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
    // blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla

    // 1. List item 1
    // 2. List item 2
    // 3. List item 3
    //    1. List item secode level 1
    //    2. List item secode level 2
    // 4. List item 4
    //    1. List item secode level 3
    //    2. List item secode level 4
    // 5. List item 5
    //    1. List item secode level 5

    // - List item one
    //   - List item two
    // - List item three
    //   - List item four
    //   - List item five
    //     - List item zzz
    // - List item six
    //   - List item seven

    // ![Picture alt1](test/data/picture.png "Picture title1")

    // Bla bla bla ![Picture alt2](test/data/picture.png "Picture title2") bla. http://example.com  [Example](http://example.com) [Example](http://example.com "Example tooltip")

    // ## Second header

    // | Syntax      | Description |
    // | ----------- | ----------- |
    // | Header      | Title       |
    // | Paragraph   | Text        |

    // Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
    // blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
    //         // println!("{:?}", document);
    //         let mut images = HashMap::new();
    //         let image_bytes = std::fs::read("test/data/picture.png")?;
    //         images.insert("test/data/image0.png".to_string(), image_bytes);
    //         let parsed = Transformer::parse(&document.as_bytes().into(), &HashMap::new());
    //         let document_string = std::str::from_utf8(document.as_bytes())?;
    //         println!("{}", document_string);
    //         assert!(parsed.is_ok());
    //         let parsed_document = parsed.unwrap();
    //         println!("==========================");
    //         println!("{:?}", parsed_document);
    //         println!("==========================");
    //         let generated_result = Transformer::generate(&parsed_document);
    //         assert!(generated_result.is_ok());
    //         // println!("{:?}", generated_result.unwrap());
    //         let generated_bytes = generated_result?;
    //         let generated_text = std::str::from_utf8(&generated_bytes.0)?;
    //         println!("{}", generated_text);
    //         println!("==========================");
    //         let generated_result = text::Transformer::generate(&parsed_document);
    //         assert!(generated_result.is_ok());
    //         // println!("{:?}", generated_result.unwrap());
    //         let generated_bytes = generated_result?;
    //         let generated_text = std::str::from_utf8(&generated_bytes.0)?;
    //         println!("{}", generated_text);

    //         let generated_result = pdf::Transformer::generate(&parsed_document)?;
    //         std::fs::write("test/data/generated.pdf", generated_result.0)?;

    //         Ok(())
    //     }

    #[test]
    fn test_parse() -> anyhow::Result<()> {

        let document = std::fs::read("test/data/document.md")?;

        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/image0.png")?;
        images.insert("image0.png".to_string(), Bytes::from(image_bytes));
        let parsed = Transformer::parse(&document.into(), &images)?;
       
        println!("==========================");
        println!("{:#?}", parsed);
        println!("==========================");
       
        Ok(())
    }
}

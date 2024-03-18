use std::collections::HashMap;
use bytes::Bytes;
use crate::core::*;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> where Self: Sized {

        let mut elements: Vec<Box<dyn Element>> = vec![];
        let document: &str = std::str::from_utf8(document.as_ref())?;
        let lines = document.lines();
        let lines_vec: Vec<&str> = lines.collect();
        let mut i = 0;
        while i < lines_vec.len() {
            let line = lines_vec[i];
            elements.push(TextElement::new(line)?);
            elements.push(TextElement::new("\n")?);
            i += 1;
        }
        let new_paragraph = ParagraphElement::new(&elements)?;
        Ok(Document {
            elements: vec![new_paragraph],
        })
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
        Ok(())

    }
}

use crate::core::Element::{Header, List, Paragraph, Table, Text};
use crate::core::{Document, Element, ListItem, ParserError, TransformerTrait};
use bytes::Bytes;
use lopdf::content::Content;
use lopdf::{Document as PdfDocument, Object, ObjectId};
use std::collections::{BTreeMap, HashMap};

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let mut elements: Vec<Element> = Vec::new();
        let pdf_document = PdfDocument::load_mem(&document)?;
        for (_id, page_id) in pdf_document.get_pages() {
            let objects = pdf_document.get_page_contents(page_id);
            for object_id in objects {
                let object = pdf_document.get_object(object_id)?;
                parse_object(page_id, &pdf_document, &object, &mut elements)?;
            }
        }
        Ok(Document::new(elements))
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        use printpdf::*;

        const PAGE_WIDTH: f32 = 210.0;
        const PAGE_HEIGHT: f32 = 297.0;

        let (mut pdf, mut page1, mut layer1) =
            PdfDocument::new("PDF Document", Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");

        fn generate_pdf(
            document: &Document,
            element: &Element,
            pdf: &mut PdfDocumentReference,
            page: &mut PdfPageIndex,
            layer: &mut PdfLayerIndex,
            vertical_position: &mut f32,
        ) -> anyhow::Result<()> {
            match element {
                Header { level, text } => {
                    let font_size = match level {
                        1 => 18.0, // Example font size for level 1 header
                        2 => 16.0, // Example font size for level 2 header
                        3 => 14.0, // Example font size for level 3 header
                        // Additional levels as needed...
                        _ => 12.0, // Default font size for other header levels
                    };

                    let font_width = (0.3528 * (font_size as f32) * 0.87) as f32;
                    let max_text_width = document.page_height
                        - document.left_page_indent
                        - document.right_page_indent;
                    let max_chars = (max_text_width / font_width) as usize;
                    let text_elements = split_string(text, max_chars);
                    for text in text_elements {
                        let step: f32 = 0.3528 * font_size as f32;
                        if (*vertical_position + step)
                            > (document.page_height - document.bottom_page_indent)
                        {
                            let (new_page, new_layer) = pdf.add_page(
                                Mm(document.page_width),
                                Mm(document.page_height),
                                "Layer 1",
                            );
                            *vertical_position = 0.0 + document.top_page_indent;
                            *layer = new_layer;
                            *page = new_page;
                        }
                        *vertical_position = *vertical_position + step;
                        let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                        let current_layer = pdf.get_page(*page).get_layer(*layer);
                        current_layer.use_text(
                            text,
                            font_size as f32,
                            Mm(document.left_page_indent + 0.0),
                            Mm(document.page_height - *vertical_position),
                            &font,
                        );
                        *vertical_position = *vertical_position + 2.5;
                    }
                }
                Paragraph { elements } => {
                    for paragraph_element in elements {
                        match paragraph_element {
                            Text { text, size } => {
                                let font_width = (0.3528 * (*size as f32) * 0.87) as f32;
                                let max_text_width = document.page_height
                                    - document.left_page_indent
                                    - document.right_page_indent;
                                let max_chars = (max_text_width / font_width) as usize;
                                let text_elements = split_string(text, max_chars);
                                for text in text_elements {
                                    let step: f32 = 0.3528 * *size as f32;
                                    if (*vertical_position + step)
                                        > (document.page_height - document.bottom_page_indent)
                                    {
                                        let (new_page, new_layer) = pdf.add_page(
                                            Mm(document.page_width),
                                            Mm(document.page_height),
                                            "Layer 1",
                                        );
                                        *vertical_position = 0.0 + document.top_page_indent;
                                        *layer = new_layer;
                                        *page = new_page;
                                    }
                                    *vertical_position = *vertical_position + step;
                                    let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                                    let current_layer = pdf.get_page(*page).get_layer(*layer);
                                    current_layer.use_text(
                                        text,
                                        *size as f32,
                                        Mm(document.left_page_indent + 0.0),
                                        Mm(document.page_height - *vertical_position),
                                        &font,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Table { headers, rows } => {
                    if !headers.is_empty() {
                        for _header in headers {}
                    }
                    for row in rows {
                        for _cell in &row.cells {}
                    }
                }
                _ => {}
            }

            Ok(())
        }
        let mut vertical_position = 0.0 + document.top_page_indent;
        for element in &document.elements {
            _ = generate_pdf(
                document,
                element,
                &mut pdf,
                &mut page1,
                &mut layer1,
                &mut vertical_position,
            )?;
        }

        let result = pdf.save_to_bytes()?;
        let bytes = Bytes::from(result);
        Ok((bytes, HashMap::new()))
    }
}

fn parse_object(
    page_id: ObjectId,
    pdf_document: &PdfDocument,
    _object: &Object,
    elements: &mut Vec<Element>,
) -> anyhow::Result<()> {
    fn collect_text(
        text: &mut String,
        encoding: Option<&str>,
        operands: &[Object],
        elements: &mut Vec<Element>,
    ) -> anyhow::Result<()> {
        for operand in operands.iter() {
            // println!("2 {:?}", operand);
            match *operand {
                Object::String(ref bytes, _) => {
                    let decoded_text = PdfDocument::decode_text(encoding, bytes);
                    text.push_str(&decoded_text);
                    if bytes.len() == 1 && bytes[0] == 1 {
                        match elements.last() {
                            None => {
                                let list_element = List {
                                    elements: vec![],
                                    numbered: false,
                                };
                                elements.push(list_element);
                            }
                            Some(el) => {
                                match el {
                                    List { .. } => {
                                        let old_list = elements.pop().unwrap();
                                        // let list = old_list.list_as_ref()?;
                                        if let List {
                                            elements: list_elements,
                                            numbered,
                                        } = old_list
                                        {
                                            let mut list_item_elements = list_elements.clone();
                                            let text_element = Text {
                                                text: text.clone(),
                                                size: 8,
                                            };
                                            let new_list_item_element = ListItem {
                                                element: text_element,
                                            };
                                            list_item_elements.push(new_list_item_element);
                                            let new_list = List {
                                                elements: list_item_elements,
                                                numbered: numbered,
                                            };
                                            elements.push(new_list);
                                            text.clear();
                                        }
                                    }
                                    Paragraph { .. } => {
                                        let old_paragraph = elements.pop().unwrap();
                                        // let paragraph = old_paragraph.paragraph_as_ref()?;
                                        if let Paragraph {
                                            elements: paragraph_elements,
                                        } = old_paragraph
                                        {
                                            let mut paragraph_elements = paragraph_elements.clone();
                                            let text_element = Text {
                                                text: text.clone(),
                                                size: 8,
                                            };
                                            paragraph_elements.push(text_element);
                                            let new_paragraph = Paragraph {
                                                elements: paragraph_elements,
                                            };
                                            elements.push(new_paragraph);
                                            text.clear();

                                            let list_element = List {
                                                elements: vec![],
                                                numbered: false,
                                            };
                                            elements.push(list_element);
                                        }
                                    }
                                    _ => {
                                        let list_element = List {
                                            elements: vec![],
                                            numbered: false,
                                        };
                                        elements.push(*Box::new(list_element));
                                    }
                                }
                            }
                        }
                    }
                }
                Object::Array(ref arr) => {
                    let _ = collect_text(text, encoding, arr, elements);
                    text.push(' ');
                }
                Object::Integer(i) => {
                    if i < -100 {
                        text.push(' ');
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut text = String::new();

    let fonts = pdf_document.get_page_fonts(page_id);
    let encodings = fonts
        .into_iter()
        .map(|(name, font)| (name, font.get_font_encoding()))
        .collect::<BTreeMap<Vec<u8>, &str>>();

    let vec = pdf_document.get_page_content(page_id)?;
    let content = Content::decode(&vec)?;
    let mut current_encoding = None;
    for operation in &content.operations {
        // println!("1 {:?}", operation.operator);
        match operation.operator.as_ref() {
            "Tm" => {
                let text_element = Text {
                    text: text.clone(),
                    size: 8,
                };
                match elements.last() {
                    None => {
                        let paragraph_element = Paragraph {
                            elements: vec![text_element],
                        };
                        elements.push(paragraph_element);
                    }
                    Some(el) => match el {
                        Paragraph { .. } => {
                            let old_paragraph = elements.pop().unwrap();
                            if let Paragraph {
                                elements: paragraph_elements,
                            } = old_paragraph
                            {
                                let mut paragraph_elements = paragraph_elements.clone();
                                paragraph_elements.push(text_element);
                                let new_paragraph = Paragraph {
                                    elements: paragraph_elements,
                                };
                                elements.push(new_paragraph);
                            }
                        }
                        _ => {
                            elements.push(text_element);
                        }
                    },
                }
                text.clear();
            }
            "Tf" => {
                let current_font = operation
                    .operands
                    .first()
                    .ok_or_else(|| ParserError::Common)?
                    .as_name()?;
                current_encoding = encodings.get(current_font).cloned();
            }
            "Tj" | "TJ" => {
                _ = collect_text(&mut text, current_encoding, &operation.operands, elements);
            }
            "ET" => {
                if !text.ends_with('\n') {
                    text.push('\n')
                }
            }
            _ => {}
        }
    }

    if text.len() > 0 {
        let text_element = Text {
            text: text.clone(),
            size: 8,
        };
        match elements.last() {
            None => {
                let paragraph_element = Paragraph {
                    elements: vec![text_element],
                };
                elements.push(*Box::new(paragraph_element));
            }
            Some(el) => {
                match el {
                    Paragraph { .. } => {
                        let old_paragraph = elements.pop().unwrap();
                        if let Paragraph {
                            elements: paragraph_elements,
                        } = old_paragraph
                        {
                            let mut paragraph_elements = paragraph_elements.clone();
                            paragraph_elements.push(text_element);
                            let new_paragraph = Paragraph {
                                elements: paragraph_elements,
                            };
                            elements.push(*Box::new(new_paragraph));
                        }
                    }
                    List { .. } => {
                        let old_list = elements.pop().unwrap();
                        // let list = old_list.list_as_ref()?;
                        if let List {
                            elements: list_elements,
                            numbered,
                        } = old_list
                        {
                            let mut list_item_elements = list_elements.clone();
                            let new_list_item_element = ListItem {
                                element: text_element,
                            };
                            list_item_elements.push(new_list_item_element);
                            let new_list = List {
                                elements: list_item_elements,
                                numbered: numbered,
                            };
                            elements.push(*Box::new(new_list));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{}", text);

    Ok(())
}

fn split_string(input: &str, max_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_string = String::new();

    for char in input.chars() {
        if current_string.chars().count() < max_length {
            current_string.push(char);
        } else {
            result.push(current_string);
            current_string = char.to_string();
        }
    }

    if !current_string.is_empty() {
        result.push(current_string);
    }

    result
}
#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::pdf::Transformer;
    use bytes::Bytes;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        let pdf = std::fs::read("test/data/document.pdf")?;
        let pdf_bytes = Bytes::from(pdf);
        let parsed = Transformer::parse(&pdf_bytes, &HashMap::new());
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        Ok(())
    }
}

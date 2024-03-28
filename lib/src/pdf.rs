use std::collections::{BTreeMap, HashMap};
use bytes::Bytes;
use crate::core::{Document, Element, ElementType, ListElement, ListItemElement, ParagraphElement, ParserError, TextElement, TransformerTrait};
use lopdf::{Document as PdfDocument, Object, ObjectId};
use lopdf::content::Content;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let mut elements: Vec<Box<dyn Element>> = Vec::new();
        let pdf_document = PdfDocument::load_mem(&document)?;
        for (_id, page_id) in pdf_document.get_pages() {
            let objects = pdf_document.get_page_contents(page_id);
            for object_id in objects {
                let object = pdf_document.get_object(object_id)?;
                parse_object(page_id, &pdf_document, &object, &mut elements)?;
            }

        }
        Ok(Document::new(&elements)?)
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {

        use printpdf::*;

        const PAGE_WIDTH: f32 = 210.0;
        const PAGE_HEIGHT: f32 = 297.0;


        let (mut pdf, page1, layer1) = PdfDocument::new("PDF Document", Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");

        fn generate_pdf(elements:&Vec<Box<dyn Element>>, pdf:&mut PdfDocumentReference, mut page: PdfPageIndex, mut layer: PdfLayerIndex, mut vertical_position: f32) -> anyhow::Result<()> {


            for element in elements {
                match element.as_ref() {
                    e if e.element_type() == ElementType::Header => {

                    },
                    e if e.element_type() == ElementType::Paragraph => {
                        let paragraph = ParagraphElement::from(element)?;
                        for paragraph_element in &paragraph.elements {
                            match paragraph_element.as_ref() {
                                e if e.element_type() == ElementType::Text => {
                                    let text_element = TextElement::from(paragraph_element)?;
                                    let step: f32 = 0.3528 * text_element.size as f32;
                                    if (vertical_position + step) > PAGE_HEIGHT {
                                        let (new_page, new_layer) = pdf.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
                                        vertical_position = 0.0;
                                        layer = new_layer;
                                        page = new_page;
                                    }
                                    vertical_position = vertical_position + step;
                                    let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                                    let current_layer = pdf.get_page(page).get_layer(layer);
                                    current_layer.use_text(&text_element.text, text_element.size as f32, Mm(0.0), Mm(PAGE_HEIGHT - vertical_position), &font);
                                },
                                _ => {}
                            }
                        }

                    },
                    _ => {}
                }
            }

            Ok(())
        }

        _ = generate_pdf(&document.elements, &mut pdf, page1, layer1, 0.0)?;


        let result = pdf.save_to_bytes()?;
        let bytes = Bytes::from(result);
        Ok((bytes, HashMap::new()))
    }
}

fn parse_object(page_id: ObjectId, pdf_document: &PdfDocument, _object: &Object, elements: &mut Vec<Box<dyn Element>>) -> anyhow::Result<()> {

    fn collect_text(text: &mut String, encoding: Option<&str>, operands: &[Object], elements: &mut Vec<Box<dyn Element>>) -> anyhow::Result<()>{
        for operand in operands.iter() {
            // println!("2 {:?}", operand);
            match *operand {
                Object::String(ref bytes, _) => {
                    let decoded_text = PdfDocument::decode_text(encoding, bytes);
                    text.push_str(&decoded_text);
                    if bytes.len() == 1 && bytes[0] == 1 {
                        match elements.last() {
                            None => {
                                let list_element = ListElement{
                                    elements: vec![],
                                    numbered: false,
                                };
                                elements.push(Box::new(list_element));
                            }
                            Some(el) => {
                                if el.element_type() == ElementType::List {
                                    let old_list = elements.pop().unwrap();
                                    let list = ListElement::from(&old_list)?;
                                    let mut list_item_elements = list.elements.clone();
                                    let text_element = TextElement{
                                        text: text.clone(),
                                        size: 8,
                                    };
                                    let new_list_item_element = ListItemElement{
                                        element: Box::new(text_element),
                                    };
                                    list_item_elements.push(new_list_item_element);
                                    let new_list = ListElement{
                                        elements: list_item_elements,
                                        numbered: list.numbered,
                                    };
                                    elements.push(Box::new(new_list));
                                    text.clear();
                                } else {
                                    if el.element_type() == ElementType::Paragraph {
                                        let old_paragraph = elements.pop().unwrap();
                                        let paragraph = ParagraphElement::from(&old_paragraph)?;
                                        let mut paragraph_elements = paragraph.elements.clone();
                                        let text_element = TextElement{
                                            text: text.clone(),
                                            size: 8,
                                        };
                                        paragraph_elements.push(Box::new(text_element));
                                        let new_paragraph = ParagraphElement{
                                            elements: paragraph_elements,
                                        };
                                        elements.push(Box::new(new_paragraph));
                                        text.clear();
                                    }
                                    let list_element = ListElement{
                                        elements: vec![],
                                        numbered: false,
                                    };
                                    elements.push(Box::new(list_element));
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
                let text_element = TextElement {
                    text: text.clone(),
                    size: 8,
                };
                match elements.last() {
                    None => {
                        let paragraph_element = ParagraphElement{
                            elements: vec![Box::new(text_element)],
                        };
                        elements.push(Box::new(paragraph_element));
                    }
                    Some(el) => {
                        if el.element_type() == ElementType::Paragraph {
                            let old_paragraph = elements.pop().unwrap();
                            let paragraph = ParagraphElement::from(&old_paragraph)?;
                            let mut paragraph_elements = paragraph.elements.clone();
                            paragraph_elements.push(Box::new(text_element));
                            let new_paragraph = ParagraphElement{
                                elements: paragraph_elements,
                            };
                            elements.push(Box::new(new_paragraph));
                        } else {
                            elements.push(Box::new(text_element));
                        }

                    }
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
        let text_element = TextElement {
            text: text.clone(),
            size: 8,
        };
        match elements.last() {
            None => {
                let paragraph_element = ParagraphElement {
                    elements: vec![Box::new(text_element)],
                };
                elements.push(Box::new(paragraph_element));
            }
            Some(el) => {
                if el.element_type() == ElementType::Paragraph {
                    let old_paragraph = elements.pop().unwrap();
                    let paragraph = ParagraphElement::from(&old_paragraph)?;
                    let mut paragraph_elements = paragraph.elements.clone();
                    paragraph_elements.push(Box::new(text_element));
                    let new_paragraph = ParagraphElement {
                        elements: paragraph_elements,
                    };
                    elements.push(Box::new(new_paragraph));
                } else if el.element_type() == ElementType::List {
                    let old_list = elements.pop().unwrap();
                    let list = ListElement::from(&old_list)?;
                    let mut list_item_elements = list.elements.clone();
                    let new_list_item_element = ListItemElement{
                        element: Box::new(text_element),
                    };
                    list_item_elements.push(new_list_item_element);
                    let new_list = ListElement{
                        elements: list_item_elements,
                        numbered: list.numbered,
                    };
                    elements.push(Box::new(new_list));
                } else {

                }
            }
        }

    }

    println!("{}", text);




    Ok(())


}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use bytes::Bytes;
    use crate::core::*;
    use crate::pdf::Transformer;

    #[test]
    fn test() -> anyhow::Result<()> {
        let pdf = std::fs::read("test/data/document.pdf")?;
        let pdf_bytes = Bytes::from(pdf);
        let parsed =  Transformer::parse(&pdf_bytes, &HashMap::new());
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        // let generated_result = Transformer::generate(&parsed_document);
        // assert!(generated_result.is_ok());
        // let generated_bytes = generated_result?;
        // let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        // println!("{}", generated_text);
        Ok(())

    }
}

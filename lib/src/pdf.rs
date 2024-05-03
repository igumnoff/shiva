use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use crate::core::{
    Document, Element, ImageType, ListItem, ParserError, TableHeader, TableRow, TransformerTrait,
};

use comemo::Prehashed;

use bytes::Bytes;
use lopdf::content::Content;
use lopdf::{Document as PdfDocument, Object, ObjectId};
use std::collections::{BTreeMap, HashMap};
use time::{OffsetDateTime, UtcOffset};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Datetime, Smart};
use typst::{
    eval::Tracer,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    Library, World,
};

type TypstString = String;

struct ShivaWorld {
    fonts: Vec<Font>,
    book: Prehashed<FontBook>,
    library: Prehashed<Library>,
    source: Source,
    img_map: HashMap<String, typst::foundations::Bytes>,
}

impl ShivaWorld {
    fn new(source: String, img_map: HashMap<String, typst::foundations::Bytes>) -> Self {
        let source = Source::detached(source);

        let fonts = std::fs::read_dir("fonts")
            .unwrap()
            .map(Result::unwrap)
            .flat_map(|entry| {
                let path = entry.path();
                let bytes = std::fs::read(&path).unwrap();
                let buffer = typst::foundations::Bytes::from(bytes);
                let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
                (0..face_count).map(move |face| {
                    Font::new(buffer.clone(), face).unwrap_or_else(|| {
                        panic!("failed to load font from {path:?} (face index {face})")
                    })
                })
            })
            .collect::<Vec<Font>>();

        Self {
            book: Prehashed::new(FontBook::from_fonts(&fonts)),
            fonts,
            library: Prehashed::new(Library::default()),
            source,
            img_map,
        }
    }
}

impl World for ShivaWorld {
    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn main(&self) -> Source {
        self.source.clone()
    }

    fn source(&self, _id: FileId) -> FileResult<Source> {
        Ok(self.source.clone())
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    // need to think how to implement path and file extraction
    fn file(&self, id: FileId) -> Result<typst::foundations::Bytes, FileError> {
        let path = id.vpath();

        let key = path.as_rootless_path().to_str().unwrap();
        let img = self.img_map.get(key).unwrap();

        Ok(img.clone())
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        // We are in UTC.
        let offset = offset.unwrap_or(0);
        let offset = UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = OffsetDateTime::now_utc().checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let mut elements: Vec<Element> = Vec::new();
        let pdf_document = PdfDocument::load_mem(document)?;
        for (_id, page_id) in pdf_document.get_pages() {
            let objects = pdf_document.get_page_contents(page_id);
            for object_id in objects {
                let object = pdf_document.get_object(object_id)?;
                parse_object(page_id, &pdf_document, object, &mut elements)?;
            }
        }
        Ok(Document::new(elements))
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        fn process_header(
            source: &mut TypstString,
            level: usize,
            text: &str,
        ) -> anyhow::Result<()> {
            let header_depth = "=".repeat(level);
            let header_text = format!("{header_depth} {text}");
            source.push_str(&header_text);
            source.push('\n');

            Ok(())
        }

        fn process_text(
            source: &mut TypstString,
            _size: u8,
            text: &str,
            is_bold: bool,
        ) -> anyhow::Result<()> {
            if is_bold {
                let bold_text = format!("*{text}*");
                source.push_str(&bold_text);
            } else {
                source.push_str(text);
            }

            Ok(())
        }

        fn process_link(source: &mut TypstString, url: &str) -> anyhow::Result<()> {
            let link = format!("#link(\"{url}\")");

            source.push_str(&link);

            Ok(())
        }

        fn process_table(
            source: &mut TypstString,
            headers: &Vec<TableHeader>,
            rows: &Vec<TableRow>,
        ) -> anyhow::Result<()> {
            let mut headers_text = TypstString::new();

            for header in headers {
                match &header.element {
                    Text { text, size } => {
                        headers_text.push('[');
                        process_text(&mut headers_text, *size, text, true)?;
                        headers_text.push(']');
                        headers_text.push(',');
                    }
                    _ => {
                        eprintln!(
                            "Should implement element for processing in inside table header - {:?}",
                            header.element
                        );
                    }
                }
            }

            let mut cells_text = TypstString::new();

            for row in rows {
                for cell in &row.cells {
                    match &cell.element {
                        Text { text, size } => {
                            cells_text.push('[');
                            process_text(&mut cells_text, *size, text, false)?;
                            cells_text.push(']');
                            cells_text.push(',');
                        }
                        _ => {
                            eprintln!(
                                "Should implement element for processing in inside cell - {:?}",
                                cell.element
                            );
                        }
                    }
                }

                cells_text.push('\n');
            }

            let columns = headers.len();
            let table_text = format!(
                r#"
            #table(
                columns:{columns},
                {headers_text}
                {cells_text}
            )
            "#
            );

            source.push_str(&table_text);
            Ok(())
        }

        fn process_list(
            source: &mut TypstString,
            img_map: &mut HashMap<String, typst::foundations::Bytes>,
            list: &Vec<ListItem>,
            numbered: bool,
            depth: usize,
        ) -> anyhow::Result<()> {
            source.push_str(&" ".repeat(depth));
            for el in list {
                if let List { elements, numbered } = &el.element {
                    process_list(source, img_map, elements, *numbered, depth + 1)?;
                } else {
                    if numbered {
                        source.push_str("+ ")
                    } else {
                        source.push_str("- ")
                    };

                    process_element(source, img_map, &el.element)?;
                }
            }

            Ok(())
        }

        fn process_image(
            source: &mut TypstString,
            bytes: &Bytes,
            title: &str,
            alt: &str,
            image_type: &str,
        ) -> anyhow::Result<()> {
            if !bytes.is_empty() {
                let image_text = format!(
                    "
            #figure(
                image(\"{title}{image_type}\", alt: \"{alt}\"), 
                caption: [
                 {title}
                ],
              )"
                );
                source.push_str(&image_text);
            }
            // need to think how to implement using raw bytes
            Ok(())
        }

        fn process_element(
            source: &mut TypstString,
            img_map: &mut HashMap<String, typst::foundations::Bytes>,
            element: &Element,
        ) -> anyhow::Result<()> {
            match element {
                Header { level, text } => process_header(source, *level as usize, text),
                Paragraph { elements } => {
                    for paragraph_element in elements {
                        process_element(source, img_map, paragraph_element)?;
                    }

                    Ok(())
                }
                Text { text, size } => {
                    process_text(source, *size, text, false)?;
                    source.push('\n');

                    Ok(())
                }
                List { elements, numbered } => {
                    process_list(source, img_map, elements, *numbered, 0)?;
                    Ok(())
                }
                Hyperlink {
                    url,
                    title: _,
                    alt: _,
                    size: _,
                } => {
                    process_link(source, url)?;
                    source.push('\n');

                    Ok(())
                }
                Table { headers, rows } => {
                    process_table(source, headers, rows)?;
                    Ok(())
                }
                Image {
                    bytes,
                    title,
                    alt,
                    image_type,
                } => {
                    let image_type = match image_type {
                        ImageType::Jpeg => ".jpeg",
                        ImageType::Png => ".png",
                    };
                    let key = format!("{title}{image_type}");
                    img_map.insert(key, typst::foundations::Bytes::from(bytes.to_vec()));
                    process_image(source, bytes, title, alt, image_type)?;
                    source.push('\n');
                    Ok(())
                }
                _ => {
                    eprintln!("Should implement element - {:?}", element);
                    Ok(())
                }
            }
        }

        let mut source = TypstString::new();
        let mut img_map: HashMap<String, typst::foundations::Bytes> = HashMap::new();

        let mut header_text = String::new();
        document.page_header.iter().for_each(|el| match el {
            Text { text, size: _ } => {
                header_text.push_str(text);
            }
            _ => {}
        });
        let mut footer_text = String::new();

        document.page_footer.iter().for_each(|el| match el {
            Text { text, size: _ } => {
                footer_text.push_str(text);
            }
            _ => {}
        });

        let footer_header_text = format!(
            "#set page(
            header: \"{header_text}\",
            footer: \"{footer_text}\",
          )\n"
        );

        source.push_str(&footer_header_text);
        for element in &document.elements {
            process_element(&mut source, &mut img_map, element)?;
        }

        let world = ShivaWorld::new(source, img_map);
        let mut tracer = Tracer::default();

        let document = typst::compile(&world, &mut tracer).unwrap();
        let warnings = tracer.warnings();

        if !warnings.is_empty() {
            for warn in warnings {
                println!("Warning - {}", warn.message);
            }
        }

        let pdf = typst_pdf::pdf(&document, Smart::Auto, None);

        let bytes = Bytes::from(pdf);
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
                                                numbered,
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
                    .ok_or(ParserError::Common)?
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

    if !text.is_empty() {
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
                                numbered,
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

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::markdown;
    use crate::pdf::Transformer;
    use bytes::Bytes;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        let pdf = std::fs::read("test/data/document.pdf")?;
        let pdf_bytes = Bytes::from(pdf);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        let image_bytes = Bytes::from(image_bytes);
        images.insert("test/data/image0.png".to_string(), image_bytes);
        let parsed = Transformer::parse(&pdf_bytes, &images);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = Transformer::generate(&parsed_document)?;
        std::fs::write("test/data/generated.pdf", generated_result.0)?;
        Ok(())
    }

    #[test]
    fn test_list() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.md")?;
        let documents_bytes = Bytes::from(document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        let image_bytes = Bytes::from(image_bytes);
        images.insert("image0.png".to_string(), image_bytes);
        let parsed = markdown::Transformer::parse(&documents_bytes, &images);
        assert!(parsed.is_ok());
        let mut parsed_document = parsed.unwrap();
        println!("==========================");
        // println!("{:?}", parsed_document);
        println!("==========================");
        parsed_document.page_header = vec![Element::Text {
            text: "header".to_string(),
            size: 10,
        }];

        parsed_document.page_footer = vec![Element::Text {
            text: "footer".to_string(),
            size: 10,
        }];
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        std::fs::write("test/data/typst.pdf", generated_result.unwrap().0)?;

        Ok(())
    }

    #[test]
    fn test_hyperlink_generation() -> anyhow::Result<()> {
        use Element::*;

        let document = Document {
            elements: vec![
                Paragraph {
                    elements: vec![
                        Text {
                            text: "Line 1".to_owned(),
                            size: 8,
                        },
                        Text {
                            text: "Line 2".to_owned(),
                            size: 8,
                        },
                        Text {
                            text: "Line 3".to_owned(),
                            size: 8,
                        },
                    ],
                },
                Hyperlink {
                    title: "Example".to_owned(),
                    url: "https://www.example.com".to_owned(),
                    alt: "Example Site".to_owned(),
                    size: 8,
                },
                Hyperlink {
                    title: "GitHub".to_owned(),
                    url: "https://www.github.com".to_owned(),
                    alt: "GitHub".to_owned(),
                    size: 8,
                },
            ],
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 20.0,
            bottom_page_indent: 10.0,
            page_header: vec![],
            page_footer: vec![],
        };

        println!("==========================");
        println!("{:?}", document);
        println!("==========================");

        let generated_result = Transformer::generate(&document);

        assert!(generated_result.is_ok());

        std::fs::write(
            "test/data/generated_hyperlink.pdf",
            generated_result.unwrap().0,
        )?;

        Ok(())
    }
}

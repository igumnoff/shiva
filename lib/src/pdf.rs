use crate::core::Element::{Header, Hyperlink, List, Paragraph, Table, Text};
use crate::core::{
    Document, Element, ListItem, ParserError, TableHeader, TableRow, TransformerTrait,
};

use comemo::Prehashed;

use bytes::Bytes;
use lopdf::content::Content;
use lopdf::{Document as PdfDocument, Object, ObjectId};
use printpdf::{BuiltinFont, Mm, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};
use std::collections::{BTreeMap, HashMap};
use time::{OffsetDateTime, UtcOffset};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Datetime, Smart};
use typst::syntax::VirtualPath;
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
}

impl ShivaWorld {
    fn new(source: String) -> Self {
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
            source: source,
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

    fn source(&self, id: FileId) -> FileResult<Source> {
        println!("self.source - {:?}", self.source);
        println!("self.source.text - {:?}", self.source.text());
        Ok(self.source.clone())
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    fn file(&self, id: FileId) -> Result<typst::foundations::Bytes, FileError> {
        todo!()
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

        fn process_text(source: &mut TypstString, _size: u8, text: &str) -> anyhow::Result<()> {
            source.push_str(text);
            source.push('\n');

            Ok(())
        }

        fn process_link(source: &mut TypstString, url: &str) -> anyhow::Result<()> {
            let link = format!("#link(\"{url}\") \n");

            println!("{}", link);

            source.push_str(&link);

            Ok(())
        }


        fn process_link(source: &mut TypstString, url: &str) -> anyhow::Result<()> {
            let link = format!("#link(\"{url}\") \n");

            println!("{}", link);

            source.push_str(&link);

            Ok(())
        }

        fn process_list(
            source: &mut TypstString,
            list: &Vec<ListItem>,
            numbered: bool,
            depth: usize,
        ) -> anyhow::Result<()> {
            source.push_str(&" ".repeat(depth));
            for el in list {
                if let List { elements, numbered } = &el.element {
                    process_list(source, elements, *numbered, depth + 1)?;
                } else {
                    if numbered {
                        source.push_str("+ ")
                    } else {
                        source.push_str("- ")
                    };

                    process_element(source, &el.element)?;
                }
            }

            Ok(())
        }

        fn process_element(source: &mut TypstString, element: &Element) -> anyhow::Result<()> {
            match element {
                Header { level, text } => process_header(source, *level as usize, text),
                Paragraph { elements } => {
                    for paragraph_element in elements {
                        process_element(source, paragraph_element)?;
                    }

                    Ok(())
                }
                Text { text, size } => process_text(source, *size, text),
                List { elements, numbered } => {
                    process_list(source, elements, *numbered, 0)?;
                    Ok(())
                }
                Hyperlink {
                    url,
                    title: _,
                    alt: _,
                    size: _,
                } => {
                    process_link(source, url)?;

                    Ok(())
                },
                                Table { headers, rows } => {
                                    

             Ok(())
                }
                _ => {
                    eprintln!("Should implement element - {:?}", element);
                    Ok(())
                }
            }
        }

        let mut source = TypstString::new();
        for element in &document.elements {
            println!("element - {:?}", element);
            process_element(&mut source, element)?;
        }

        println!("source - {}", source);

        let world = ShivaWorld::new(source);
        let mut tracer = Tracer::default();

        let document = typst::compile(&world, &mut tracer).unwrap();
        println!("document - {:?}", document);
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

    // fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
    //     use printpdf::*;
    //     use typst::*;

    //     const PAGE_WIDTH: f32 = 210.0;
    //     const PAGE_HEIGHT: f32 = 297.0;

    //     let (mut pdf, mut page1, mut layer1) =
    //         PdfDocument::new("PDF Document", Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");

    //     render_header_footer(&mut pdf, &mut page1, &mut layer1, document)?;

    //     fn render_table_header(
    //         header: &TableHeader,
    //         pdf: &mut PdfDocumentReference,
    //         page: &mut PdfPageIndex,
    //         layer: &mut PdfLayerIndex,
    //         vertical_position: &mut f32,
    //         horizontal_position: &mut f32,
    //         document: &Document,
    //     ) -> anyhow::Result<()> {
    //         let font_size: f32 = match &header.element {
    //             Text { text: _, size } => *size as f32,
    //             _ => 10.0,
    //         };

    //         let font = pdf.add_builtin_font(BuiltinFont::Courier)?;

    //         let max_text_width = header.width;
    //         let max_chars = (max_text_width / (0.3528 * font_size)) as usize;

    //         let text_elements: Vec<String> = match &header.element {
    //             Text { text, size: _ } => split_string(text, max_chars),
    //             _ => {
    //                 vec!["".to_string()]
    //             }
    //         };

    //         for text in text_elements {
    //             let step: f32 = 0.3528 * font_size;
    //             if (*vertical_position + step)
    //                 > (document.page_height - document.bottom_page_indent)
    //             {
    //                 let (mut new_page, mut new_layer) =
    //                     pdf.add_page(Mm(document.page_width), Mm(document.page_height), "Layer 1");
    //                 render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
    //                 *vertical_position = document.top_page_indent;
    //                 *page = new_page;
    //                 *layer = new_layer;
    //             }

    //             let current_layer = pdf.get_page(*page).get_layer(*layer);
    //             current_layer.use_text(
    //                 text,
    //                 font_size,
    //                 Mm(document.left_page_indent + *horizontal_position),
    //                 Mm(document.page_height - *vertical_position),
    //                 &font,
    //             );

    //             *vertical_position += step + 2.5; // Adjust vertical position for next element
    //         }

    //         Ok(())
    //     }

    //     fn render_table_row(
    //         row: &TableRow,
    //         pdf: &mut PdfDocumentReference,
    //         page: &mut PdfPageIndex,
    //         layer: &mut PdfLayerIndex,
    //         vertical_position: &mut f32,
    //         headers: &Vec<TableHeader>,
    //         document: &Document,
    //     ) -> anyhow::Result<()> {
    //         let mut horizontal_position: f32 = 0.0;

    //         let mut vertical_position_max: f32 = *vertical_position;
    //         for (i, cell) in row.cells.iter().enumerate() {
    //             let vertical_position_backup: f32 = *vertical_position;
    //             match &cell.element {
    //                 Text { text, size } => {
    //                     let font_size = *size as f32;
    //                     let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
    //                     let max_text_width = headers[i].width;
    //                     let max_chars = (max_text_width / (0.3528 * font_size)) as usize;

    //                     let text_elements = split_string(text, max_chars);
    //                     let step: f32 = 0.3528 * font_size; // Height adjustment for text elements
    //                     for text in text_elements {
    //                         if (*vertical_position + step)
    //                             > (document.page_height - document.bottom_page_indent)
    //                         {
    //                             let (mut new_page, mut new_layer) = pdf.add_page(
    //                                 Mm(document.page_width),
    //                                 Mm(document.page_height),
    //                                 "Layer 1",
    //                             );
    //                             render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
    //                             *vertical_position = document.top_page_indent;
    //                             *page = new_page;
    //                             *layer = new_layer;
    //                         }
    //                         let current_layer = pdf.get_page(*page).get_layer(*layer);
    //                         current_layer.use_text(
    //                             text,
    //                             font_size,
    //                             Mm(document.left_page_indent + horizontal_position),
    //                             Mm(document.page_height - *vertical_position),
    //                             &font,
    //                         );
    //                         *vertical_position += step; // Additional vertical spacing between cells
    //                     }
    //                     horizontal_position += headers[i].width;
    //                     if *vertical_position > vertical_position_max {
    //                         vertical_position_max = *vertical_position;
    //                     };
    //                 }

    //                 _ => { /* */ } // Implement other element types as necessary
    //             }
    //             *vertical_position = vertical_position_backup;
    //         }
    //         *vertical_position = vertical_position_max;
    //         Ok(())
    //     }

    //     fn render_list(
    //         pdf: &mut PdfDocumentReference,
    //         document: &Document,
    //         layer: &mut PdfLayerIndex,
    //         page: &mut PdfPageIndex,
    //         elements: &Vec<ListItem>,
    //         numbered: bool,
    //         vertical_position: &mut f32,
    //         list_depth: usize,
    //     ) -> anyhow::Result<()> {
    //         let bullet_type = ["\u{2022} ", " ", " "];
    //         for (index, list_item) in elements.iter().enumerate() {
    //             match &list_item.element {
    //                 Text { text, size } => {
    //                     let font_width = (0.3528 * (*size as f32) * 0.6) as f32;
    //                     let max_text_width = document.page_width
    //                         - document.left_page_indent
    //                         - document.right_page_indent;
    //                     let max_chars = (max_text_width / font_width) as usize;
    //                     let text_elements = split_string(text, max_chars);
    //                     for text in text_elements {
    //                         let step: f32 = 0.3528 * *size as f32;
    //                         if (*vertical_position + step)
    //                             > (document.page_height - document.bottom_page_indent)
    //                         {
    //                             let (mut new_page, mut new_layer) = pdf.add_page(
    //                                 Mm(document.page_width),
    //                                 Mm(document.page_height),
    //                                 "Layer 1",
    //                             );
    //                             render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
    //                             *vertical_position = 0.0 + document.top_page_indent;
    //                             *layer = new_layer;
    //                             *page = new_page;
    //                         }
    //                         *vertical_position = *vertical_position + step;
    //                         let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
    //                         let current_layer = pdf.get_page(*page).get_layer(*layer);
    //                         let mut item_text;
    //                         let left_indent = 12.0 * (list_depth as f32);

    //                         if numbered {
    //                             item_text = (index + 1).to_string();
    //                             item_text.push_str(&format!(". {}", text));

    //                             current_layer.use_text(
    //                                 item_text,
    //                                 *size as f32,
    //                                 Mm(document.left_page_indent + left_indent),
    //                                 Mm(document.page_height - *vertical_position),
    //                                 &font,
    //                             );
    //                         } else {
    //                             item_text = bullet_type[list_depth % bullet_type.len()].to_string();
    //                             item_text.push_str(&text);

    //                             match list_depth % bullet_type.len() {
    //                                 0 => {
    //                                     current_layer.use_text(
    //                                         item_text,
    //                                         *size as f32,
    //                                         Mm(document.left_page_indent + left_indent),
    //                                         Mm(document.page_height - *vertical_position),
    //                                         &font,
    //                                     );
    //                                 }
    //                                 1 => {
    //                                     let mut rect = Rect::new(
    //                                         Mm(document.left_page_indent + left_indent),
    //                                         Mm(document.page_height - *vertical_position + 0.5),
    //                                         Mm(document.left_page_indent + left_indent + 1.),
    //                                         Mm(document.page_height - *vertical_position + 1.5),
    //                                     );

    //                                     rect = rect.with_mode(path::PaintMode::Stroke);
    //                                     current_layer.add_rect(rect);

    //                                     current_layer.use_text(
    //                                         item_text,
    //                                         *size as f32,
    //                                         Mm(document.left_page_indent + left_indent + 2.),
    //                                         Mm(document.page_height - *vertical_position),
    //                                         &font,
    //                                     );
    //                                 }
    //                                 2 => {
    //                                     current_layer.add_rect(Rect::new(
    //                                         Mm(document.left_page_indent + left_indent),
    //                                         Mm(document.page_height - *vertical_position + 0.5),
    //                                         Mm(document.left_page_indent + left_indent + 1.),
    //                                         Mm(document.page_height - *vertical_position + 1.5),
    //                                     ));

    //                                     current_layer.use_text(
    //                                         item_text,
    //                                         *size as f32,
    //                                         Mm(document.left_page_indent + left_indent + 2.),
    //                                         Mm(document.page_height - *vertical_position),
    //                                         &font,
    //                                     );
    //                                 }
    //                                 _ => {}
    //                             }
    //                         }
    //                     }
    //                 }
    //                 List { elements, numbered } => render_list(
    //                     pdf,
    //                     document,
    //                     layer,
    //                     page,
    //                     elements,
    //                     *numbered,
    //                     vertical_position,
    //                     list_depth + 1,
    //                 )?,
    //                 _ => {}
    //             }
    //         }

    //         Ok(())
    //     }

    //     fn generate_pdf(
    //         document: &Document,
    //         element: &Element,
    //         pdf: &mut PdfDocumentReference,
    //         page: &mut PdfPageIndex,
    //         layer: &mut PdfLayerIndex,
    //         vertical_position: &mut f32,
    //     ) -> anyhow::Result<()> {
    //         match element {
    //             Header { level, text } => {
    //                 let font_size = match level {
    //                     1 => 18.0, // Example font size for level 1 header
    //                     2 => 16.0, // Example font size for level 2 header
    //                     3 => 14.0, // Example font size for level 3 header
    //                     // Additional levels as needed...
    //                     _ => 12.0, // Default font size for other header levels
    //                 };

    //                 let font_width = 0.3528 * (font_size as f32) * 0.6;
    //                 let max_text_width = document.page_width
    //                     - document.left_page_indent
    //                     - document.right_page_indent;
    //                 let max_chars = (max_text_width / font_width) as usize;
    //                 let text_elements = split_string(text, max_chars);
    //                 for text in text_elements {
    //                     let step: f32 = 0.3528 * font_size as f32;
    //                     if (*vertical_position + step)
    //                         > (document.page_height - document.bottom_page_indent)
    //                     {
    //                         let (mut new_page, mut new_layer) = pdf.add_page(
    //                             Mm(document.page_width),
    //                             Mm(document.page_height),
    //                             "Layer 1",
    //                         );
    //                         render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
    //                         *vertical_position = 0.0 + document.top_page_indent;
    //                         *layer = new_layer;
    //                         *page = new_page;
    //                     }
    //                     *vertical_position += step;
    //                     let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
    //                     let current_layer = pdf.get_page(*page).get_layer(*layer);
    //                     current_layer.use_text(
    //                         text,
    //                         font_size as f32,
    //                         Mm(document.left_page_indent + 0.0),
    //                         Mm(document.page_height - *vertical_position),
    //                         &font,
    //                     );
    //                     *vertical_position += 2.5;
    //                 }
    //             }
    //             Paragraph { elements } => {
    //                 for paragraph_element in elements {
    //                     match paragraph_element {
    //                         Text { text, size } => {
    //                             let font_width = 0.3528 * (*size as f32) * 0.6;
    //                             let max_text_width = document.page_width
    //                                 - document.left_page_indent
    //                                 - document.right_page_indent;
    //                             let max_chars = (max_text_width / font_width) as usize;
    //                             let text_elements = split_string(text, max_chars);
    //                             for text in text_elements {
    //                                 let step: f32 = 0.3528 * *size as f32;
    //                                 if (*vertical_position + step)
    //                                     > (document.page_height - document.bottom_page_indent)
    //                                 {
    //                                     let (mut new_page, mut new_layer) = pdf.add_page(
    //                                         Mm(document.page_width),
    //                                         Mm(document.page_height),
    //                                         "Layer 1",
    //                                     );
    //                                     render_header_footer(
    //                                         pdf,
    //                                         &mut new_page,
    //                                         &mut new_layer,
    //                                         document,
    //                                     )?;
    //                                     *vertical_position = 0.0 + document.top_page_indent;
    //                                     *layer = new_layer;
    //                                     *page = new_page;
    //                                 }
    //                                 *vertical_position += step;
    //                                 let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
    //                                 let current_layer = pdf.get_page(*page).get_layer(*layer);
    //                                 current_layer.use_text(
    //                                     text,
    //                                     *size as f32,
    //                                     Mm(document.left_page_indent + 0.0),
    //                                     Mm(document.page_height - *vertical_position),
    //                                     &font,
    //                                 );
    //                             }
    //                         }
    //                         _ => { /* */ }
    //                     }
    //                 }
    //             }
    //             List { elements, numbered } => render_list(
    //                 pdf,
    //                 document,
    //                 layer,
    //                 page,
    //                 elements,
    //                 *numbered,
    //                 vertical_position,
    //                 0,
    //             )?,
    //             Table { headers, rows } => {
    //                 let mut vertical_position_max: f32 = *vertical_position;
    //                 if !headers.is_empty() {
    //                     *vertical_position += 2.5; // Additional spacing after text
    //                     let mut horizontal_position: f32 = 0.0;
    //                     let vertical_position_backup: f32 = *vertical_position;
    //                     for header in headers {
    //                         render_table_header(
    //                             header,
    //                             pdf,
    //                             page,
    //                             layer,
    //                             vertical_position,
    //                             &mut horizontal_position,
    //                             document,
    //                         )?;
    //                         horizontal_position += header.width;
    //                         if *vertical_position > vertical_position_max {
    //                             vertical_position_max = *vertical_position;
    //                         }
    //                         *vertical_position = vertical_position_backup;
    //                     }
    //                 }
    //                 *vertical_position = vertical_position_max;
    //                 for row in rows {
    //                     render_table_row(
    //                         row,
    //                         pdf,
    //                         page,
    //                         layer,
    //                         vertical_position,
    //                         headers,
    //                         document,
    //                     )?;
    //                 }
    //             }

    //             // This currently doesn't support inline, inline support will be added to Paragraph itself.
    //             Hyperlink {
    //                 title,
    //                 url,
    //                 alt: _,
    //                 size,
    //             } => {
    //                 let text = title;
    //                 let font = pdf.add_builtin_font(BuiltinFont::Courier)?;

    //                 let font_size = *size;

    //                 let font_width = 0.3528 * (font_size as f32) * 0.6;

    //                 let current_layer = pdf.get_page(*page).get_layer(*layer);

    //                 *vertical_position += 0.3528 * (font_size as f32);

    //                 let (x, y) = (
    //                     document.left_page_indent + 0.0,
    //                     document.page_height - *vertical_position,
    //                 );

    //                 current_layer.use_text(text, font_size as f32, Mm(x), Mm(y), &font);

    //                 let y = y - 0.3;

    //                 // Adding the clickable border box around the text.
    //                 current_layer.add_link_annotation(LinkAnnotation::new(
    //                     printpdf::Rect::new(
    //                         Mm(x),
    //                         Mm(y),
    //                         Mm(x + ((text.len() as f32) * font_width)),
    //                         Mm(y + font_width + 0.3),
    //                     ),
    //                     Some(printpdf::BorderArray::default()),
    //                     Some(printpdf::ColorArray::Transparent),
    //                     printpdf::Actions::uri(url.clone()),
    //                     Some(printpdf::HighlightingMode::Invert),
    //                 ));

    //                 // Insertion of UnderLine (Can be improved)
    //                 current_layer.add_link_annotation(LinkAnnotation::new(
    //                     printpdf::Rect::new(
    //                         Mm(x),
    //                         Mm(y),
    //                         Mm(x + ((text.len() as f32) * font_width)),
    //                         Mm(y),
    //                     ),
    //                     Some(printpdf::BorderArray::default()),
    //                     Some(printpdf::ColorArray::Gray([0.0])),
    //                     printpdf::Actions::uri(url.clone()),
    //                     Some(printpdf::HighlightingMode::Invert),
    //                 ));
    //             }

    //             _ => {}
    //         }

    //         Ok(())
    //     }

    //     let mut vertical_position = 0.0 + document.top_page_indent;
    // for element in &document.elements {
    //     generate_pdf(
    //         document,
    //         element,
    //         &mut pdf,
    //         &mut page1,
    //         &mut layer1,
    //         &mut vertical_position,
    //     )?;
    // }

    //     let result = pdf.save_to_bytes()?;
    //     let bytes = Bytes::from(result);
    //     Ok((bytes, HashMap::new()))
    // }
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

fn render_header_footer(
    pdf: &mut PdfDocumentReference,
    page: &mut PdfPageIndex,
    layer: &mut PdfLayerIndex,
    document: &Document,
) -> anyhow::Result<()> {
    let mut vertical_position = 0.0;
    let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
    document
        .page_header
        .iter()
        .for_each(|element| match element {
            Text { text, size } => {
                let font_size = *size as f32;
                let font_width = (0.3528 * font_size * 0.6) as f32;
                let max_text_width =
                    document.page_width - document.left_page_indent - document.right_page_indent;
                let max_chars = (max_text_width / font_width) as usize;
                let text_elements = split_string(text, max_chars);
                for text in text_elements {
                    let step: f32 = 0.3528 * font_size;
                    vertical_position += step;
                    let current_layer = pdf.get_page(*page).get_layer(*layer);
                    current_layer.use_text(
                        text,
                        font_size,
                        Mm(document.left_page_indent),
                        Mm(document.page_height - vertical_position),
                        &font,
                    );
                }
            }
            _ => {}
        });
    vertical_position = 0.0;
    document
        .page_footer
        .iter()
        .for_each(|element| match element {
            Text { text, size } => {
                let font_size = *size as f32;
                let font_width = (0.3528 * font_size * 0.6) as f32;
                let max_text_width =
                    document.page_width - document.left_page_indent - document.right_page_indent;
                let max_chars = (max_text_width / font_width) as usize;
                let text_elements = split_string(text, max_chars);
                for text in text_elements {
                    let step: f32 = 0.3528 * font_size;
                    vertical_position += step;
                    let current_layer = pdf.get_page(*page).get_layer(*layer);
                    current_layer.use_text(
                        text,
                        font_size,
                        Mm(document.left_page_indent),
                        Mm(document.bottom_page_indent - vertical_position),
                        &font,
                    );
                }
            }
            _ => {}
        });

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
    use crate::markdown;
    use crate::pdf::Transformer;
    use bytes::Bytes;
    use std::collections::HashMap;

    // #[test]
    // fn test() -> anyhow::Result<()> {
    //     let pdf = std::fs::read("test/data/document.pdf")?;
    //     let pdf_bytes = Bytes::from(pdf);
    //     let parsed = Transformer::parse(&pdf_bytes, &HashMap::new());
    //     assert!(parsed.is_ok());
    //     let parsed_document = parsed.unwrap();
    //     println!("==========================");
    //     println!("{:?}", parsed_document);
    //     println!("==========================");
    //     let generated_result = Transformer::generate(&parsed_document)?;
    //     std::fs::write("test/data/generated.pdf", generated_result.0)?;
    //     Ok(())
    // }

    #[test]
    fn test_list() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.md")?;
        let documents_bytes = Bytes::from(document);
        let parsed = markdown::Transformer::parse(&documents_bytes, &HashMap::new());
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        std::fs::write("test/data/typst.pdf", generated_result.unwrap().0)?;

        Ok(())
    }

    // #[test]
    // fn test_hyperlink_generation() -> anyhow::Result<()> {
    //     use Element::*;

    //     let document = Document {
    //         elements: vec![
    //             Paragraph {
    //                 elements: vec![
    //                     Text {
    //                         text: "Line 1".to_owned(),
    //                         size: 8,
    //                     },
    //                     Text {
    //                         text: "Line 2".to_owned(),
    //                         size: 8,
    //                     },
    //                     Text {
    //                         text: "Line 3".to_owned(),
    //                         size: 8,
    //                     },
    //                 ],
    //             },
    //             Hyperlink {
    //                 title: "Example".to_owned(),
    //                 url: "https://www.example.com".to_owned(),
    //                 alt: "Example Site".to_owned(),
    //                 size: 8,
    //             },
    //             Hyperlink {
    //                 title: "GitHub".to_owned(),
    //                 url: "https://www.github.com".to_owned(),
    //                 alt: "GitHub".to_owned(),
    //                 size: 8,
    //             },
    //         ],
    //         page_width: 210.0,
    //         page_height: 297.0,
    //         left_page_indent: 10.0,
    //         right_page_indent: 10.0,
    //         top_page_indent: 20.0,
    //         bottom_page_indent: 10.0,
    //         page_header: vec![],
    //         page_footer: vec![],
    //     };

    //     println!("==========================");
    //     println!("{:?}", document);
    //     println!("==========================");

    //     let generated_result = Transformer::generate(&document);

    //     assert!(generated_result.is_ok());

    //     std::fs::write(
    //         "test/data/generated_hyperlink.pdf",
    //         generated_result.unwrap().0,
    //     )?;

    //     Ok(())
    // }
}

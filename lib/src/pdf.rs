use crate::core::Element::{Header, Hyperlink, List, Paragraph, Table, Text};
use crate::core::{
    Document, Element, ListItem, ParserError, TableHeader, TableRow, TransformerTrait,
};
use bytes::Bytes;
use lopdf::content::Content;
use lopdf::{Document as PdfDocument, Object, ObjectId};
use printpdf::{BuiltinFont, Mm, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};
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

        render_header_footer(&mut pdf, &mut page1, &mut layer1, document)?;

        fn render_table_header(
            header: &TableHeader,
            pdf: &mut PdfDocumentReference,
            page: &mut PdfPageIndex,
            layer: &mut PdfLayerIndex,
            vertical_position: &mut f32,
            horizontal_position: &mut f32,
            document: &Document,
        ) -> anyhow::Result<()> {
            let font_size: f32 = match &header.element {
                Text { text: _, size } => size.clone() as f32,
                _ => 10.0,
            };

            let font = pdf.add_builtin_font(BuiltinFont::Courier)?;

            let max_text_width = header.width;
            let max_chars = (max_text_width / (0.3528 * font_size)) as usize;

            let text_elements: Vec<String> = match &header.element {
                Text { text, size: _ } => split_string(text, max_chars),
                _ => {
                    vec!["".to_string()]
                }
            };

            for text in text_elements {
                let step: f32 = 0.3528 * font_size;
                if (*vertical_position + step)
                    > (document.page_height - document.bottom_page_indent)
                {
                    let (mut new_page, mut new_layer) =
                        pdf.add_page(Mm(document.page_width), Mm(document.page_height), "Layer 1");
                    render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
                    *vertical_position = document.top_page_indent;
                    *page = new_page;
                    *layer = new_layer;
                }

                let current_layer = pdf.get_page(*page).get_layer(*layer);
                current_layer.use_text(
                    text,
                    font_size,
                    Mm(document.left_page_indent + *horizontal_position),
                    Mm(document.page_height - *vertical_position),
                    &font,
                );

                *vertical_position += step + 2.5; // Adjust vertical position for next element
            }

            Ok(())
        }

        fn render_table_row(
            row: &TableRow,
            pdf: &mut PdfDocumentReference,
            page: &mut PdfPageIndex,
            layer: &mut PdfLayerIndex,
            vertical_position: &mut f32,
            headers: &Vec<TableHeader>,
            document: &Document,
        ) -> anyhow::Result<()> {
            let mut horizontal_position: f32 = 0.0;

            let mut vertical_position_max: f32 = *vertical_position;
            for (i, cell) in row.cells.iter().enumerate() {
                let vertical_position_backup: f32 = *vertical_position;
                match &cell.element {
                    Text { text, size } => {
                        let font_size = *size as f32;
                        let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                        let max_text_width = headers[i].width;
                        let max_chars = (max_text_width / (0.3528 * font_size)) as usize;

                        let text_elements = split_string(text, max_chars);
                        let step: f32 = 0.3528 * font_size; // Height adjustment for text elements
                        for text in text_elements {
                            if (*vertical_position + step)
                                > (document.page_height - document.bottom_page_indent)
                            {
                                let (mut new_page, mut new_layer) = pdf.add_page(
                                    Mm(document.page_width),
                                    Mm(document.page_height),
                                    "Layer 1",
                                );
                                render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
                                *vertical_position = document.top_page_indent;
                                *page = new_page;
                                *layer = new_layer;
                            }
                            let current_layer = pdf.get_page(*page).get_layer(*layer);
                            current_layer.use_text(
                                text,
                                font_size,
                                Mm(document.left_page_indent + horizontal_position),
                                Mm(document.page_height - *vertical_position),
                                &font,
                            );
                            *vertical_position += step; // Additional vertical spacing between cells
                        }
                        horizontal_position += headers[i].width;
                        if *vertical_position > vertical_position_max {
                            vertical_position_max = *vertical_position;
                        };
                    }

                    _ => { /* */ } // Implement other element types as necessary
                }
                *vertical_position = vertical_position_backup;
            }
            *vertical_position = vertical_position_max;
            Ok(())
        }

        fn render_list(
            pdf: &mut PdfDocumentReference,
            document: &Document,
            layer: &mut PdfLayerIndex,
            page: &mut PdfPageIndex,
            elements: &Vec<ListItem>,
            numbered: bool,
            vertical_position: &mut f32,
            list_depth: usize,
        ) -> anyhow::Result<()> {
            let bullet_type = ["\u{2022} ", " ", " "];
            for (index, list_item) in elements.iter().enumerate() {
                match &list_item.element {
                    Text { text, size } => {
                        let font_width = (0.3528 * (*size as f32) * 0.6) as f32;
                        let max_text_width = document.page_width
                            - document.left_page_indent
                            - document.right_page_indent;
                        let max_chars = (max_text_width / font_width) as usize;
                        let text_elements = split_string(text, max_chars);
                        for text in text_elements {
                            let step: f32 = 0.3528 * *size as f32;
                            if (*vertical_position + step)
                                > (document.page_height - document.bottom_page_indent)
                            {
                                let (mut new_page, mut new_layer) = pdf.add_page(
                                    Mm(document.page_width),
                                    Mm(document.page_height),
                                    "Layer 1",
                                );
                                render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
                                *vertical_position = 0.0 + document.top_page_indent;
                                *layer = new_layer;
                                *page = new_page;
                            }
                            *vertical_position = *vertical_position + step;
                            let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                            let current_layer = pdf.get_page(*page).get_layer(*layer);
                            let mut item_text;
                            let left_indent = 12.0 * (list_depth as f32);

                            if numbered {
                                item_text = (index + 1).to_string();
                                item_text.push_str(&format!(". {}", text));

                                current_layer.use_text(
                                    item_text,
                                    *size as f32,
                                    Mm(document.left_page_indent + left_indent),
                                    Mm(document.page_height - *vertical_position),
                                    &font,
                                );
                            } else {
                                item_text = bullet_type[list_depth % bullet_type.len()].to_string();
                                item_text.push_str(&text);

                                match list_depth % bullet_type.len() {
                                    0 => {
                                        current_layer.use_text(
                                            item_text,
                                            *size as f32,
                                            Mm(document.left_page_indent + left_indent),
                                            Mm(document.page_height - *vertical_position),
                                            &font,
                                        );
                                    }
                                    1 => {
                                        let mut rect = Rect::new(
                                            Mm(document.left_page_indent + left_indent),
                                            Mm(document.page_height - *vertical_position + 0.5),
                                            Mm(document.left_page_indent + left_indent + 1.),
                                            Mm(document.page_height - *vertical_position + 1.5),
                                        );

                                        rect = rect.with_mode(path::PaintMode::Stroke);
                                        current_layer.add_rect(rect);

                                        current_layer.use_text(
                                            item_text,
                                            *size as f32,
                                            Mm(document.left_page_indent + left_indent + 2.),
                                            Mm(document.page_height - *vertical_position),
                                            &font,
                                        );
                                    }
                                    2 => {
                                        current_layer.add_rect(Rect::new(
                                            Mm(document.left_page_indent + left_indent),
                                            Mm(document.page_height - *vertical_position + 0.5),
                                            Mm(document.left_page_indent + left_indent + 1.),
                                            Mm(document.page_height - *vertical_position + 1.5),
                                        ));

                                        current_layer.use_text(
                                            item_text,
                                            *size as f32,
                                            Mm(document.left_page_indent + left_indent + 2.),
                                            Mm(document.page_height - *vertical_position),
                                            &font,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    List { elements, numbered } => render_list(
                        pdf,
                        document,
                        layer,
                        page,
                        elements,
                        *numbered,
                        vertical_position,
                        list_depth + 1,
                    )?,
                    _ => {}
                }
            }

            Ok(())
        }

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

                    let font_width = 0.3528 * (font_size as f32) * 0.6;
                    let max_text_width = document.page_width
                        - document.left_page_indent
                        - document.right_page_indent;
                    let max_chars = (max_text_width / font_width) as usize;
                    let text_elements = split_string(text, max_chars);
                    for text in text_elements {
                        let step: f32 = 0.3528 * font_size as f32;
                        if (*vertical_position + step)
                            > (document.page_height - document.bottom_page_indent)
                        {
                            let (mut new_page, mut new_layer) = pdf.add_page(
                                Mm(document.page_width),
                                Mm(document.page_height),
                                "Layer 1",
                            );
                            render_header_footer(pdf, &mut new_page, &mut new_layer, document)?;
                            *vertical_position = 0.0 + document.top_page_indent;
                            *layer = new_layer;
                            *page = new_page;
                        }
                        *vertical_position += step;
                        let font = pdf.add_builtin_font(BuiltinFont::Courier)?;
                        let current_layer = pdf.get_page(*page).get_layer(*layer);
                        current_layer.use_text(
                            text,
                            font_size as f32,
                            Mm(document.left_page_indent + 0.0),
                            Mm(document.page_height - *vertical_position),
                            &font,
                        );
                        *vertical_position += 2.5;
                    }
                }
                Paragraph { elements } => {
                    for paragraph_element in elements {
                        match paragraph_element {
                            Text { text, size } => {
                                println!("{size}");
                                let font_width = 0.3528 * (*size as f32) * 0.6;
                                let max_text_width = document.page_width
                                    - document.left_page_indent
                                    - document.right_page_indent;
                                let max_chars = (max_text_width / font_width) as usize;
                                let text_elements = split_string(text, max_chars);
                                for text in text_elements {
                                    let step: f32 = 0.3528 * *size as f32;
                                    if (*vertical_position + step)
                                        > (document.page_height - document.bottom_page_indent)
                                    {
                                        let (mut new_page, mut new_layer) = pdf.add_page(
                                            Mm(document.page_width),
                                            Mm(document.page_height),
                                            "Layer 1",
                                        );
                                        render_header_footer(
                                            pdf,
                                            &mut new_page,
                                            &mut new_layer,
                                            document,
                                        )?;
                                        *vertical_position = 0.0 + document.top_page_indent;
                                        *layer = new_layer;
                                        *page = new_page;
                                    }
                                    *vertical_position += step;
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
                            _ => { /* */ }
                        }
                    }
                }
                List { elements, numbered } => render_list(
                    pdf,
                    document,
                    layer,
                    page,
                    elements,
                    *numbered,
                    vertical_position,
                    0,
                )?,
                Table { headers, rows } => {
                    let mut vertical_position_max: f32 = *vertical_position;
                    if !headers.is_empty() {
                        *vertical_position += 2.5; // Additional spacing after text
                        let mut horizontal_position: f32 = 0.0;
                        let vertical_position_backup: f32 = *vertical_position;
                        for header in headers {
                            render_table_header(
                                header,
                                pdf,
                                page,
                                layer,
                                vertical_position,
                                &mut horizontal_position,
                                document,
                            )?;
                            horizontal_position += header.width;
                            if *vertical_position > vertical_position_max {
                                vertical_position_max = *vertical_position;
                            }
                            *vertical_position = vertical_position_backup;
                        }
                    }
                    *vertical_position = vertical_position_max;
                    for row in rows {
                        render_table_row(
                            row,
                            pdf,
                            page,
                            layer,
                            vertical_position,
                            headers,
                            document,
                        )?;
                    }
                }

                // This currently doesn't support inline, inline support will be added to Paragraph itself.
                Hyperlink { title, url, alt } => {
                    let text = title;
                    let font = pdf.add_builtin_font(BuiltinFont::Courier)?;

                    let font_size = 16_u8; // this is the typographical size,
                                           // currently it's set to default "16"

                    let font_width = 0.3528 * (font_size as f32) * 0.6;

                    let current_layer = pdf.get_page(*page).get_layer(*layer);

                    *vertical_position += 0.3528 * (font_size as f32);

                    let (x, y) = (
                        document.left_page_indent + 0.0,
                        document.page_height - *vertical_position,
                    );

                    current_layer.use_text(text, font_size as f32, Mm(x), Mm(y), &font);

                    let y = y - 0.3;

                    // Adding the clickable border box around the text.
                    current_layer.add_link_annotation(LinkAnnotation::new(
                        printpdf::Rect::new(
                            Mm(x),
                            Mm(y),
                            Mm(x + ((text.len() as f32) * font_width)),
                            Mm(y + font_width + 0.3),
                        ),
                        Some(printpdf::BorderArray::default()),
                        Some(printpdf::ColorArray::Transparent),
                        printpdf::Actions::uri(url.clone()),
                        Some(printpdf::HighlightingMode::Invert),
                    ));

                    // Insertion of UnderLine (Can be improved)
                    current_layer.add_link_annotation(LinkAnnotation::new(
                        printpdf::Rect::new(
                            Mm(x),
                            Mm(y),
                            Mm(x + ((text.len() as f32) * font_width)),
                            Mm(y),
                        ),
                        Some(printpdf::BorderArray::default()),
                        Some(printpdf::ColorArray::Gray([0.0])),
                        printpdf::Actions::uri(url.clone()),
                        Some(printpdf::HighlightingMode::Invert),
                    ));
                }

                _ => {}
            }

            Ok(())
        }

        let mut vertical_position = 0.0 + document.top_page_indent;
        for element in &document.elements {
            generate_pdf(
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
        let generated_result = Transformer::generate(&parsed_document)?;
        std::fs::write("test/data/generated.pdf", generated_result.0)?;
        Ok(())
    }

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
        std::fs::write("test/data/generated_list.pdf", generated_result.unwrap().0)?;

        Ok(())
    }
}

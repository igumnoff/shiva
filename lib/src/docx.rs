use crate::core::{Document, Element, ListItem, TableCell, TableRow, TransformerTrait};

use bytes::Bytes;
use docx_rs::{read_docx, Docx, Paragraph, ParagraphStyle, Run, RunChild, TableRowChild, Hyperlink, HyperlinkType, Pic, Drawing};
use std::io::Cursor;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document> {
        fn extract_text(doc_element: &docx_rs::Paragraph) -> String {
            for c in &doc_element.children {
                match c {
                    docx_rs::ParagraphChild::Run(run) => {
                        if run.children.is_empty() {
                            return "".to_string();
                        }
                        if let RunChild::Text(t) = &run.children[0] {
                            return t.text.to_string();
                        }
                    }
                    _ => {}
                }
            }
            "".to_string()
        }

        let docx = read_docx(document)?;
        const HEADING1: &str = "Heading1";
        const HEADING2: &str = "Heading2";
        const NORMAL: &str = "Normal";
        const BODY_TEXT: &str = "BodyText";
        let mut result: Vec<Element> = vec![];

        let mut is_list_numbered = false;

        let mut current_list: Option<(usize, Vec<ListItem>)> = None;

        for ch in docx.document.children {
            if let docx_rs::DocumentChild::Paragraph(par) = ch {
                if let Some(numbering_property) = &par.property.numbering_property {
                    let num_id = numbering_property
                        .id
                        .as_ref()
                        .expect("No number id in list item")
                        .id;
                    if num_id == 3 || num_id == 2 {
                        let list_text = extract_text(&par);

                        let list_item = ListItem {
                            element: Element::Text {
                                text: list_text,
                                size: 12,
                            },
                        };

                        let numbered = numbering_property
                            .id
                            .as_ref()
                            .expect("No number id in list item")
                            .id
                            == 3;
                        let level = numbering_property
                            .level
                            .as_ref()
                            .expect("Expect indent level to be Some")
                            .val;
                        if let Some((last_level, ref mut list_items)) = current_list {
                            if level > last_level {
                                let nested_list = Element::List {
                                    elements: vec![list_item],
                                    numbered,
                                };
                                list_items.push(ListItem {
                                    element: nested_list,
                                });
                            } else if level < last_level {
                                // Finish the current list and start a new one
                                result.push(Element::List {
                                    elements: list_items.clone(),
                                    numbered,
                                });
                                current_list = Some((level, vec![list_item]));
                            } else {
                                list_items.push(list_item);
                            }
                        } else {
                            current_list = Some((level, vec![list_item]));
                            is_list_numbered = numbered;
                        }
                    } else {
                        if let Some((_, list_items)) = current_list.take() {
                            result.push(Element::List {
                                elements: list_items,
                                numbered: is_list_numbered,
                            });
                        }

                        match &par.property.style {
                            Some(ParagraphStyle { val }) => match val.as_str() {
                                HEADING1 => {
                                    let text = extract_text(&par);
                                    let element = Element::Header { level: 1, text };

                                    result.push(element);
                                }
                                HEADING2 => {
                                    let text = extract_text(&par);
                                    let element = Element::Header { level: 2, text };

                                    result.push(element);
                                }

                                BODY_TEXT => {
                                    let text = extract_text(&par);
                                    let element = Element::Text { text, size: 16 };

                                    result.push(element);
                                }

                                NORMAL => {
                                    let text = extract_text(&par);
                                    let element = Element::Text { text, size: 16 };

                                    result.push(element);
                                }

                                _ => {}
                            },
                            _ => {
                                unimplemented!("Should implement");
                            }
                        }
                    }
                } else {
                    if let Some((_, list_items)) = current_list.take() {
                        result.push(Element::List {
                            elements: list_items,
                            numbered: is_list_numbered,
                        });
                    }
                    match &par.property.style {
                        Some(ParagraphStyle { val }) => match val.as_str() {
                            HEADING1 => {
                                let text = extract_text(&par);
                                let element = Element::Header { level: 1, text };

                                result.push(element);
                            }
                            HEADING2 => {
                                let text = extract_text(&par);
                                let element = Element::Header { level: 2, text };

                                result.push(element);
                            }

                            BODY_TEXT => {
                                let text = extract_text(&par);
                                let element = Element::Text { text, size: 16 };

                                result.push(element);
                            }

                            NORMAL => {
                                let text = extract_text(&par);
                                let element = Element::Text { text, size: 16 };

                                result.push(element);
                            }

                            _ => {}
                        },
                        _ => {
                            unimplemented!("Should implement");
                        }
                    }
                }
            } else {
                if let Some((_, list_items)) = current_list.take() {
                    result.push(Element::List {
                        elements: list_items,
                        numbered: is_list_numbered,
                    });
                }
                match ch {
                    docx_rs::DocumentChild::Table(table) => {
                        let mut rows = vec![];
                        for row in &table.rows {
                            let docx_rs::TableChild::TableRow(tr) = row;
                            let mut cells = TableRow { cells: vec![] };

                            for table_cell in &tr.cells {
                                let TableRowChild::TableCell(tc) = table_cell;
                                for ch in &tc.children {
                                    match ch {
                                        docx_rs::TableCellContent::Paragraph(par) => {
                                            let text = extract_text(par);
                                            cells.cells.push(TableCell {
                                                element: Element::Text { text, size: 16 },
                                            });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            rows.push(cells);
                        }

                        result.push(Element::Table {
                            headers: vec![],
                            rows,
                        });
                    }
                    _ => {}
                }
            }
        }

        if let Some((_, list_items)) = current_list.take() {
            result.push(Element::List {
                elements: list_items,
                numbered: is_list_numbered,
            });
        }

        Ok(Document::new(result))
    }


    fn generate(document: &Document) -> anyhow::Result<Bytes> {
        let mut doc = Docx::new();

        for element in &document.elements {
            match element {
                Element::Header { level, text } => {
                    let size = match level {
                        1 => 18,
                        2 => 16,
                        _ => 14,
                    };
                    doc = doc.add_paragraph(
                        Paragraph::new().add_run(Run::new().add_text(text).size(size * 2)),
                    );
                }

                Element::Text { text, size } => {
                    doc = doc.add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(text).size(*size as usize * 2)),
                    )
                }

                Element::Paragraph { elements } => {
                    for paragraph_element in elements {
                        match paragraph_element {
                            Element::Text { text, size } => {
                                doc =
                                    doc.add_paragraph(Paragraph::new().add_run(
                                        Run::new().add_text(text).size(*size as usize * 2),
                                    ));
                            }
                            _ => {
                                eprintln!("Unknown paragraph element");
                            }
                        }
                    }
                }

                Element::List { elements, numbered } => {
                    for list_item in elements {
                        match &list_item.element {
                            Element::Text { text, size } => {
                                let mut paragraph = Paragraph::new()
                                    .add_run(Run::new().add_text(text).size(*size as usize * 2));
                                if *numbered {
                                    paragraph = paragraph.style("ListNumber");
                                } else {
                                    paragraph = paragraph.style("ListBullet");
                                }
                                doc = doc.add_paragraph(paragraph);
                            }
                            Element::List { elements, numbered } => {
                                for sub_list_item in elements {
                                    match &sub_list_item.element {
                                        Element::Text { text, size } => {
                                            let mut sub_paragraph = Paragraph::new().add_run(
                                                Run::new().add_text(text).size(*size as usize * 2),
                                            );
                                            if *numbered {
                                                sub_paragraph = sub_paragraph.style("ListNumber");
                                            } else {
                                                sub_paragraph = sub_paragraph.style("ListBullet");
                                            }
                                            doc = doc.add_paragraph(sub_paragraph);
                                        }
                                        Element::List { elements, numbered } => {
                                            for sub_sub_list_item in elements {
                                                match &sub_sub_list_item.element {
                                                    Element::Text { text, size } => {
                                                        let mut sub_sub_paragraph =
                                                            Paragraph::new().add_run(
                                                                Run::new()
                                                                    .add_text(text)
                                                                    .size(*size as usize * 2),
                                                            );
                                                        if *numbered {
                                                            sub_sub_paragraph = sub_sub_paragraph
                                                                .style("ListNumber");
                                                        } else {
                                                            sub_sub_paragraph = sub_sub_paragraph
                                                                .style("ListBullet");
                                                        }
                                                        doc = doc.add_paragraph(sub_sub_paragraph);
                                                    }
                                                    _ => {
                                                        println!("unknown element in sub-sub-list");
                                                    }
                                                }
                                            }
                                        }

                                        _ => {
                                            println!("unknown element in sub-list");
                                        }
                                    }
                                }
                            }
                            Element::Header { level, text } => {
                                let size = match level {
                                    1 => 18,
                                    2 => 16,
                                    _ => 14,
                                };
                                let mut paragraph = Paragraph::new()
                                    .add_run(Run::new().add_text(text).size(size * 2));
                                if *numbered {
                                    paragraph = paragraph.style("ListNumber");
                                } else {
                                    paragraph = paragraph.style("ListBullet");
                                }
                                doc = doc.add_paragraph(paragraph);
                            }
                            Element::Hyperlink { title, url, alt, size } => {
                                let _ = alt;
                                let hyperlink = Hyperlink::new(url, HyperlinkType::External)
                                    .add_run(Run::new().add_text(url).size(*size as usize * 2));
                                let paragraph = Paragraph::new()
                                    .add_run(Run::new().add_text(title).size(*size as usize * 2));

                                doc = doc.add_paragraph(Paragraph::add_hyperlink(paragraph, hyperlink));
                            }
                            _ => {
                                println!("unknown element in list");
                            }
                        }
                    }
                }


                Element::Hyperlink { title, url, alt, size } => {
                    let _ = alt;
                    let hyperlink = Hyperlink::new(url, HyperlinkType::External)
                        .add_run(Run::new().add_text(url).size(*size as usize * 2));
                    let paragraph = Paragraph::new()
                        .add_run(Run::new().add_text(title).size(*size as usize * 2));

                    doc = doc.add_paragraph(Paragraph::add_hyperlink(paragraph, hyperlink));
                }

                Element::Image { bytes, title, alt, image_type} => {
                    let mut pic = Pic::new(&bytes);

                    // Устанавливаем максимальный размер изображения (в EMU)
                    let max_width = 5900000; // 16.5 cm
                    let max_height = 10629420; // 29.7 cm

                    // Получаем текущий размер изображения
                    let (width, height) = pic.size;

                    // Масштабируем изображение, если оно превышает размеры страницы
                    let mut new_width = width;
                    let mut new_height = height;

                    // Масштабируем изображение, если оно превышает размеры страницы
                    if width > max_width {
                        let ratio = max_width as f32 / width as f32;
                        new_width = (width as f32 * ratio) as u32;
                        new_height = (height as f32 * ratio) as u32;
                    }
                    if new_height > max_height {
                        let ratio = max_height as f32 / new_height as f32;
                        new_width = (new_width as f32 * ratio) as u32;
                        new_height = (new_height as f32 * ratio) as u32;
                    }

                    pic = pic.size(new_width, new_height);

                    let paragraph = Paragraph::new()
                        .add_run(Run::new().add_image(pic));

                    doc = doc.add_paragraph(paragraph);
                }

                Element::Table { headers, rows } => {
                    let mut table_rows = Vec::new();

                    if !headers.is_empty() {
                        let mut header_cell: Vec<docx_rs::TableCell> = Vec::new();
                        for header in headers {
                            if let Element::Text { text, size } = &header.element {
                                let cell = docx_rs::TableCell::new().add_paragraph(
                                    Paragraph::new().add_run(
                                        Run::new().add_text(text).size(*size as usize * 2),
                                    ),
                                );
                                header_cell.push(cell);
                            }
                        }
                        let header_row = docx_rs::TableRow::new(header_cell);
                        table_rows.push(header_row)
                    }

                    for row in rows {
                        let mut rows_cell = Vec::new();

                        for cell in &row.cells {
                            if let Element::Text { text, size } = &cell.element {
                                let table_cell = docx_rs::TableCell::new().add_paragraph(
                                    Paragraph::new().add_run(
                                        Run::new().add_text(text).size(*size as usize * 2),
                                    ),
                                );
                                rows_cell.push(table_cell);
                            }
                        }
                        let table_row = docx_rs::TableRow::new(rows_cell);
                        table_rows.push(table_row);
                    }
                    let table = docx_rs::Table::new(table_rows);
                    doc = doc.add_table(table);
                }
                _ => {
                    eprintln!("Unknown element");
                }
            }
        }

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        doc.build().pack(&mut cursor)?;
        let buffer = cursor.into_inner();

        Ok(bytes::Bytes::from(buffer))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{disk_image_loader, TransformerWithImageLoaderSaverTrait};
    use crate::{docx, markdown};
    use bytes::Bytes;

    #[test]
    fn test() -> anyhow::Result<()> {
        // read from document.docx file from disk
        let document = std::fs::read("test/data/document.md")?;
        let documents_bytes = Bytes::from(document);
        let parsed = markdown::Transformer::parse_with_loader(
            &documents_bytes,
            disk_image_loader("test/data"),
        )?;

        let generated_result = docx::Transformer::generate(&parsed)?;
        //write to file
        println!("--->>>{:<12} - start writing *.docx", "TEST");
        std::fs::write("test/data/document_from_md.docx", generated_result)?;

        Ok(())
    }
}

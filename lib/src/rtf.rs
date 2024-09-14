use std::io::Cursor;
use bytes::Bytes;
use image::GenericImageView;
use crate::core::{Document, Element, TableHeader, TableRow, TransformerTrait};
use image::io::Reader as ImageReader;

use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;


pub struct Transformer;

struct ImageSize {
    output_width: u32,
    output_height: u32,
}

fn re_size_picture(image_bytes: &Bytes) -> ImageSize {
    //setting the maximum image size
    let max_width = 9700; // 16.5 cm
    let max_height = 18000; // 29.7 cm

    let size_img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format().expect("the image format is not defined")
        .decode().expect("fail decode image");
    let (width, height) = size_img.dimensions();

    //reassigning the dimensions taking into account the coefficients
    let width = width * 15;
    let height = height * 15;

    //scale the image if it exceeds the page size
    let mut new_width = width;
    let mut new_height = height;

    //scale the image if it exceeds the page size
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

    let output_width = new_width;
    let output_height = new_height;

    ImageSize {
        output_width,
        output_height
    }
}

fn detect_element_in_list(
    rtf_content: &mut String,
    element: &Element,
    numbered: bool,
    parent_indices: &mut Vec<usize>,
    depth: usize,
) {
    match element {
        Element::Text { text, size } => {
            let indent = " ".repeat(depth * 4); // 4 пробела для каждого уровня вложенности
            let modified_text = if numbered {
                let numbering = parent_indices.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
                format!("{}{}. {}", indent, numbering, text)
            } else {
                format!("{}- {}", indent, text)
            };
            rtf_content.push_str(&format!(
                "{{\\fs{} {}}} ",
                *size as i32 * 2,
                modified_text
            ));
            rtf_content.push_str("\\par ");
        }

        Element::Header { level, text } => {
            let header_size = 30 + (level);
            let indent = " ".repeat(depth * 4); // 4 пробела для каждого уровня вложенности
            let modified_text = if numbered {
                let numbering = parent_indices.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
                format!("{}. {} ", numbering, text)
            } else {
                format!("{}- {}", indent, text)
            };
            rtf_content.push_str(&format!(
                "{{\\fs{}\\b {} \\b0}}\\par ",
                header_size,
                modified_text
            ));
        }

        Element::Hyperlink { title, url, .. } => {
            let indent = " ".repeat(depth * 4);
            let modified_title = if numbered {
                let numbering = parent_indices.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
                format!("{}{}. {}", indent, numbering, title)
            } else {
                format!("{}- {}", indent, title)
            };
                rtf_content.push_str(&format!(
                "{{\\field{{\\*\\fldinst HYPERLINK \"{}\" }}{{\\fldrslt {{\\ul\\cf1 {}}}}}}}",
                url,
                modified_title
            ));
            rtf_content.push_str("\\par ");
        }

        Element::Image(image) => {
            let image_bytes = image.bytes();
            let image_size = re_size_picture(image_bytes);
            let image = image_bytes.iter().map(|b| format!("{:02X}",
                                                           b)).collect::<String>();

            let indent = " ".repeat(depth * 4);
            let modified_image_caption = if numbered {
                let numbering = parent_indices.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
                format!("{}{}. {}", indent, numbering, "Image")
            } else {
                format!("{}- Image", indent)
            };

            rtf_content.push_str(&format!("{{\\fs24 {}}}\\par ", modified_image_caption));

            rtf_content.push_str(&format!(
                "{{{{\\pict\\jpegblip\\picwgoal{}\\pichgoal{} {} }}}}",
                image_size.output_width,
                image_size.output_height,
                image
            ));
            rtf_content.push_str("\\par ");
        }

        Element::List { elements, numbered } => {
            if *numbered {
                parent_indices.push(0); // Добавляем новый уровень для вложенного списка
            }
            for list_item in elements {
                if *numbered {
                    *parent_indices.last_mut().unwrap() += 1; // Увеличиваем индекс текущего уровня
                }
                detect_element_in_list(rtf_content, &list_item.element, *numbered, parent_indices, depth + 1);
            }
            if *numbered {
                parent_indices.pop(); // Удаляем уровень после обработки вложенного списка
            }
        }

        _ => {
            eprintln!("Unknown element in list: {:?}", element);
        }
    }
}




impl TransformerTrait for Transformer {
    fn parse(
        document: &bytes::Bytes,
    ) -> anyhow::Result<Document>
    {
        let data_str = std::str::from_utf8(document).unwrap();
        let tokens = Lexer::scan(&data_str).unwrap();
    
        // keeping the document in a box since it might contain huge data and also
        // for easy manipulation
        let mut document: Document = Document::new(vec![]);
        // initializing header levels
        let mut level = 1;
        for styleblock in Parser::new(tokens).parse().unwrap().body.as_slice() {
            if styleblock.painter.font_size >= 30 && styleblock.painter.bold == true {
                document.add_element(Element::Header {
                    level: level,
                    text: styleblock.text.to_owned(),
                });
                level += 1
            } else {
                {
                    document.add_element(Element::Paragraph {
                        elements: vec![Element::Text {
                            text: styleblock.text.to_owned(),
                            size: styleblock.painter.font_size as u8,
                        }],
                    })
                }
            }   
    }
    Ok(document)
    }

    fn generate(document: &Document, ) -> anyhow::Result<bytes::Bytes> {
        let mut rtf_content = String::new();
        let mut parent_indices = Vec::new();

        rtf_content.push_str("{\\rtf1\\ansi\\deff0"); //the standard title of an RTF document, which indicates that it is an RTF document using ANSI characters and the default font
        for element in &document.get_all_elements() {
            match element {

                Element::Header { level, text} => {
                    let header_size = 30 + (level);

                    //formatting the string RTF
                    rtf_content.push_str(&format!(
                        "{{\\fs{}\\b {} \\b0}}\\par ",
                        header_size,
                        text
                    ));
                }

                Element::Text { text, size} => {
                    rtf_content.push_str(&format!(
                        "{{\\fs{} {}}} ",
                        *size as i32 * 2,
                        text
                    ));
                }

                Element::Paragraph { elements } => {
                    for elem in elements {
                        if let Element::Text { text, size } = elem {
                            rtf_content.push_str(&format!(
                                "{{\\fs{} {}}}",
                                *size as i32 * 2,
                                text
                            ));
                        }
                    }
                    rtf_content.push_str("\\par ");
                }

                Element::List { elements, numbered } => {
                    if *numbered {
                        parent_indices.push(0); // Начинаем с 0 для нового списка
                    }
                    for list_item in elements {
                        if *numbered {
                            *parent_indices.last_mut().unwrap() += 1; // Увеличиваем текущий уровень нумерации
                        }

                        // Если элемент является вложенным списком, уменьшаем индекс родительского уровня
                        if let Element::List { .. } = list_item.element {
                            if *numbered {
                                *parent_indices.last_mut().unwrap() -= 1;
                            }

                        }
                        detect_element_in_list(&mut rtf_content,
                                               &list_item.element,
                                               *numbered,
                                               &mut parent_indices,
                                               0);

                    }
                    if *numbered {
                        parent_indices.pop(); // Удаляем уровень после обработки списка
                    }
                }

                Element::Hyperlink { title, url, alt: _, size: _} => {
                    rtf_content.push_str(&format!(
                        "{{\\field{{\\*\\fldinst HYPERLINK \"{}\" }}{{\\fldrslt {{\\ul\\cf1 {}}}}}}}",
                        url,
                        title
                    ));
                    rtf_content.push_str("\\par ");
                }

                Element::Image(image) => {
                    let image_bytes = image.bytes();

                    let image_size = re_size_picture(image_bytes);

                    let image = image_bytes.iter().map(|b| format!("{:02X}",
                                                             b)).collect::<String>();

                    rtf_content.push_str(&format!(
                        "{{{{\\pict\\jpegblip\\picwgoal{}\\pichgoal{} {} }}}}",
                        image_size.output_width,
                        image_size.output_height,
                        image
                    ));
                    rtf_content.push_str("\\par ");
                }

                Element::Table { headers, rows } => {
                    let column_widths = calculate_column_widths(headers, rows);
                    let mut current_x = 0;

                    rtf_content.push_str("\\trowd\\trgaph108\\trleft-108");
                    for (_, width) in column_widths.iter().enumerate() {
                        current_x += width;
                        rtf_content.push_str("\\clbrdrt\\brdrs\\brdrw10\\clbrdrl\\brdrs\\brdrw10\\clbrdrb\\brdrs\\brdrw10\\clbrdrr\\brdrs\\brdrw10");
                        rtf_content.push_str(&format!("\\cellx{}", current_x));
                    }
                    rtf_content.push_str("\\intbl");

                    for header in headers {
                        if let Element::Text { text, size } = &header.element {
                            rtf_content.push_str(&format!("{{\\fs{} {}}}\\cell", *size as i32 * 2, text));
                        }
                    }
                    rtf_content.push_str("\\row");

                    for row in rows {
                        for cell in &row.cells {
                            if let Element::Text { text, size } = &cell.element {
                                rtf_content.push_str(&format!("{{\\fs{} {}}}\\cell", *size as i32 * 2, text));
                            }
                        }
                        rtf_content.push_str("\\row");
                    }
                }

                _other_element => {
                    eprintln!("Unknown element in list: {:?}", element);
                }
            }
        }

        rtf_content.push_str("}");

        Ok(bytes::Bytes::from(rtf_content.into_bytes()))

    }
}

fn calculate_column_widths(headers: &Vec<TableHeader>, rows: &Vec<TableRow>) -> Vec<i32> {
    let max_width = 9700;
    let mut column_widths: Vec<i32> = headers.iter().map(|_| 0).collect();
    let mut column_content_lengths: Vec<usize> = headers.iter().map(|_| 0).collect();

    for (i, header) in headers.iter().enumerate() {
        if let Element::Text { text, .. } = &header.element {
            column_content_lengths[i] = text.len().max(column_content_lengths[i]);
        }
    }

    for row in rows {
        for (i, cell) in row.cells.iter().enumerate() {
            if let Element::Text { text, .. } = &cell.element {
                column_content_lengths[i] = text.len().max(column_content_lengths[i]);
            }
        }
    }

    let total_length: usize = column_content_lengths.iter().sum();
    let total_length = total_length as i32;

    if total_length == 0 {
        return column_widths;
    }

    for (i, &length) in column_content_lengths.iter().enumerate() {
        column_widths[i] = (length as f32 / total_length as f32 * max_width as f32) as i32;
    }

    column_widths
}

#[cfg(test)]

mod tests {
    use bytes::Bytes;
    use crate::{markdown};
    use crate::core::{disk_image_loader, TransformerWithImageLoaderSaverTrait};

    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.md")?;
        let documents_bytes = Bytes::from(document);
        let parsed = markdown::Transformer::parse_with_loader(&documents_bytes,disk_image_loader("test/data"))?;
        let generated_result = crate::rtf::Transformer::generate(&parsed)?;
        std::fs::write("test/data/document_from_rtf.rtf", generated_result)?;

        Ok(())
    }
}


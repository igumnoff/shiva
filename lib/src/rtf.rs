use std::io::Cursor;
use image::GenericImageView;
use crate::core::{Document, Element, TransformerTrait};
use image::io::Reader as ImageReader;

use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;




pub struct Transformer;

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
                document.elements.push(Element::Header {
                    level: level,
                    text: styleblock.text.to_owned(),
                });
                level += 1
            } else {
                {
                    document.elements.push(Element::Paragraph {
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
    fn generate(
        document: &Document,
    ) -> anyhow::Result<
        bytes::Bytes
    > {
        let mut rtf_content = String::new();

        rtf_content.push_str("{\\rtf1\\ansi\\deff0"); //the standard title of an RTF document, which indicates that it is an RTF document using ANSI characters and the default font
        for element in &document.elements {
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
                /*
                                Element::List {elements, numbered} => {
                                    todo!()
                                }
                */
                Element::Hyperlink { title, url, alt: _, size: _} => {
                    rtf_content.push_str(&format!(
                        "{{\\field{{\\*\\fldinst HYPERLINK \"{}\" }}{{\\fldrslt {{\\ul\\cf1 {}}}}}}}",
                        url,
                        title
                    ));
                    rtf_content.push_str("\\par ");
                }

                Element::Image { bytes, title: _, alt: _, image_type: _} => {
                    //определяем размеры изображения
                    //setting the maximum image size
                    let max_width = 9700; // 16.5 cm
                    let max_height = 18000; // 29.7 cm

                    let size_img = ImageReader::new(Cursor::new(bytes))
                        .with_guessed_format()?.decode()?;
                    let (width, height) = size_img.dimensions();

                    //переназначаем размеры с учётом коэффициентов
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

                    let output_width_size = new_width;
                    let output_height_size = new_height;


                    let image = bytes.iter().map(|b| format!("{:02X}",
                                                             b)).collect::<String>();


                    rtf_content.push_str(&format!(
                        "{{{{\\pict\\jpegblip\\picwgoal{}\\pichgoal{} {} }}}}",
                        output_width_size,
                        output_height_size,
                        image
                    ));
                    rtf_content.push_str("\\par ");
                }

                Element::Table { headers, rows } => {
                    // Начало таблицы
                    rtf_content.push_str("\\trowd \\trgaph100");

                    // Добавление заголовков таблицы
                    for (i, header) in headers.iter().enumerate() {
                        if let Element::Text { text, size } = &header.element {
                            rtf_content.push_str(&format!(
                                "\\cellx{} {{\\b\\fs{} {}}} ",
                                (i + 1) * 1500, // Ширина ячейки
                                size * 2, // Размер шрифта
                                text // Текст заголовка
                            ));
                        }
                    }
                    rtf_content.push_str("\\row ");

                    // Добавление строк таблицы
                    for row in rows {
                        rtf_content.push_str("\\trowd \\trgaph100");
                        for (i, cell) in row.cells.iter().enumerate() {
                            if let Element::Text { text, size } = &cell.element {
                                rtf_content.push_str(&format!(
                                    "\\cellx{}\\fs{} {} ",
                                    (i + 1) * 1500, // Ширина ячейки
                                    size * 2, // Размер шрифта
                                    text // Текст ячейки
                                ));
                            }
                        }
                        rtf_content.push_str("\\row ");
                    }

                    // Конец таблицы
                    rtf_content.push_str("\\pard ");
                }



                _ => {
                    eprintln!("Unknown element");
                }
            }
        }

        rtf_content.push_str("}");

        Ok(bytes::Bytes::from(rtf_content.into_bytes()))

    }
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


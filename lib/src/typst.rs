use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};

use bytes::Bytes;
use anyhow;
use spreadsheet_ods::text;
use typst_pdf;
use std::{collections::HashMap, io::Read};
use crate::core::{
    Document, Element, ImageType, ListItem, TableHeader, TableRow, TransformerTrait,
};
use time::{OffsetDateTime, UtcOffset};
use std::path::Path;
use comemo::Prehashed;


use typst::{
    diag::{FileError, FileResult},
    foundations::Datetime,
    eval::Tracer,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    Library, World,

};


type TypstString = String;

pub struct ShivaWorld {
    fonts: Vec<Font>,
    book: Prehashed<FontBook>,
    library: Prehashed<Library>,
    source: Source,
    img_map: HashMap<String, typst::foundations::Bytes>,
}

impl ShivaWorld {
    pub fn new<T>(source: T, img_map: HashMap<String, typst::foundations::Bytes>) -> Self where T: Into<String> {
        let source = Source::detached(source);
        
        let folder = "fonts";
        
        // Check if the "fonts" folder exists
        if !std::path::Path::new(folder).exists() {
            // Create the "fonts" folder
            std::fs::create_dir_all(folder).expect("Failed to create folder");

            // Download fonts
            let font_info = vec![
                ("DejaVuSansMono-Bold.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/DejaVuSansMono-Bold.ttf"),
                ("DejaVuSansMono.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/DejaVuSansMono.ttf"),
                ("FiraMath-Regular.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/FiraMath-Regular.otf"),
                ("IBMPlexSerif-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/IBMPlexSerif-Regular.ttf"),
                ("InriaSerif-BoldItalic.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/InriaSerif-BoldItalic.ttf"),
                ("InriaSerif-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/InriaSerif-Regular.ttf"),
                ("LinLibertine_R.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/LinLibertine_R.ttf"),
                ("LinLibertine_RB.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/LinLibertine_RB.ttf"),
                ("LinLibertine_RBI.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/LinLibertine_RBI.ttf"),
                ("LinLibertine_RI.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/LinLibertine_RI.ttf"),
                ("Nerd.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/Nerd.ttf"),
                ("NewCM10-Bold.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NewCM10-Bold.otf"),
                ("NewCM10-Regular.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NewCM10-Regular.otf"),
                ("NewCMMath-Book.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NewCMMath-Book.otf"),
                ("NewCMMath-Regular.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NewCMMath-Regular.otf"),
                ("NotoColorEmoji.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoColorEmoji.ttf"),
                ("NotoSansArabic-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoSansArabic-Regular.ttf"),
                ("NotoSansSymbols2-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoSansSymbols2-Regular.ttf"),
                ("NotoSerifCJKsc-Regular.otf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoSerifCJKsc-Regular.otf"),
                ("NotoSerifHebrew-Bold.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoSerifHebrew-Bold.ttf"),
                ("NotoSerifHebrew-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/NotoSerifHebrew-Regular.ttf"),
                ("PTSans-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/PTSans-Regular.ttf"),
                ("Roboto-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/Roboto-Regular.ttf"),
                ("TwitterColorEmoji.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/TwitterColorEmoji.ttf"),
                ("Ubuntu-Regular.ttf", "https://github.com/igumnoff/shiva/raw/main/lib/fonts/Ubuntu-Regular.ttf"),
            ];

            for (filename, url) in font_info {
                download_font(url, folder, filename);
            }
        }
        
        let fonts = std::fs::read_dir(folder)
            .unwrap()
            .map(Result::unwrap)
            .flat_map(|entry| {
                let path = entry.path();
                let bytes = std::fs::read(&path).unwrap();
                let buffer = typst::foundations::Bytes::from(bytes);
                let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
                (0..face_count).map(move |face| {
                    Font::new(buffer.clone(), face).unwrap_or_else(|| {
                        panic!("failed to load font from {path:?} (face index {face})");
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

fn download_font(url: &str, folder: &str, filename: &str) {
    let font_path = Path::new(folder).join(filename);

    println!("Downloading font file {}...", font_path.display());

    let mut reader = ureq::get(url)
        .call().unwrap()
        .into_reader();

    let f = std::fs::File::create(&font_path).unwrap();
    let mut writer = std::io::BufWriter::new(f);

    let _bytes_io_count =
        std::io::copy(&mut reader, &mut writer).unwrap();


    println!("Font file {} downloaded successfully!", font_path.display());
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
    /*
    This parser utilizes typst built in parsing function that outputs file contents into a Document type
    After this we convert their type into our intermediate repr

    Note: 
    Current document is missing things like fonts and features like bold and underlined text
    */
    fn parse(document: &bytes::Bytes) -> anyhow::Result<Document> {

        fn process_element(frame: &typst::layout::Frame) -> Element {
            // Iterating over frame items

            let mut res = Vec::with_capacity(frame.items().len());

            for (_point, item) in frame.items() {
                res.push(match item {
                    typst::layout::FrameItem::Group(group_item) => {
                        process_group(group_item)
                    },
                    typst::layout::FrameItem::Text(text_item) => {
                        process_text(text_item)
                    },
                    typst::layout::FrameItem::Shape(shape, span) => {
                        process_shape(shape, span)
                    },
                    typst::layout::FrameItem::Image(image, axes, span) => {
                        process_image(image, axes, span)
                    },
                    typst::layout::FrameItem::Meta(meta, _) => {
                        process_meta(meta)
                    },
                })
            }
            Paragraph { elements: res }
        }

        fn process_text(text_item: &typst::text::TextItem) -> Element {
            Element::Text { text: text_item.text.to_string(), size: text_item.size.to_pt().round() as u8}
        }

        fn process_image(image: &typst::visualize::Image, _axes: &typst::layout::Axes<typst::layout::Abs>, _span: &typst_syntax::Span) -> Element {
            // Image kind is wrong, we do not support all image kinds yet. throw error?
            let image_type = match image.kind() {
                typst::visualize::ImageKind::Raster(_) => ImageType::Jpeg,
                typst::visualize::ImageKind::Svg(_) => ImageType::Png,
            };
            Element::Image { bytes: bytes::Bytes::from(image.data().to_vec()),
                             title: String::new(),
                             alt: String::from(image.alt().unwrap_or("")), 
                             image_type}
        }

        fn process_meta(meta: &typst::introspection::Meta) -> Element {
            match meta {
                // Unable to complete this lines as document does not support them
                typst::introspection::Meta::Link(dest) => {
                    match dest {
                        typst::model::Destination::Url(string) => {
                            Hyperlink { title: String::new(), url: string.to_string(), alt: String::new(), size: 12 }
                        },
                        typst::model::Destination::Position(_) => todo!(),
                        typst::model::Destination::Location(_) => todo!(),
                    }
                },
                typst::introspection::Meta::Elem(_) => todo!(),
                typst::introspection::Meta::Hide => todo!(),
            }
        }

         fn process_shape(shape: &typst::visualize::Shape, span: &typst_syntax::Span) -> Element {
            match shape.geometry {
                
            }
         }

         fn process_group(group_item: &typst::layout::GroupItem) -> Element {
            process_element(&group_item.frame)
         }

        let world = ShivaWorld::new(String::from_utf8_lossy(document), HashMap::new());

        let mut tracer = Tracer::new();
        // Using typst compiler to read file into intermediate repr
        let doc = typst::compile(&world, &mut tracer).unwrap();

        let warnings = tracer.warnings();
        if !warnings.is_empty() {// Trowing any warnings if necessary
            for warn in warnings {
                println!("Warning - {}", warn.message);
            }
        }

        // Converting from typst document format to ours
        let mut pages:Vec<Element>  = Vec::with_capacity(doc.pages.len());

        // Iterating over pages
        // Panics if page numbers in document are set wrong
        for typst::layout::Page {frame, numbering: _, number} in doc.pages {
            pages.insert(number, process_element(&frame))
        }

        Ok(Document::new(pages))
    }

    fn generate(document: &Document) -> anyhow::Result<bytes::Bytes> {
        let (text, _) = generate_document(document)?;
        let bytes = Bytes::from(text);
        Ok(bytes)
    }
}

/// Converts Document into a typst::model::Document
pub fn generate_document(document: &Document) -> anyhow::Result<(TypstString, HashMap<String, typst::foundations::Bytes>)> {
    // Array of methods to process Document object into a typst string repr
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
            let image_text = format!("
            #image(\"{title}{image_type}\", alt: \"{alt}\")
            "
            );
            source.push_str(&image_text);
        }
        // need to think how to implement using raw bytes
        Ok(())
    }

    fn process_element (
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
            // _ => {
            //     eprintln!("Should implement element - {:?}", element);
            //     Ok(())
            // }
        }
    }

    // String to build off of
    let mut source = TypstString::new();
    // Mapping of connections between elements
    let mut img_map: HashMap<String, typst::foundations::Bytes> = HashMap::new();


    // Converting both headers and footers into a string repr of them in Typst
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

    // Converting Document repr to one of typst string
    source.push_str(&footer_header_text);
    for element in &document.elements {
        process_element(&mut source, &mut img_map, element)?;
    }

    Ok((source, img_map))
}

#[cfg(test)]
mod test {
    use bytes::Bytes;
    use crate::markdown;
    use crate::core::{disk_image_loader, TransformerWithImageLoaderSaverTrait};

    use super::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {

        Ok(())
    }

    #[test]
    fn test_generate() -> anyhow::Result<()> {
        let document = std::fs::read("test/data/document.md")?;
        let documents_bytes = Bytes::from(document);
        let parsed = markdown::Transformer::parse_with_loader(&documents_bytes,disk_image_loader("test/data"))?;
        let generated_result = crate::typst::Transformer::generate(&parsed)?;
        std::fs::write("test/data/document_from_md.typ", generated_result)?;

        Ok(())
    }
}
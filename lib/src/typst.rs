use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};

use bytes::Bytes;
use anyhow;
use typst_pdf;
use std::collections::HashMap;
use crate::core::{
    Document, Element, ImageType, ListItem, TableHeader, TableRow, TransformerTrait,
};
use time::{OffsetDateTime, UtcOffset};
use std::path::Path;
use comemo::Prehashed;


use typst::{
    diag::{FileError, FileResult},
    foundations::{Datetime, Smart},
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
    #[allow(unused)]
    fn parse(document: &bytes::Bytes) -> anyhow::Result<Document> {
        todo!()
    }

    fn generate(document: &Document) -> anyhow::Result<bytes::Bytes> {
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

        // Creating a enviroment object 
        let world = ShivaWorld::new(source, img_map);
        let mut tracer = Tracer::default();

        let document = typst::compile(&world, &mut tracer).unwrap();
        let warnings = tracer.warnings();

        if !warnings.is_empty() {// Trowing any warnings if necessary
            for warn in warnings {
                println!("Warning - {}", warn.message);
            }
        }

        // Converting to pdf then to bytes
        let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
        let bytes = Bytes::from(pdf);
        Ok(bytes)
    }
}

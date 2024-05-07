use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use crate::core::{
    Document, Element, ImageType, ListItem, ParserError, TableHeader, TableRow, TransformerTrait,
};

use bytes::{Buf, Bytes};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::{
    collections::HashMap,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};
use zip::ZipArchive;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        fn parse_docx_xml(content: Vec<u8>) {
        let mut xml = Reader::from_reader(content.reader());

            let mut buf = Vec::new();
            // let mut txt = Vec::new();
            let mut count = 0;
            
            loop {
                // NOTE: this is the generic case when we don't know about the input BufRead.
                // when the input is a &str or a &[u8], we don't actually need to use another
                // buffer, we could directly call `reader.read_event()`
                let read_data = xml.read_event_into(&mut buf) ;

                println!("read_data - {:?}", read_data);
                match read_data {
                    // Err(e) => panic!("Error at position {}: {:?}", xml.buffer_position(), e),
                    // exits the loop when reaching end of file
                    Ok(Event::Eof) => break,
                    // Ok(Event::Start(e)) => {
                    //     println!("attributes values: {:?}", e)
                    // }
                    // Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
    
                    // There are several other `Event`s we do not consider here
                    _ => {},
                }


                buf.clear();
            }
        }

        let mut elements: Vec<Element> = Vec::new();

        let cursor = Cursor::new(document);
        let mut zip = ZipArchive::new(cursor).expect("Wasn't able to read document");
        let mut main_document = zip
            .by_name("word/document.xml")
            .expect("Didn't find document xml file in docx");

        let mut content = vec![];

        main_document
            .read_to_end(&mut content)
            .expect("Couldn't read content of document into buffer");

        // let mut xml = Reader::from_reader(content.reader());
        parse_docx_xml(content);

        // loop {
        //     // NOTE: this is the generic case when we don't know about the input BufRead.
        //     // when the input is a &str or a &[u8], we don't actually need to use another
        //     // buffer, we could directly call `reader.read_event()`
        //     match xml.read_event_into(&mut buf) {
        //         Err(e) => panic!("Error at position {}: {:?}", xml.buffer_position(), e),
        //         // exits the loop when reaching end of file
        //         Ok(Event::Eof) => break,
        //         Ok(Event::Start(e)) => match e.name().as_ref() {
        //             b"w:p" => {
        //                 // let b = e.bytes().fold(String::new(), |mut acc, e| {
        //                 //     let e = e.unwrap();

        //                 //     let e = e.ch;
        //                 //     acc.push_str(&e);
        //                 //     acc
        //                 // });
        //                 println!("attributes values: {:?}", b)
        //             }
        //             b"tag2" => count += 1,
        //             _ => {
        //                 println!("e - {:?}", e);
        //             }
        //         },
        //         Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

        //         // There are several other `Event`s we do not consider here
        //         _ => (),
        //     }
        //     buf.clear();
        // }

        // println!("{:?}", txt);
        todo!()
    }
    fn generate(_document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::TransformerTrait;
    use crate::docx;
    use bytes::Bytes;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        // read from document.docx file from disk
        let document = std::fs::read("test/data/document.docx")?;
        let bytes = Bytes::from(document);
        let images = HashMap::new();
        let parsed = docx::Transformer::parse(&bytes, &images);
        assert!(parsed.is_ok());
        Ok(())
    }
}

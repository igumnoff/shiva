use crate::core::{Document, TransformerTrait};
use anyhow::Ok;
use bytes::Bytes;
use serde_xml_rs::{from_str, to_string};
use std::collections::HashMap;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let doc_string = String::from_utf8(document.to_vec())?;
        let doc: Document = from_str(&doc_string)?;
        return Ok(doc);
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let result = to_string(document).unwrap();
        Ok((Bytes::from(result), HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Document, TransformerTrait};
    use crate::json;
    use crate::xml;
    use bytes::Bytes;
    use std::collections::HashMap;
    #[test]
    fn test() -> anyhow::Result<()> {
        // let document = std::fs::read("test/data/document_xml.xml")?;
        // let bytes = Bytes::from(document);
        // // println!("{:?}", document);
        // let mut images = HashMap::new();
        // let image_bytes = std::fs::read("test/data/picture.png")?;
        // images.insert("test/data/picture.png".to_string(), image_bytes);
        // let parsed = xml::Transformer::parse(&bytes, &HashMap::new());
        // let document_string = std::str::from_utf8(&bytes)?;
        // println!("{}", document_string);
        // assert!(parsed.is_ok());
        // let parsed_document = parsed.unwrap();
        // println!("==========================");
        // println!("{:?}", parsed_document);
        let document = r#"{"elements":[{"Header":{"level":1,"text":"First header"}},{"Paragraph":{"elements":[{"Text":{"text":"Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla","size":8}},{"Text":{"text":"blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla","size":8}}]}},{"List":{"elements":[{"element":{"Text":{"text":"List item 1","size":8}}},{"element":{"Text":{"text":"List item 2","size":8}}},{"element":{"Text":{"text":"List item 3","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 1","size":8}}},{"element":{"Text":{"text":"List item secode level 2","size":8}}}],"numbered":true}}},{"element":{"Text":{"text":"List item 4","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 3","size":8}}},{"element":{"Text":{"text":"List item secode level 4","size":8}}}],"numbered":true}}},{"element":{"Text":{"text":"List item 5","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 5","size":8}}}],"numbered":true}}}],"numbered":true}},{"List":{"elements":[{"element":{"Text":{"text":"List item one","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item two","size":8}}}],"numbered":false}}},{"element":{"Text":{"text":"List item three","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item four","size":8}}},{"element":{"Text":{"text":"List item five","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item zzz","size":8}}}],"numbered":false}}}],"numbered":false}}},{"element":{"Text":{"text":"List item six","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item seven","size":8}}}],"numbered":false}}}],"numbered":false}},{"Paragraph":{"elements":[{"Image":{"bytes":[],"title":"Picture title1","alt":"Picture alt1","image_type":"Png"}}]}},{"Paragraph":{"elements":[{"Text":{"text":"Bla bla bla ","size":8}},{"Image":{"bytes":[],"title":"Picture title2","alt":"Picture alt2","image_type":"Png"}},{"Text":{"text":" bla. ","size":8}},{"Hyperlink":{"title":"http://example.com","url":"http://example.com","alt":"http://example.com","size":8}},{"Text":{"text":"  ","size":8}},{"Hyperlink":{"title":"Example","url":"http://example.com","alt":"Example","size":8}},{"Text":{"text":" ","size":8}},{"Hyperlink":{"title":"Example","url":"http://example.com","alt":"Example tooltip","size":8}}]}},{"Header":{"level":2,"text":"Second header"}},{"Table":{"headers":[{"element":{"Text":{"text":"Syntax","size":8}},"width":10.0},{"element":{"Text":{"text":"Description","size":8}},"width":10.0}],"rows":[{"cells":[{"element":{"Text":{"text":"Header","size":8}}},{"element":{"Text":{"text":"Title","size":8}}}]},{"cells":[{"element":{"Text":{"text":"Paragraph","size":8}}},{"element":{"Text":{"text":"Text","size":8}}}]}]}},{"Paragraph":{"elements":[{"Text":{"text":"Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla","size":8}},{"Text":{"text":"blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla","size":8}}]}}],"page_width":210.0,"page_height":297.0,"left_page_indent":10.0,"right_page_indent":10.0,"top_page_indent":10.0,"bottom_page_indent":10.0,"page_header":[],"page_footer":[]}"#;
        // println!("{:?}", document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        images.insert("test/data/picture.png".to_string(), image_bytes);
        let parsed = json::Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();

        println!("==========================");
        let generated_result = xml::Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);
        Ok(())
    }
}

use crate::core::{Document, Element, TransformerTrait};
use bytes::Bytes;
use serde::ser::SerializeStructVariant;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
pub struct Transformer;

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Element::Text { text, size } => {
                let mut state = serializer.serialize_struct_variant("Element", 0, "Text", 2)?;
                state.serialize_field("text", text)?;
                state.serialize_field("size", size)?;
                state.end()
            }
            Element::Header { level, text } => {
                let mut state = serializer.serialize_struct_variant("Element", 1, "Header", 2)?;
                state.serialize_field("level", level)?;
                state.serialize_field("text", text)?;
                state.end()
            }
            Element::Paragraph { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("Element", 2, "Paragraph", 1)?;
                state.serialize_field("elements", elements)?;
                state.end()
            }
            Element::Table { headers, rows } => {
                let mut state = serializer.serialize_struct_variant("Element", 3, "Table", 2)?;
                state.serialize_field("headers", headers)?;
                state.serialize_field("rows", rows)?;
                state.end()
            }
            Element::List { elements, numbered } => {
                let mut state = serializer.serialize_struct_variant("Element", 4, "List", 2)?;
                state.serialize_field("elements", elements)?;
                state.serialize_field("numbered", numbered)?;
                state.end()
            }
            Element::Image {
                bytes,
                title,
                alt,
                image_type,
            } => {
                let mut state = serializer.serialize_struct_variant("Element", 5, "Image", 4)?;
                state.serialize_field("bytes", bytes.iter().as_slice())?;
                state.serialize_field("title", title)?;
                state.serialize_field("alt", alt)?;
                state.serialize_field("image_type", image_type)?;
                state.end()
            }
            Element::Hyperlink { title, url, alt } => {
                let mut state =
                    serializer.serialize_struct_variant("Element", 6, "Hyperlink", 3)?;
                state.serialize_field("title", title)?;
                state.serialize_field("url", url)?;
                state.serialize_field("alt", alt)?;
                state.end()
            }
        }
    }
}

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        unimplemented!()
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let hm: HashMap<String, Bytes> = HashMap::new();
        let result = serde_json::to_string(document)?;
        Ok((Bytes::from(result), hm))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::markdown::*;
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"# First header

Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla

1. List item 1
2. List item 2
3. List item 3
   1. List item secode level 1
   2. List item secode level 2
4. List item 4
   1. List item secode level 3
   2. List item secode level 4
5. List item 5
   1. List item secode level 5

- List item one
  - List item two
- List item three
  - List item four
  - List item five
    - List item zzz
- List item six
  - List item seven

![Picture alt1](test/data/picture.png "Picture title1")


Bla bla bla ![Picture alt2](test/data/picture.png "Picture title2") bla. http://example.com  [Example](http://example.com) [Example](http://example.com "Example tooltip")


## Second header

| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |

Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
        // println!("{:?}", document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        images.insert("test/data/picture.png".to_string(), image_bytes);
        let parsed = Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = crate::json::Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);
        Ok(())
    }
}

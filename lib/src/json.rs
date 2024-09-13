use crate::core::{Document, Element, TransformerTrait};
use bytes::Bytes;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document> {
        let doc: Document = serde_json::from_slice(document.as_ref())?;
        Ok(doc)
    }
     fn generate(document: &Document) -> anyhow::Result<Bytes> {
            use serde_json::{Value, Map};
            // Helper function to serialize an Element into serde_json::Value
            fn serialize_element(element: &Element) -> Value {
                match element {
                    Element::Text { text, size } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Text".to_string()));
                        map.insert("text".to_string(), Value::String(text.clone()));
                        map.insert("size".to_string(), Value::Number((*size).into()));
                        Value::Object(map)
                    },
                    Element::Header { level, text } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Header".to_string()));
                        map.insert("level".to_string(), Value::Number((*level).into()));
                        map.insert("text".to_string(), Value::String(text.clone()));
                        Value::Object(map)
                    },
                    Element::Paragraph { elements } => {
                        let elements_json = elements.iter().map(serialize_element).collect();
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Paragraph".to_string()));
                        map.insert("elements".to_string(), Value::Array(elements_json));
                        Value::Object(map)
                    },
                    Element::Table { headers, rows } => {
                        let headers_json: Vec<Value> = headers.iter().map(|h| {
                            let mut h_map = Map::new();
                            h_map.insert("element".to_string(), serialize_element(&h.element));
                            h_map.insert("width".to_string(), Value::Number(serde_json::Number::from_f64(h.width as f64).unwrap()));
                            Value::Object(h_map)
                        }).collect();

                        let rows_json: Vec<Value> = rows.iter().map(|r| {
                            let cells_json: Vec<Value> = r.cells.iter().map(|c| serialize_element(&c.element)).collect();
                            let mut row_map = Map::new();
                            row_map.insert("cells".to_string(), Value::Array(cells_json));
                            Value::Object(row_map)
                        }).collect();

                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Table".to_string()));
                        map.insert("headers".to_string(), Value::Array(headers_json));
                        map.insert("rows".to_string(), Value::Array(rows_json));
                        Value::Object(map)
                    },
                    Element::List { elements, numbered } => {
                        let elements_json: Vec<Value> = elements.iter().map(|item| {
                            let mut item_map = Map::new();
                            item_map.insert("element".to_string(), serialize_element(&item.element));
                            Value::Object(item_map)
                        }).collect();

                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("List".to_string()));
                        map.insert("numbered".to_string(), Value::Bool(*numbered));
                        map.insert("elements".to_string(), Value::Array(elements_json));
                        Value::Object(map)
                    },
                    Element::Image(image_data) => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Image".to_string()));
                        // Encode image bytes to base64 for JSON representation
                        map.insert("bytes".to_string(), Value::String(base64::encode(&image_data.bytes())));
                        map.insert("title".to_string(), Value::String(image_data.title().to_string()));
                        map.insert("alt".to_string(), Value::String(image_data.alt().to_string()));
                        map.insert("image_type".to_string(), Value::String(format!("{:?}", image_data.image_type().to_string())));
                        map.insert("align".to_string(), Value::String(format!("{:?}", image_data.align())));

                        let mut size_map = Map::new();
                        if let Some(width) = &image_data.size().width {
                            size_map.insert("width".to_string(), Value::String(width.clone()));
                        }
                        if let Some(height) = &image_data.size().height {
                            size_map.insert("height".to_string(), Value::String(height.clone()));
                        }
                        map.insert("size".to_string(), Value::Object(size_map));
                        Value::Object(map)
                    },
                    Element::Hyperlink { title, url, alt, size } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Hyperlink".to_string()));
                        map.insert("title".to_string(), Value::String(title.clone()));
                        map.insert("url".to_string(), Value::String(url.clone()));
                        map.insert("alt".to_string(), Value::String(alt.clone()));
                        map.insert("size".to_string(), Value::Number((*size).into()));
                        Value::Object(map)
                    },
                }
            }

            // Serialize the main Document
            let mut doc_map = Map::new();

            // Serialize elements
            let elements_json: Vec<Value> = document.elements.iter().map(serialize_element).collect();
            doc_map.insert("elements".to_string(), Value::Array(elements_json));

            // Serialize page dimensions and indents
            doc_map.insert("page_width".to_string(), Value::Number(serde_json::Number::from_f64(document.page_width as f64).unwrap()));
            doc_map.insert("page_height".to_string(), Value::Number(serde_json::Number::from_f64(document.page_height as f64).unwrap()));
            doc_map.insert("left_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.left_page_indent as f64).unwrap()));
            doc_map.insert("right_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.right_page_indent as f64).unwrap()));
            doc_map.insert("top_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.top_page_indent as f64).unwrap()));
            doc_map.insert("bottom_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.bottom_page_indent as f64).unwrap()));

            // Serialize page headers
            let page_header_json: Vec<Value> = document.page_header.iter().map(serialize_element).collect();
            doc_map.insert("page_header".to_string(), Value::Array(page_header_json));

            // Serialize page footers
            let page_footer_json: Vec<Value> = document.page_footer.iter().map(serialize_element).collect();
            doc_map.insert("page_footer".to_string(), Value::Array(page_footer_json));

            // Create the final JSON value
            let doc_value = Value::Object(doc_map);

            // Serialize the JSON value to a string
            let json_string = doc_value.to_string();

            // Convert the string to Bytes and return
            Ok(Bytes::from(json_string))
        }

}

#[cfg(test)]
mod tests {
    use crate::core::{disk_image_loader, TransformerWithImageLoaderSaverTrait};
    use crate::json::{Transformer, TransformerTrait};

    #[test]
    fn test() -> anyhow::Result<()> {
        // let document = r#"{"elements":[{"Header":{"level":1,"text":"First header"}},{"Paragraph":{"elements":[{"Text":{"text":"Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla","size":8}},{"Text":{"text":"blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla","size":8}}]}},{"List":{"elements":[{"element":{"Text":{"text":"List item 1","size":8}}},{"element":{"Text":{"text":"List item 2","size":8}}},{"element":{"Text":{"text":"List item 3","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 1","size":8}}},{"element":{"Text":{"text":"List item secode level 2","size":8}}}],"numbered":true}}},{"element":{"Text":{"text":"List item 4","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 3","size":8}}},{"element":{"Text":{"text":"List item secode level 4","size":8}}}],"numbered":true}}},{"element":{"Text":{"text":"List item 5","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item secode level 5","size":8}}}],"numbered":true}}}],"numbered":true}},{"List":{"elements":[{"element":{"Text":{"text":"List item one","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item two","size":8}}}],"numbered":false}}},{"element":{"Text":{"text":"List item three","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item four","size":8}}},{"element":{"Text":{"text":"List item five","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item zzz","size":8}}}],"numbered":false}}}],"numbered":false}}},{"element":{"Text":{"text":"List item six","size":8}}},{"element":{"List":{"elements":[{"element":{"Text":{"text":"List item seven","size":8}}}],"numbered":false}}}],"numbered":false}},{"Paragraph":{"elements":[{"Image":{"bytes":[],"title":"Picture title1","alt":"Picture alt1","image_type":"Png"}}]}},{"Paragraph":{"elements":[{"Text":{"text":"Bla bla bla ","size":8}},{"Image":{"bytes":[],"title":"Picture title2","alt":"Picture alt2","image_type":"Png"}},{"Text":{"text":" bla. ","size":8}},{"Hyperlink":{"title":"http://example.com","url":"http://example.com","alt":"http://example.com","size":8}},{"Text":{"text":"  ","size":8}},{"Hyperlink":{"title":"Example","url":"http://example.com","alt":"Example","size":8}},{"Text":{"text":" ","size":8}},{"Hyperlink":{"title":"Example","url":"http://example.com","alt":"Example tooltip","size":8}}]}},{"Header":{"level":2,"text":"Second header"}},{"Table":{"headers":[{"element":{"Text":{"text":"Syntax","size":8}},"width":10.0},{"element":{"Text":{"text":"Description","size":8}},"width":10.0}],"rows":[{"cells":[{"element":{"Text":{"text":"Header","size":8}}},{"element":{"Text":{"text":"Title","size":8}}}]},{"cells":[{"element":{"Text":{"text":"Paragraph","size":8}}},{"element":{"Text":{"text":"Text","size":8}}}]}]}},{"Paragraph":{"elements":[{"Text":{"text":"Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla","size":8}},{"Text":{"text":"blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla","size":8}}]}}],"page_width":210.0,"page_height":297.0,"left_page_indent":10.0,"right_page_indent":10.0,"top_page_indent":10.0,"bottom_page_indent":10.0,"page_header":[],"page_footer":[]}"#;
        // let parsed = Transformer::parse(&document.as_bytes().into());
        // let document_string = std::str::from_utf8(document.as_bytes())?;
        // println!("{}", document_string);
        // assert!(parsed.is_ok());
        // let parsed_document = parsed.unwrap();
        let document = r#"
# First header

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

![Picture alt1](picture.png "Picture title1")

## Second header

| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |

Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
        // println!("{:?}", document);
        let parsed_r = crate::markdown::Transformer::parse_with_loader(&document.as_bytes().into(), disk_image_loader("test/data"));
        let parsed = parsed_r?;
        println!("==========================");
        println!("{:#?}", parsed);
        println!("==========================");
        let generated_result = crate::json::Transformer::generate(&parsed);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes)?;
        println!("{}", generated_text);

        Ok(())
    }
}

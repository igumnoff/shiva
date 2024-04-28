use crate::core::{Document, TransformerTrait};
use bytes::Bytes;
use std::collections::HashMap;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        println!("Parsing XML document...");
        let doc: Document = serde_xml::from_slice(document.as_ref())?;
        println!("XML document parsed successfully.");
        Ok(doc)
    }
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        println!("Generating XML document...");
        // Assuming document is serialized into bytes
        let serialized_document = serde_xml::to_string(document)?;
        let bytes = Bytes::from(serialized_document);

        // Placeholder for images HashMap
        let images: HashMap<String, Bytes> = HashMap::new();

        println!("XML document generated successfully.");
        Ok((bytes, images))
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::{Transformer, TransformerTrait};
    use std::collections::HashMap;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"<data><elements><Header level="1" text="First header"/><Paragraph><elements><Text text="Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla" size="8"/><Text text="blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla" size="8"/></elements></Paragraph><List numbered="true"><elements><element><Text text="List item 1" size="8"/></element><element><Text text="List item 2" size="8"/></element><element><Text text="List item 3" size="8"/></element><element><List numbered="true"><elements><element><Text text="List item secode level 1" size="8"/></element><element><Text text="List item secode level 2" size="8"/></element></elements></List></element><element><Text text="List item 4" size="8"/></element><element><List numbered="true"><elements><element><Text text="List item secode level 3" size="8"/></element><element><Text text="List item secode level 4" size="8"/></element></elements></List></element><element><Text text="List item 5" size="8"/></element><element><List numbered="true"><elements><element><Text text="List item secode level 5" size="8"/></element></elements></List></element></elements></List></elements></List><List numbered="false"><elements><element><Text text="List item one" size="8"/></element><element><List numbered="false"><elements><element><Text text="List item two" size="8"/></element></elements></List></element><element><Text text="List item three" size="8"/></element><element><List numbered="false"><elements><element><Text text="List item four" size="8"/></element><element><Text text="List item five" size="8"/></element><element><List numbered="false"><elements><element><Text text="List item zzz" size="8"/></element></elements></List></element></elements></List></element><element><Text text="List item six" size="8"/></element><element><List numbered="false"><elements><element><Text text="List item seven" size="8"/></element></elements></List></element></elements></List></elements></List><Paragraph><elements><Image bytes="" title="Picture title1" alt="Picture alt1" image_type="Png"/></elements></Paragraph><Paragraph><elements><Text text="Bla bla bla " size="8"/><Image bytes="" title="Picture title2" alt="Picture alt2" image_type="Png"/><Text text=" bla. " size="8"/><Hyperlink title="http://example.com" url="http://example.com" alt="http://example.com" size="8"/><Text text="  " size="8"/><Hyperlink title="Example" url="http://example.com" alt="Example" size="8"/><Text text=" " size="8"/><Hyperlink title="Example" url="http://example.com" alt="Example tooltip" size="8"/></elements></Paragraph><Header level="2" text="Second header"/><Table><headers><element><Text text="Syntax" size="8"/></element><element><Text text="Description" size="8"/></element></headers><rows><cells><element><Text text="Header" size="8"/></element><element><Text text="Title" size="8"/></element></cells><cells><element><Text text="Paragraph" size="8"/></element><element><Text text="Text" size="8"/></element></cells></rows></Table><Paragraph><elements><Text text="Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla" size="8"/><Text text="blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla" size="8"/></elements></Paragraph></elements><page_width>210.0</page_width><page_height>297.0</page_height><left_page_indent>10.0</left_page_indent><right_page_indent>10.0</right_page_indent><top_page_indent>10.0</top_page_indent><bottom_page_indent>10.0</bottom_page_indent><page_header/><page_footer/></data>"#;
        // println!("{:?}", document);
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/picture.png")?;
        images.insert("test/data/picture.png".to_string(), image_bytes);
        println!("Loaded images into HashMap.");
        println!("XML document string:");
        println!("{}", document);

        // Generate XML document
        println!("Generating XML document...");
        let generated_result = xml::Generator::generate(document.as_bytes())?;
        let generated_bytes = generated_result.0;
        let generated_text = std::str::from_utf8(&generated_bytes)?;
        println!("Generated XML document:");
        println!("{}", generated_text);
        Ok(())
    }
}

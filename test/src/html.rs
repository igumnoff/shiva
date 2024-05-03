#[cfg(test)]
mod tests {
    use shiva::core::Element::Header;
    use shiva::core::*;
    use shiva::html::Transformer;
    use std::collections::HashMap;

    #[test]
    fn test_html_header_parse() -> anyhow::Result<()> {
        let html_document = r#"
      <!DOCTYPE html>
      <html>
      <body>
      <h1>First header</h1>
      <h2>Second header</h2>
      <h3>Third header</h3>
      <h4>Fourth header</h4>
      <h5>Fifth header</h5>
      <h6>Sixth header</h6>
     </body>
      </html>
              "#;

        let parsed = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new())?;
        assert_eq!(parsed.elements.len(), 6);
        let elements = parsed.elements;
        match &elements[0] {
            Header { level: _, text } => {
                assert_eq!(text, "First header");
            }
            _ => panic!("Expected header"),
        }

        Ok(())
    }

    #[test]
    fn test_html_header_generate() -> anyhow::Result<()> {
        let html_document = r#"<!DOCTYPE html>
<html>
<body>
<h1>First header</h1>
<h2>Second header</h2>
<h3>Third header</h3>
<h4>Fourth header</h4>
<h5>Fifth header</h5>
<h6>Sixth header</h6>
</body>
</html>"#;

        let parsed: Document = Document {
            elements: [
                Header {
                    level: 1,
                    text: "First header".to_string(),
                },
                Header {
                    level: 2,
                    text: "Second header".to_string(),
                },
                Header {
                    level: 3,
                    text: "Third header".to_string(),
                },
                Header {
                    level: 4,
                    text: "Fourth header".to_string(),
                },
                Header {
                    level: 5,
                    text: "Fifth header".to_string(),
                },
                Header {
                    level: 6,
                    text: "Sixth header".to_string(),
                },
            ]
            .to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };

        let generated_result = Transformer::generate(&parsed);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_document);

        Ok(())
    }
}

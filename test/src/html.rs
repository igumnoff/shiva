#[cfg(test)]
mod tests {
    use shiva::core::*;
    use shiva::html::Transformer;
    use std::collections::HashMap;
    use shiva::core::Element::Header;

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

}

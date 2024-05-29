#[cfg(test)]
mod tests {
    use shiva::core::Element::{Header, Text, Hyperlink, Paragraph, List, Table, Image};
    use shiva::core::{Element, Document, TransformerTrait, TableHeader, TableRow, TableCell, ListItem, ImageType}; 
    use shiva::html::Transformer;
    use std::collections::HashMap;
    use bytes::Bytes;

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

        let parsed: Document = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new())?;
        assert_eq!(parsed.elements.len(), 6);
        let elements: Vec<Element> = parsed.elements;
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
        let html_str = r#"<!DOCTYPE html>
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

        let html_document: Document = Document {
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

        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);

        Ok(())
    }

    #[test]
    fn test_html_paragraph_parse() -> anyhow::Result<()> {
        let html_document: &str = r#"
<!DOCTYPE html>
<html>
<body>
<p>First Paragraph</p>
<p>Second Paragraph</p>
<p>Third Paragraph</p>
<p>Fourth Paragraph</p>
<p>Fifth Paragraph</p>
<p>Sixth Paragraph</p>
</body>
</html>
"#;
    
        let parsed: Document = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new())?;
        assert_eq!(parsed.elements.len(), 6);
        let elements: Vec<Element> = parsed.elements;
        match &elements[0] {
            Paragraph { elements } => {
                match &elements[0] {
                    Text { text, size: _ } => {
                        assert_eq!(text, "First Paragraph");
                    }
                    _ => panic!("Expected Paragraph"),
                }
            }
            _ => panic!("Expected Paragraph"),
        }

        Ok(())
    }
    #[test]
    fn test_html_paragraph_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>First Paragraph</p>
<p>Second Paragraph</p>
<p>Third Paragraph</p>
<p>Fourth Paragraph</p>
<p>Fifth Paragraph</p>
<p>Sixth Paragraph</p>
</body>
</html>"#;
        let html_document: Document = Document {
            elements: [
                Paragraph {
                    elements: [
                        Text {
                            text: "First Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
                Paragraph {
                    elements: [
                        Text {
                            text: "Second Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
                Paragraph {
                    elements: [
                        Text {
                            text: "Third Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
                Paragraph {
                    elements: [
                        Text {
                            text: "Fourth Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
                Paragraph {
                    elements: [
                        Text {
                            text: "Fifth Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
                Paragraph {
                    elements: [
                        Text {
                            text: "Sixth Paragraph".to_string(),
                            size: 8,
                        }
                    ].to_vec(),
                },
            ].to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };
        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);

        Ok(())
}

    #[test]
    fn test_html_list_parse() -> anyhow::Result<()> {
        let html_document: &str =  r#"<!DOCTYPE html>
<html>
<body>
<ol>
<li>List item 1</li>
<li>List item 2</li>
<li>List item 3</li>
<li>List item 4</li>
<li>List item 5</li>
</ol>
<ul>
<li>List item one</li>
<li>List item three</li>
<li>List item six</li>
</ul>
</body>
</html>"#;

        let parsed: Document = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new())?;
        let elements: Vec<Element> = parsed.elements;
        match &elements[0] {
            List { elements, numbered: _ } => {
                match &elements[0] {
                    ListItem { element } => {
                        match element {
                            Text { text, size: _} => {
                                assert_eq!(text, "List item 1");
                            }
                            _ => panic!("Expected Paragraph"),
                        }
                    }
                }
            }
            _ => panic!("Expected Paragraph")
        }
        

        Ok(())

    }

    #[test]
    fn test_html_list_generate() -> anyhow::Result<()> {
        let html_str: &str =  r#"<!DOCTYPE html>
<html>
<body>
<ol>
<li>List item 1</li>
</ol>
<ul>
<li>List item one</li>
</ul>
</body>
</html>"#;

        let html_document: Document = Document {
            elements: [
                List {
                    elements: vec! [
                        {ListItem{ element: {Text { size: 8, text: "List item 1".to_string()}}}},
                    ],
                    numbered: true
                },
                List {
                    elements: vec! [
                        {ListItem{ element: {Text { size: 8, text: "List item one".to_string()}}}},
                    ],
                    numbered: false
                },
            ].to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };
        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);

        Ok(())
    }

    #[test]
    fn test_html_table_parse() -> anyhow::Result<()> {
        let html_document: &str = r#"<!DOCTYPE html>
<html>
<body>
<table  border="1">
<tr>
<th>Syntax</th>
<th>Description</th>
</tr>
<tr>
<td>Header</td>
<td>Title</td>
</tr>
<tr>
<td>Paragraph</td>
<td>Text</td>
</tr>
</table>
</body>
</html>"#;
        
        let parsed: Document = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new())?;
        let elements: Vec<Element> = parsed.elements;
        match &elements[0] {
            Table { headers, rows  } => {
                match &headers[0] {
                    TableHeader { element, width: _ } => {
                        match element {
                            Text { text, size: _} => {
                                assert_eq!(text, "Syntax");
                            }
                            _ => panic!("Expected Paragraph"),
                        }
                    }
                }
                match &rows[0] {
                    TableRow { cells } => {
                        match &cells[0] {
                            TableCell {element} => {
                                match element {
                                    Text { text, size: _} => {
                                        assert_eq!(text, "Header");
                                    }
                                    _=> panic!("Expected Paragraph"),
                                }
                            }
                        }
                    }
                }
            }
            _ => panic!("Expected Paragraph")
        }
        Ok(())
    }
        
    #[test]
    fn test_html_table_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<table  border="1">
<tr>
<th>Syntax</th>
<th>Description</th>
</tr>
<tr>
<td>Header</td>
<td>Title</td>
</tr>
<tr>
<td>Paragraph</td>
<td>Text</td>
</tr>
</table>
</body>
</html>"#;

        let html_document: Document = Document {
            elements: [
                Table {
                    headers: vec! [
                        {TableHeader{ element: {Text { size: 8, text: "Syntax".to_string()}}, width: 10.0}},
                        {TableHeader{ element: {Text { size: 8, text: "Description".to_string()}}, width: 10.0}}
                    ],
                    rows:  vec! [
                        TableRow { 
                            cells:vec! [
                                {TableCell{ element: {Text { size: 8, text: "Header".to_string()}}}},
                                {TableCell{ element: {Text { size: 8, text: "Title".to_string()}}}}
                            ],
                        },
                        TableRow { 
                            cells:vec! [
                                {TableCell{ element: {Text { size: 8, text: "Paragraph".to_string()}}}},
                                {TableCell{ element: {Text { size: 8, text: "Text".to_string()}}}}
                            ],
                        }
                    ],
                },
            ].to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };
        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);

        Ok(())
    }

    #[test]
    fn test_html_image_parse() -> anyhow::Result<()> {
        let html_document: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>
<img src="test/data/image1.png" alt="Picture alt2" title="Picture title2" />
</p>
</body>
</html>"#;

        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());    assert!(parsed.is_ok());
        assert!(parsed.is_ok());
        let parsed: Document = parsed?;
        let elements: Vec<Element> = parsed.elements;
        match &elements[0] {
            Paragraph { elements  } => {
                match &elements[0] {
                    Image { title, alt: _ , image_type: _, bytes: _} => {
                        assert_eq!(title, "Picture title2");
                    }
                    _ => panic!("Expected Paragraph"),
                }
            }
            _ => panic!("Expected Paragraph")
        }
        Ok(())
    }
    
    #[test]
    fn test_html_image_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>bla<img src="image0.png" alt="Picture alt2" title="Picture title2" />bla bla</p>
</body>
</html>"#;

        let image_path = format!("image{}.png", 1);
        let image_bytes = std::fs::read(image_path).unwrap_or_default();

        let html_document: Document = Document {
            elements: [
                Paragraph {
                    elements: vec! [
                        {Text{ size: 8, text: "bla".to_string()}},
                        {Image {alt:"Picture alt2".to_string(),image_type: ImageType::Png, title:"Picture title2".to_string(), bytes: Bytes::from(image_bytes) }},
                        {Text{ size: 8, text: "bla bla".to_string()}}
                    ],
                },
            ].to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };
        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);
        
        Ok(())
    }

    #[test]
    fn test_html_hyperlink_parse() -> anyhow::Result<()> {
        let html_document: &str = r#"<!DOCTYPE html>
<html>
<body>
<a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a> <a href="http://example.com" title="Example tooltip">Example</a>
</body>
</html>"#;

        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());    assert!(parsed.is_ok());
        assert!(parsed.is_ok());
        let parsed: Document = parsed?;
        let elements: Vec<Element> = parsed.elements;

        match &elements[0] {
            Hyperlink { title, alt: _ , url, size: _} => {
                assert_eq!(title, "http://example.com");
                assert_eq!(url, "http://example.com");
            }

            _ => panic!("Expected Hyperlink")
        }
        Ok(())
    }

    #[test]
    fn test_html_hyperlink_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p><a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a><a href="http://example.com" title="Example tooltip">Example</a></p>
</body>
</html>"#;  

        let html_document: Document = Document {
            elements: [
                Paragraph {
                    elements: [
                        Hyperlink {
                            title: "http://example.com".to_string(),
                            url: "http://example.com".to_string(),
                            size: 8,
                            alt: "http://example.com".to_string(),
                        },
                        Text {
                            size: 8,
                            text: "  ".to_string(),
                        },
                        Hyperlink {
                            title: "Example".to_string(),
                            url: "http://example.com".to_string(),
                            size: 8,
                            alt: "Example".to_string(),
                        },
                        Hyperlink {
                            title: "Example".to_string(),
                            url: "http://example.com".to_string(),
                            size: 8,
                            alt: "Example tooltip".to_string(),
                        },
                    ].to_vec(),
                },
            ].to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
            page_header: [].to_vec(),
            page_footer: [].to_vec(),
        };
        let generated_result = Transformer::generate(&html_document);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, html_str);
        
        Ok(())
    }

    #[test]
    fn test_html_pageheader_parse() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>test</p>
</body>
</html>"#;
          
        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_str.as_bytes().into(), &HashMap::new());
        assert!(parsed.is_ok());
        let mut parsed: Document = parsed?;

        let mut header_elements: Vec<Element> = Vec::new();
        let header: Element = Text {
            size: 10,
            text: std::string::String::from("This is page header text"),
        };

        header_elements.push(header);
        parsed.page_header = header_elements.clone();

        Ok(())
    }
    
    #[test]
    fn test_html_pageheader_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>test</p>
</body>
</html>"#;
        let expected_html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>This is page header text</p>
<p>test</p>
</body>
</html>"#;
            
        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_str.as_bytes().into(), &HashMap::new());
        assert!(parsed.is_ok());
        let mut parsed: Document = parsed?;

        let mut header_elements: Vec<Element> = Vec::new();
        let header: Element = Text {
            size: 10,
            text: std::string::String::from("This is page header text"),
        };

        header_elements.push(header);
        parsed.page_header = header_elements.clone();

        let generated_result = Transformer::generate(&parsed);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, expected_html_str);
        
        Ok(())
    }

    #[test]
    fn test_html_pagefooter_parse() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>test</p>
</body>
</html>"#;
          
        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_str.as_bytes().into(), &HashMap::new());
        assert!(parsed.is_ok());
        let mut parsed: Document = parsed?;

        let mut footer_elements: Vec<Element> = Vec::new();
        let footer: Element = Text {
            size: 10,
            text: std::string::String::from("This is page footer text"),
        };

        footer_elements.push(footer);
        parsed.page_footer = footer_elements.clone();

        Ok(())
    }

    #[test]
    fn test_html_pagefooter_generate() -> anyhow::Result<()> {
        let html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>test</p>
</body>
</html>"#;
        let expected_html_str: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>test</p>
<p>This is page footer text</p>
</body>
</html>"#;
          
        let parsed: Result<Document, anyhow::Error> = Transformer::parse(&html_str.as_bytes().into(), &HashMap::new());
        assert!(parsed.is_ok());
        let mut parsed: Document = parsed?;

        let mut footer_elements: Vec<Element> = Vec::new();
        let footer: Element = Text {
            size: 10,
            text: std::string::String::from("This is page footer text"),
        };

        footer_elements.push(footer);
        parsed.page_footer = footer_elements.clone();

        let generated_result = Transformer::generate(&parsed);
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        assert_eq!(generated_text, expected_html_str);
        
        Ok(())
    }
}
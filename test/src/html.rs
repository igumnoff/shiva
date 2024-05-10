#[cfg(test)]
mod tests {
    use shiva::core::Element::{Header, Text, Hyperlink};
    use shiva::core::*;
    use shiva::html::Transformer;
    use shiva::markdown;
    use std::collections::HashMap;
    use serde_json;

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
    let test_html_document: &str = r#"<p>Paragraph1 bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1</p>

<p>Paragraph2 bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2</p>

<p>Paragraph3 bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3</p>"#;

    let expected_markdown_document: &str = r#"Paragraph1 bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1

Paragraph2 bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2

Paragraph3 bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3"#;
    
    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_paragraph_generate() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Paragraph1 bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1</p>
<p>Paragraph2 bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2</p>
<p>Paragraph3 bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3</p>"#;

    let expected_markdown_document: &str = r#"Paragraph1 bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1bla1 bla1 bla1 bla1

Paragraph2 bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2bla2 bla2 bla2 bla2

Paragraph3 bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3bla3 bla3 bla3 bla3"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n";
    let test_footer_string: &str = "\n<p>This is page footer text</p>\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document= html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}

#[test]
fn test_html_list_parse() -> anyhow::Result<()> {
let test_html_document: &str = r#"<ol>
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
</ul>"#;

let expected_markdown_document: &str = r#"1. List item 1
2. List item 2
3. List item 3
4. List item 4
5. List item 5

- List item one
- List item three
- List item six"#;
    
    let parsed_html = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let parsed_html: Document = parsed_html?;
    let parsed_markdown: Document = parsed_markdown?;

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_list_generate() -> anyhow::Result<()> {
let test_html_document: &str = r#"<ol>
<li>List item 1</li>
<li>List item 2</li>
<li>List item 3</li>
<ol>
<li>List item secode level 1</li>
<li>List item secode level 2</li>
</ol>
<li>List item 4</li>
<ol>
<li>List item secode level 3</li>
<li>List item secode level 4</li>
</ol>
<li>List item 5</li>
<ol>
<li>List item secode level 5</li>
</ol>
</ol>
<ul>
<li>List item one</li>
<ul>
<li>List item two</li>
</ul>
<li>List item three</li>
<ul>
<li>List item four</li>
<li>List item five</li>
<ul>
<li>List item zzz</li>
</ul>
</ul>
<li>List item six</li>
<ul>
<li>List item seven</li>
</ul>
</ul>"#;

let expected_markdown_document: &str = r#"1. List item 1
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
  - List item seven"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n";
    let test_footer_string: &str = "\n<p>This is page footer text</p>\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    // assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}

#[test]
fn test_html_table_parse() -> anyhow::Result<()> {
let test_html_document: &str = r#"<table  border="1">
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
</table>"#;

let expected_markdown_document: &str = r#"| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |"#;
    
    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_table_generate() -> anyhow::Result<()> {
let test_html_document: &str = r#"<table  border="1">
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
</table>"#;

let expected_markdown_document: &str = r#"| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n";
    let test_footer_string: &str = "\n<p>This is page footer text</p>\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    // assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}

#[test]
fn test_html_image_parse() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla bla <img src="test/data/image1.png" alt="Picture alt2" title="Picture title2" /> bla. </p>"#;

    let expected_markdown_document: &str = r#"Bla bla bla ![Picture alt2](test/data/image1.png "Picture title2") bla. "#;
    
    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    let mut parsed_markdown: Document = parsed_markdown?;

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();


    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_image_generate() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla bla <img src="test/data/image1.png" alt="Picture alt2" title="Picture title2" /> bla. </p>"#;

    let expected_markdown_document: &str = r#"Bla bla bla ![Picture alt2](test/data/image1.png "Picture title2") bla. "#;
    
    let expected_html_document: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n<p>Bla bla bla <img src=\"image0.png\" alt=\"Picture alt2\" title=\"Picture title2\" /> bla. </p>\n<p>This is page footer text</p>\n</body>\n</html>";

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    let mut parsed_markdown: Document = parsed_markdown?;

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}

#[test]
fn test_html_hyperlink_parse() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a> <a href="http://example.com" title="Example tooltip">Example</a>"#;

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());

    let parsed_html: Document = parsed_html?;
    let html_elements: Vec<Element> = parsed_html.elements;

    match &html_elements[0] {
        Hyperlink { title: _, url, alt: _, size: _} => {
            assert_eq!(url, "http://example.com");
        }
        _ => panic!("Expected header"),
    }

    Ok(())
}
#[test]
fn test_html_hyperlink_generate() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla <a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a> <a href="http://example.com" title="Example tooltip">Example</a></p>"#;

    let expected_markdown_document: &str = r#"Bla bla http://example.com  [Example](http://example.com) [Example](http://example.com "Example tooltip")"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n";
    let test_footer_string: &str = "\n<p>This is page footer text</p>\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_header = header_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    // assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}


#[test]
fn test_html_pageheader_parse() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla</p>"#;

    let expected_markdown_document: &str = r#"Bla bla"#;
    
    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_markdown.page_header = header_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_pageheader_generate() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla</p>"#;

    let expected_markdown_document: &str = r#"Bla bla"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n<p>This is page header text</p>\n";
    let test_footer_string: &str = "\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_header = header_elements.clone();
    parsed_markdown.page_header = header_elements.clone();

    // assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}

#[test]
fn test_html_pagefooter_parse() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla</p>"#;

    let expected_markdown_document: &str = r#"Bla bla"#;
    
    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);

    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    Ok(())
}
#[test]
fn test_html_pagefooter_generate() -> anyhow::Result<()> {
    let test_html_document: &str = r#"<p>Bla bla</p>"#;

    let expected_markdown_document: &str = r#"Bla bla"#;
    
    let test_header_string: &str = "<!DOCTYPE html>\n<html>\n<body>\n";
    let test_footer_string: &str = "\n<p>This is page footer text</p>\n</body>\n</html>";
    let expected_html_document: &str = &(test_header_string.to_owned() + test_html_document + test_footer_string);

    let parsed_html: Result<Document, anyhow::Error> = Transformer::parse(&test_html_document.as_bytes().into(), &HashMap::new());
    assert!(parsed_html.is_ok());
    let parsed_markdown: Result<Document, anyhow::Error> =   markdown::Transformer::parse(&expected_markdown_document.as_bytes().into(), &HashMap::new());

    let mut parsed_html: Document = parsed_html?;
    println!("{:?}", parsed_html);
    println!("=========================");
    let mut parsed_markdown: Document = parsed_markdown?;
    println!("{:?}", parsed_markdown);

    let mut footer_elements: Vec<Element> = Vec::new();
    let mut header_elements: Vec<Element> = Vec::new();

    let header: Element = Text {
        size: 10,
        text: std::string::String::from("This is page header text"),
    };
    let footer: Element = Text {
        size: 10,
        text: std::string::String::from("This is page footer text"),
    };

    footer_elements.push(footer);
    header_elements.push(header);
    parsed_html.page_footer = footer_elements.clone();
    parsed_markdown.page_footer = footer_elements.clone();

    // assert_eq!(serde_json::to_string(&parsed_html).unwrap() , serde_json::to_string(&parsed_markdown).unwrap());

    let html_generated_document = Transformer::generate(&parsed_markdown);

    assert!(html_generated_document.is_ok());

    let generated_document = html_generated_document?;

    let generated_document: &str = std::str::from_utf8(&generated_document.0)?;

    assert_eq!(generated_document, expected_html_document);

    Ok(())
}
}

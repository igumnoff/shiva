use crate::core::*;
use bytes::Bytes;
use ego_tree::{iter::Children, NodeRef};
use std::collections::HashMap;

use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use scraper::{Html, Node};

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let html = String::from_utf8(document.to_vec())?;
        let document = Html::parse_document(&html);
        let mut elements: Vec<Element> = Vec::new();

        parse_html(document.root_element().children(), &mut elements)?;
        Ok(Document::new(elements))
    }

    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let mut html = String::new();
        let mut images: HashMap<String, Bytes> = HashMap::new();
        let mut image_num: i32 = 0;

        html.push_str("<!DOCTYPE html>\n<html>\n<body>\n");

        for element in &document.elements {
            match element {
                Element::Header { level, text } => {
                    html.push_str(&format!("<h{}>{}</h{}>\n", level, text, level));
                }
                Paragraph { elements } => {
                    html.push_str("<p>");

                    for child in elements {
                        html.push_str(&generate_html_for_element(
                            child,
                            &mut images,
                            &mut image_num,
                        )?);
                    }

                    html.push_str("</p>\n");
                }
                List {
                    elements: _,
                    numbered: _,
                } => {
                    let list = generate_html_for_element(element, &mut images, &mut image_num)?;

                    html.push_str(&list);
                }
                Table { headers, rows } => {
                    let mut table_html = String::from("<table  border=\"1\">\n");

                    if !headers.is_empty() {
                        table_html.push_str("<tr>\n");

                        for header in headers {
                            let header_html = generate_html_for_element(
                                &header.element,
                                &mut images,
                                &mut image_num,
                            )?;

                            table_html.push_str(&format!("<th>{}</th>\n", header_html));
                        }

                        table_html.push_str("</tr>\n");
                    }
                    for row in rows {
                        table_html.push_str("<tr>\n");

                        for cell in &row.cells {
                            let cell_html = generate_html_for_element(
                                &cell.element,
                                &mut images,
                                &mut image_num,
                            )?;

                            table_html.push_str(&format!("<td>{}</td>\n", cell_html));
                        }

                        table_html.push_str("</tr>\n");
                    }

                    table_html.push_str("</table>\n");
                    html.push_str(&table_html)
                }

                _ => {}
            }
        }

        html.push_str("</body>\n</html>");
        Ok((Bytes::from(html), HashMap::new()))
    }
}

fn parse_html(children: Children<Node>, elements: &mut Vec<Element>) -> anyhow::Result<()> {
    for child in children {
        match child.value() {
            Node::Element(ref element) => match element.name() {
                "table" => {
                    let mut headers: Vec<TableHeader> = Vec::new();
                    let mut rows: Vec<TableRow> = Vec::new();
                    for table_child in child.children() {
                        for child in table_child.children() {
                            match child.value() {
                                Node::Element(ref table_element) => match table_element.name() {
                                    "tr" => {
                                        let mut cells: Vec<TableCell> = Vec::new();
                                        let mut is_header = false;
                                        for tr_child in child.children() {
                                            match tr_child.value() {
                                                Node::Element(ref tr_element) => {
                                                    match tr_element.name() {
                                                        "th" => {
                                                            is_header = true;
                                                            let mut header_elements: Vec<Element> =
                                                                Vec::new();
                                                            parse_html(
                                                                tr_child.children(),
                                                                &mut header_elements,
                                                            )?;
                                                            headers.extend(
                                                                header_elements.into_iter().map(
                                                                    |element| TableHeader {
                                                                        element,
                                                                        width: 10.0,
                                                                    },
                                                                ),
                                                            );
                                                        }
                                                        "td" => {
                                                            let mut cell_elements: Vec<Element> =
                                                                Vec::new();
                                                            parse_html(
                                                                tr_child.children(),
                                                                &mut cell_elements,
                                                            )?;
                                                            cells.extend(
                                                                cell_elements.into_iter().map(
                                                                    |element| TableCell { element },
                                                                ),
                                                            );
                                                        }
                                                        _ => { /*  */ }
                                                    }
                                                }
                                                _ => { /*  */ }
                                            }
                                        }
                                        if !is_header {
                                            rows.push(TableRow { cells });
                                        }
                                    }
                                    _ => { /*  */ }
                                },
                                _ => { /*  */ }
                            }
                        }
                    }
                    if !headers.is_empty() || !rows.is_empty() {
                        elements.push(Table { headers, rows });
                    }
                }
                "p" => {
                    let mut paragraph_elements: Vec<Element> = Vec::new();
                    parse_html(child.children(), &mut paragraph_elements)?;
                    elements.push(Paragraph {
                        elements: paragraph_elements,
                    });
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    let level = element.name().as_bytes()[1] - b'0';
                    // Retrieve the deepest text within any nested structure of the same header tag
                    let text = retrieve_deep_text(child, element.name()).trim().to_string();

                    if text.is_empty() {
                        continue;
                    }

                    elements.push(Header { text, level });
                }
                "img" => {
                    let src = element.attr("src").unwrap_or_default();
                    let title = element.attr("title").unwrap_or_default();
                    let alt = element.attr("alt").unwrap_or_default();
                    let image_bytes = std::fs::read(src).unwrap_or_default();
                    elements.push(Image {
                        bytes: Bytes::from(image_bytes),
                        title: title.to_string(),
                        alt: alt.to_string(),
                        image_type: ImageType::Png,
                    });
                }
                "ul" | "ol" => {
                    let mut list_items: Vec<ListItem> = Vec::new();
                    let numbered = element.name() == "ol";
                    for list_child in child.children() {
                        if let Node::Element(ref li_element) = list_child.value() {
                            if li_element.name() == "li" {
                                let mut item_elements: Vec<Element> = Vec::new();
                                parse_html(list_child.children(), &mut item_elements)?;
                                list_items.extend(
                                    item_elements
                                        .into_iter()
                                        .map(|element| ListItem { element }),
                                );
                            }
                        }
                    }
                    elements.push(List {
                        elements: list_items,
                        numbered,
                    });
                }
                "a" => {
                    let href = element.attr("href").unwrap_or_default().to_string();
                    let text = child
                        .children()
                        .filter_map(|n| {
                            if let Node::Text(ref text) = n.value() {
                                Some(text.clone().to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("");
                    elements.push(Hyperlink {
                        title: text,
                        url: href,
                        alt: "".to_owned(),
                        size: 8,
                    });
                }
                _ => {
                    parse_html(child.children(), elements)?;
                }
            },
            Node::Text(ref text) => {
                let text_str = text.to_string();
                if !text_str.trim().is_empty() {
                    elements.push(Text {
                        text: text_str,
                        size: 8,
                    });
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn generate_html_for_element(
    element: &Element,
    images: &mut HashMap<String, Bytes>,
    image_num: &mut i32,
) -> anyhow::Result<String> {
    match element {
        Text { text, size: _ } => Ok(text.to_string()),
        Paragraph { elements } => {
            let mut paragraph_html = String::from("<p>");
            for child in elements {
                paragraph_html
                    .push_str(generate_html_for_element(child, images, image_num)?.as_str());
            }
            paragraph_html.push_str("</p>");
            Ok(paragraph_html)
        }
        Header { level, text } => Ok(format!(
            "<h{level}>{text}</h{level}>",
            level = level,
            text = text
        )),
        List { elements, numbered } => {
            let tag = if *numbered { "ol" } else { "ul" };
            let mut list_html = format!("<{}>", tag);
            list_html.push('\n');
            for item in elements {
                let item_html = generate_html_for_element(&item.element, images, image_num)?;
                if let List { .. } = item.element {
                    list_html.push_str(&item_html.to_string());
                } else {
                    list_html.push_str(&format!("<li>{}</li>", item_html));
                    list_html.push('\n');
                }
            }
            list_html.push_str(&format!("</{}>", tag));
            list_html.push('\n');
            Ok(list_html)
        }
        Image {
            bytes,
            title,
            alt,
            image_type: _,
        } => {
            let image_path = format!("image{}.png", image_num);
            images.insert(image_path.to_string(), bytes.clone());
            *image_num += 1;
            Ok(format!(
                "<img src=\"{}\" alt=\"{}\" title=\"{}\" />",
                image_path, alt, title
            ))
        }
        Hyperlink {
            title, url, alt, ..
        } => Ok(format!(
            "<a href=\"{}\" title=\"{}\">{}</a>",
            url, alt, title
        )),
        _ => Ok("".to_string()),
    }
}

fn retrieve_deep_text(node: NodeRef<Node>, tag_name: &str) -> String {
    let mut text = String::new();
    let mut current_node = Some(node);
    while let Some(n) = current_node {
        if let Node::Element(ref el) = n.value() {
            if el.name() == tag_name {
                current_node = n.children().next(); // Move deeper if the same tag is nested
            } else {
                break; // Stop if a different tag is encountered
            }
        } else if let Node::Text(ref txt) = n.value() {
            text = txt.to_string(); // Set text if text node is found

            break;
        } else {
            break; // Break on encountering any other type of node
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::html::Transformer;
    use crate::markdown;
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
        let parsed = markdown::Transformer::parse(&document.as_bytes().into(), &HashMap::new());
        let document_string = std::str::from_utf8(document.as_bytes())?;
        println!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();
        println!("==========================");
        println!("{:?}", parsed_document);
        println!("==========================");
        let generated_result = Transformer::generate(&parsed_document);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);

        let html_document = r#"
<!DOCTYPE html>
<html>
<body>
<h1>First header</h1>
<p>Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla blablabla bla bla blabla bla bla blabla bla bla blabla bla bla bla</p>
<ol>
<li>List item 1</li>
<li>List item 2</li>
<li>List item 3</li>
<li><ol>
<li>List item secode level 1</li>
<li>List item secode level 2</li>
</ol>
</li>
<li>List item 4</li>
<li><ol>
<li>List item secode level 3</li>
<li>List item secode level 4</li>
</ol>
</li>
<li>List item 5</li>
<li><ol>
<li>List item secode level 5</li>
</ol>
</li>
</ol>
<ul>
<li>List item one</li>
<li><ul>
<li>List item two</li>
</ul>
</li>
<li>List item three</li>
<li><ul>
<li>List item four</li>
<li>List item five</li>
<li><ul>
<li>List item zzz</li>
</ul>
</li>
</ul>
</li>
<li>List item six</li>
<li><ul>
<li>List item seven</li>
</ul>
</li>
</ul>
<p><img src="test/data/image0.png" alt="Picture alt1" title="Picture title1" /></p>
<p>Bla bla bla <img src="test/data/image1.png" alt="Picture alt2" title="Picture title2" /> bla. <a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a> <a href="http://example.com" title="Example tooltip">Example</a></p>
<h2>Second header</h2>
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
<p>Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla blablabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla</p>
</body>
</html>
        "#;

        println!("==========================");
        let mut images = HashMap::new();
        let image_bytes = std::fs::read("test/data/image0.png")?;
        images.insert("image0.png".to_string(), image_bytes);
        let image_bytes = std::fs::read("test/data/image1.png")?;
        images.insert("image1.png".to_string(), image_bytes);
        let parsed = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());
        println!("{:?}", &parsed);
        println!("==========================");

        let generated_result = Transformer::generate(&parsed?);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);

        Ok(())
    }

    #[test]
    fn indent_test() -> anyhow::Result<()> {
        let html_document = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Document</title>\n</head>\n<body>\n    ABC\n    Lorem\n    Ipsum\n    Dolor\n    <h1> H1 </h1>\n    <h1> h1 </h1>\n    <h2> H2 </h2>\n    <h3> H3 </h3>\n    <h4> H4 </h4>\n    <h5> H5 </h5>\n    <h6> H6 </h6>\n    <h1>\n        TEST\n    </h1>\n    <h1>\n        <h1>TEST</h1>\n    </h1>\n    <h1>\n        <h1>TEST</h1>\n    </h1>\n    <h1>\n        <h1>\n            TEST\n        </h1>\n    </h1>\n    <h1>\n        <h1>\n            TEST\n        </h1>\n    </h1>\n    <div>\n        ABC\n        Lorem\n        Ipsum\n        Dolor\n        <h1> H1 </h1>\n        <h1> h1 </h1>\n        <h2> H2 </h2>\n        <h3> H3 </h3>\n        <h4> H4 </h4>\n        <h5> H5 </h5>\n        <h6> H6 </h6>\n        <h1>\n            TEST\n        </h1>\n        <h1>\n            <h1>TEST</h1>\n        </h1>\n        <h1>\n            <h1>TEST</h1>\n        </h1>\n        <h1>\n            <h1>\n                TEST\n            </h1>\n        </h1>\n        <h1>\n            <h1>\n                TEST\n            </h1>\n        </h1>\n    </div>\n    <h1> This is a test doc </h1>\n</body>\n</html>";
        let expected_md_document = "Document \n    ABC\n    Lorem\n    Ipsum\n    Dolor\n    # H1\n\n# h1\n\n## H2\n\n### H3\n\n#### H4\n\n##### H5\n\n###### H6\n\n# TEST\n\n# TEST\n\n# TEST\n\n# TEST\n\n# TEST\n\n\n        ABC\n        Lorem\n        Ipsum\n        Dolor\n        # H1\n\n# h1\n\n## H2\n\n### H3\n\n#### H4\n\n##### H5\n\n###### H6\n\n# TEST\n\n# TEST\n\n# TEST\n\n# TEST\n\n# TEST\n\n# This is a test doc\n\n";

        let parsed = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());

        assert!(parsed.is_ok());

        let parsed = parsed?;

        println!("==========================");
        println!("{:?}", parsed);
        println!("==========================");

        let generated_document = crate::markdown::Transformer::generate(&parsed);

        assert!(generated_document.is_ok());

        let generated_document = generated_document?;

        let generated_md_document = std::str::from_utf8(&generated_document.0)?;

        assert_eq!(generated_md_document, expected_md_document);

        Ok(())
    }
}

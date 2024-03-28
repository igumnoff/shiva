use std::collections::HashMap;
use bytes::Bytes;
use ego_tree::iter::Children;
use crate::core::*;

use scraper::{Html, Node};

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {
        let html = String::from_utf8(document.to_vec())?;
        let document = Html::parse_document(&html);
        let mut elements: Vec<Box<dyn Element>> = Vec::new();

        parse_html(document.root_element().children(), &mut elements)?;
        Ok(Document::new(&elements)?)
   }


    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        let mut html = String::new();
        let mut images:HashMap<String, Bytes> = HashMap::new();
        let mut image_num:i32 = 0;
        html.push_str("<!DOCTYPE html>\n<html>\n<body>\n");
        for element in &document.elements {
            match element.as_ref() {
                el if el.element_type() == ElementType::Header => {
                    let header = HeaderElement::as_ref(element)?;
                    html.push_str(&format!("<h{}>{}</h{}>\n", header.level, header.text, header.level));
                },
                el if el.element_type() == ElementType::Paragraph => {
                    let paragraph = ParagraphElement::as_ref(element)?;
                    html.push_str("<p>");
                    for child in &paragraph.elements {
                        html.push_str(&generate_html_for_element(child, &mut images, &mut image_num)?);
                    }
                    html.push_str("</p>\n");
                },
                el if el.element_type() == ElementType::List => {
                    let list = generate_html_for_element(element, &mut images, &mut image_num)?;
                    html.push_str(&list);
                },
                el if el.element_type() == ElementType::Table => {
                    let table = TableElement::from(element)?;
                    let mut table_html = String::from("<table  border=\"1\">");
                    table_html.push_str("\n");
                    if !table.headers.is_empty() {
                        table_html.push_str("<tr>");
                        table_html.push_str("\n");
                        for header in &table.headers {
                            let header_html = generate_html_for_element(&header.element, &mut images, &mut image_num)?;
                            table_html.push_str(&format!("<th>{}</th>", header_html));
                            table_html.push_str("\n");
                        }
                        table_html.push_str("</tr>");
                        table_html.push_str("\n");
                    }
                    for row in &table.rows {
                        table_html.push_str("<tr>");
                        table_html.push_str("\n");
                        for cell in &row.cells {
                            let cell_html = generate_html_for_element(&cell.element, &mut images, &mut image_num)?;
                            table_html.push_str(&format!("<td>{}</td>", cell_html));
                            table_html.push_str("\n");
                        }
                        table_html.push_str("</tr>");
                        table_html.push_str("\n");
                    }
                    table_html.push_str("</table>");
                    table_html.push_str("\n");
                    html.push_str(&table_html)
                },

                _ => {}
            }
        }

        html.push_str("</body>\n</html>");
        Ok((Bytes::from(html), HashMap::new()))
    }
}

fn parse_html(children: Children<Node>, elements: &mut Vec<Box<dyn Element>>) -> anyhow::Result<()> {
    for child in children {
        match child.value() {
            Node::Element(ref element) => {
                match element.name() {
                    "table" => {
                        let mut headers: Vec<TableHeaderElement> = Vec::new();
                        let mut rows: Vec<TableRowElement> = Vec::new();
                        for table_child in child.children() {
                            for child in table_child.children() {
                                match child.value() {
                                    Node::Element(ref table_element) => {
                                        // println!("{:?}", table_element.name());
                                        match table_element.name() {
                                            "tr" => {
                                                let mut cells: Vec<TableCellElement> = Vec::new();
                                                let mut is_header = false;
                                                for tr_child in child.children() {
                                                    match tr_child.value() {
                                                        Node::Element(ref tr_element) => {
                                                            match tr_element.name() {
                                                                "th" => {
                                                                    is_header = true;
                                                                    let mut header_elements: Vec<Box<dyn Element>> = Vec::new();
                                                                    parse_html(tr_child.children(), &mut header_elements)?;
                                                                    for header_element in header_elements {
                                                                        headers.push(TableHeaderElement::new(&header_element)?);
                                                                    }
                                                                },
                                                                "td" => {
                                                                    let mut cell_elements: Vec<Box<dyn Element>> = Vec::new();
                                                                    parse_html(tr_child.children(), &mut cell_elements)?;
                                                                    for cell_element in cell_elements {
                                                                        cells.push(TableCellElement::new(&cell_element)?);
                                                                    }
                                                                },
                                                                _ => {}
                                                            }
                                                        },
                                                        _ => {}
                                                    }
                                                }
                                                if is_header {
                                                    // Headers are processed above
                                                } else {
                                                    rows.push(TableRowElement::new(&cells)?);
                                                }
                                            },
                                            _ => {}
                                        }
                                    },
                                    _ => {}
                                }

                            }
                        }

                        // println!("{:?}", headers);
                        // println!("{:?}", rows);
                        if !headers.is_empty() || !rows.is_empty() {
                            elements.push(TableElement::new(&headers, &rows)?);
                        }
                    },
                    "p" => {
                        let mut paragraph_elements: Vec<Box<dyn Element>> = Vec::new();
                        parse_html(child.children(), &mut paragraph_elements)?;
                        if let Ok(paragraph_element) = ParagraphElement::new(&paragraph_elements) {
                            elements.push(paragraph_element);
                        }
                    },
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        let level = element.name()[1..].parse::<u8>().unwrap_or(1);
                        let text = child.children().filter_map(|n| {
                            if let Node::Text(ref text) = n.value() {
                                Some(text.clone().to_string())
                            } else {
                                None
                            }
                        }).collect::<Vec<String>>().join("");
                        if let Ok(header_element) = HeaderElement::new(&text, level) {
                            elements.push(header_element);
                        }
                    },
                    "img" => {
                        let _src = element.attr("src").unwrap_or_default();
                        let title = element.attr("title").unwrap_or_default();
                        let alt = element.attr("alt").unwrap_or_default();
                        // TODO fix debug for image bytes
                        let image_bytes = match std::fs::read("src") {
                            Ok(bytes) => bytes,
                            Err(_) => vec![],
                        };
                        if let Ok(image_element) = ImageElement::new(&Bytes::from(image_bytes), title, alt, ImageType::Png) {
                            elements.push(image_element);
                        }
                    },
                    "ul" | "ol" => {
                        let mut list_items: Vec<ListItemElement> = Vec::new();
                        let numbered = element.name() == "ol";
                        for list_child in child.children() {
                            if let Node::Element(ref li_element) = list_child.value() {
                                if li_element.name() == "li" {
                                    let mut item_elements: Vec<Box<dyn Element>> = Vec::new();
                                    parse_html(list_child.children(), &mut item_elements)?;
                                    for item_element in item_elements {
                                        if let Ok(list_item) = ListItemElement::new(&item_element) {
                                            list_items.push(list_item);
                                        }
                                    }
                                }
                            }
                        }
                        if let Ok(list_element) = ListElement::new(&list_items, numbered) {
                            elements.push(list_element);
                        }
                    },

                    "a" => {
                        let href = element.attr("href").unwrap_or_default().to_string();
                        let _title = element.attr("title").unwrap_or_default().to_string();
                        let alt = element.attr("alt").unwrap_or_default().to_string(); // 'alt' is not standard for 'a' tags but included for consistency
                        let text = child.children().filter_map(|n| {
                            if let Node::Text(ref text) = n.value() {
                                Some(text.clone().to_string())
                            } else {
                                None
                            }
                        }).collect::<Vec<String>>().join("");

                        if let Ok(hyperlink_element) = HyperlinkElement::new(&text, &href, &alt) {
                            elements.push(hyperlink_element);
                        }
                    },

                    // Add more tag handling as needed
                    _ => {
                        // For any unhandled tags, recursively parse their children without creating a new element
                        parse_html(child.children(), elements)?;
                    },
                }
            },
            Node::Text(ref text) => {
                if let Ok(text_element) = TextElement::new(text, 8) {
                    if TextElement::from(&text_element).unwrap().text != "\n"{
                        elements.push(text_element);
                    }
                }
            },
            _ => {} // Ignore other node types
        }
    }
    Ok(())
}

fn generate_html_for_element(element: &Box<dyn Element>, images: &mut HashMap<String, Bytes>, image_num: &mut i32) -> anyhow::Result<String> {
    match element.element_type() {
        ElementType::Text => {
            let text_element = TextElement::from(element)?;
            Ok(format!("{}", text_element.text))
        },
        ElementType::Paragraph => {
            let paragraph = ParagraphElement::as_ref(element)?;
            let mut paragraph_html = String::from("<p>");
            for child in &paragraph.elements {
                paragraph_html.push_str(&generate_html_for_element(child, images, image_num)?.as_str());
            }
            paragraph_html.push_str("</p>");
            Ok(paragraph_html)
        },
        ElementType::Header => {
            let header = HeaderElement::as_ref(element)?;
            Ok(format!("<h{level}>{text}</h{level}>", level = header.level, text = header.text))
        },
        ElementType::List => {
            let list = ListElement::from(element)?;
            let tag = if list.numbered { "ol" } else { "ul" };
            let mut list_html = format!("<{}>", tag);
            list_html.push_str("\n");
            for item in &list.elements {
                let item_html = generate_html_for_element(&item.element, images, image_num)?;
                if &item.element.element_type() == &ElementType::List {
                    list_html.push_str(&format!("{}", item_html));
                } else {
                    list_html.push_str(&format!("<li>{}</li>", item_html));
                    list_html.push_str("\n");
                }
            }
            list_html.push_str(&format!("</{}>", tag));
            list_html.push_str("\n");
            Ok(list_html)
        },
        ElementType::Image => {

            let image = ImageElement::from(element)?;
            let image_path = format!("image{}.png", image_num);
            images.insert(image_path.to_string(), image.bytes.clone());
            *image_num += 1;
            Ok(format!(
                "<img src=\"{}\" alt=\"{}\" title=\"{}\" />",
                image_path, image.alt, image.title
            ))
        },
        ElementType::Hyperlink => {
            let hyperlink = HyperlinkElement::from(element)?;
            Ok(format!(
                "<a href=\"{}\" title=\"{}\">{}</a>",
                hyperlink.url, hyperlink.alt, hyperlink.title
            ))
        },
        _ => {
            Ok("".to_string())
        },
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::core::*;
    use crate::{markdown};
    use crate::html::Transformer;

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
        let parsed =  markdown::Transformer::parse(&document.as_bytes().into(), &HashMap::new());
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
        let parsed =  Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());
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
}

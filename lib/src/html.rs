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

        let mut header_text = String::new();
        document.page_header.iter().for_each(|el| match el {
            Text { text, size: _ } => {
                header_text.push_str(text);
            }
            _ => {}
        });
        let mut footer_text = String::new();
        
        document.page_footer.iter().for_each(|el| match el {
            Text { text, size: _ } => {
                footer_text.push_str(text);
            }
            _ => {}
        });

        html.push_str("<!DOCTYPE html>\n<html>\n<body>\n");

        let all_elements: Vec<Element> = document
        .page_header
        .iter()
        .cloned()
        .chain(document.elements.iter().cloned())
        .chain(document.page_footer.iter().cloned())
        .collect();

        for element in &all_elements {
            match element {
                Element::Header { level, text } => {
                    html.push_str(&format!("<h{}>{}</h{}>\n", level, text, level));
                }
                Element::Text { text, size: _ } => {
                    html.push_str(&format!("<p>{}</p>\n", text));
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


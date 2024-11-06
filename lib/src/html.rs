use crate::core::*;
use bytes::Bytes;
use ego_tree::{iter::Children, NodeRef};

use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};
use scraper::{Html, Node};

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document> {
        Transformer::parse_with_loader(document, disk_image_loader("."))
    }

    fn generate(document: &Document) -> anyhow::Result<Bytes> {
        Transformer::generate_with_saver(document, disk_image_saver("."))
    }
}
impl TransformerWithImageLoaderSaverTrait for Transformer {
    fn parse_with_loader<F>(document: &Bytes, image_loader: F) -> anyhow::Result<Document>
    where
        F: Fn(&str) -> anyhow::Result<Bytes>,
    {
        let html = String::from_utf8(document.to_vec())?;
        let document = Html::parse_document(&html);
        let mut elements: Vec<Element> = Vec::new();

        let image_loader = ImageLoader {
            function: image_loader,
        };
        parse_html(
            document.root_element().children(),
            &mut elements,
            &image_loader,
        )?;
        Ok(Document::new(elements))
    }

    fn generate_with_saver<F>(document: &Document, image_saver: F) -> anyhow::Result<Bytes>
    where
        F: Fn(&Bytes, &str) -> anyhow::Result<()>,
    {
        let mut html = String::new();
        let mut image_num: i32 = 0;
        let image_saver = ImageSaver {
            function: image_saver,
        };

        //TODO: Is this needed? Commented out for now! header_text and footer_text are not read anywhere
        let mut header_text = String::new();
        document.get_page_header().iter().for_each(|el| match el {
            Text { text, size: _ } => {
                header_text.push_str(text);
            }
            _ => {}
        });
        let mut footer_text = String::new();

        document.get_page_footer().iter().for_each(|el| match el {
            Text { text, size: _ } => {
                footer_text.push_str(text);
            }
            _ => {}
        });

        html.push_str("<!DOCTYPE html>\n<html>\n<body>\n");

        let all_elements: Vec<&Element> = document.get_all_elements();

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
                            &mut image_num,
                            &image_saver,
                        )?);
                    }

                    html.push_str("</p>\n");
                }
                List {
                    elements: _,
                    numbered: _,
                } => {
                    let list = generate_html_for_element(element, &mut image_num, &image_saver)?;

                    html.push_str(&list);
                }
                Table { headers, rows } => {
                    let mut table_html = String::from("<table  border=\"1\">\n");

                    if !headers.is_empty() {
                        table_html.push_str("<tr>\n");

                        for header in headers {
                            let header_html = generate_html_for_element(
                                &header.element,
                                &mut image_num,
                                &image_saver,
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
                                &mut image_num,
                                &image_saver,
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

        Ok(Bytes::from(html))
    }
}

struct ImageLoader<F>
where
    F: Fn(&str) -> anyhow::Result<Bytes>,
{
    pub function: F,
}

struct ImageSaver<F>
where
    F: Fn(&Bytes, &str) -> anyhow::Result<()>,
{
    pub function: F,
}

fn parse_html<F>(
    children: Children<Node>,
    elements: &mut Vec<Element>,
    image_loader: &ImageLoader<F>,
) -> anyhow::Result<()>
where
    F: Fn(&str) -> anyhow::Result<Bytes>,
{
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
                                                                image_loader,
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
                                                                image_loader,
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
                "p" | "title" => {
                    let mut paragraph_elements: Vec<Element> = Vec::new();
                    parse_html(child.children(), &mut paragraph_elements, image_loader)?;
                    elements.push(Paragraph {
                        elements: paragraph_elements,
                    });
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    let level = element.name().as_bytes()[1] - b'0';
                    // Retrieve the deepest text within any nested structure of the same header tag
                    let mut text = retrieve_deep_text(child, element.name()).trim().to_string();

                    if text.is_empty() {
                        continue;
                    }

                    if text.contains("\n") {
                        text = text
                            .lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty()) // handles multiple consecutive newlines (\n\n)
                            .collect::<Vec<_>>()
                            .join(" ");
                    }

                    elements.push(Header { text, level });
                }
                "img" => {
                    let src = element.attr("src").unwrap_or_default();
                    let title = element.attr("title").unwrap_or_default();
                    let alt = element.attr("alt").unwrap_or_default();
                    let align = element.attr("align").unwrap_or_default();
                    let width = element.attr("width").and_then(|s| s.parse().ok());
                    let height = element.attr("height").and_then(|s| s.parse().ok());
                    let image_bytes = (image_loader.function)(src)?;
                    elements.push(Image(ImageData::new(
                        image_bytes,
                        title.to_string(),
                        alt.to_string(),
                        src.to_string(),
                        align.to_string(),
                        ImageDimension { width, height },
                    )));
                }
                "ul" | "ol" => {
                    let mut list_items: Vec<ListItem> = Vec::new();
                    let numbered = element.name() == "ol";
                    for list_child in child.children() {
                        if let Node::Element(ref li_element) = list_child.value() {
                            if li_element.name() == "li" {
                                let mut item_elements: Vec<Element> = Vec::new();
                                parse_html(
                                    list_child.children(),
                                    &mut item_elements,
                                    image_loader,
                                )?;
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
                    parse_html(child.children(), elements, image_loader)?;
                }
            },
            Node::Text(ref text) => {
                let txt_strings = text.lines().map(str::trim).filter(|p| !p.is_empty());
                for text_str in txt_strings {
                    elements.push(Text {
                        text: text_str.to_owned(),
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
    image_num: &mut i32,
    image_saver: &ImageSaver<impl Fn(&Bytes, &str) -> anyhow::Result<()>>,
) -> anyhow::Result<String> {
    match element {
        Text { text, size: _ } => Ok(text.to_string()),
        Paragraph { elements } => {
            let mut paragraph_html = String::from("<p>");
            for child in elements {
                paragraph_html
                    .push_str(generate_html_for_element(child, image_num, image_saver)?.as_str());
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
                let item_html = generate_html_for_element(&item.element, image_num, image_saver)?;
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
        Image(image) => {
            let image_path = format!("image{}.png", image_num);
            // images.insert(image_path.to_string(), bytes.clone());
            (image_saver.function)(image.bytes(), &image_path)?;
            *image_num += 1;

            let align_str = match image.align() {
                ImageAlignment::None => String::new(),
                _ => format!(" align=\"{}\"", image.align()),
            };

            let width_str = match &image.size().width {
                Some(width) => format!(" width=\"{}\"", width),
                None => String::new(),
            };

            let height_str = match &image.size().height {
                Some(height) => format!(" height=\"{}\"", height),
                None => String::new(),
            };
            Ok(format!(
                "<img src=\"{}\" alt=\"{}\" title=\"{}\"{}{}{} />",
                image_path,
                image.alt(),
                image.title(),
                align_str,
                width_str,
                height_str
            ))
        }
        Hyperlink {
            title, url, alt, ..
        } => {
            Ok(format!(
            "<a href=\"{}\" title=\"{}\">{}</a>",
            url, alt, title
        ))},
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
    use log::info;
    use crate::core::*;
    use crate::html::*;
    use crate::markdown;
    use crate::core::tests::init_logger;
    
    #[test]
    fn test_image_loader_saver() -> anyhow::Result<()> {
        init_logger();
        let document_html = r#"
        <html>
        <body>
        <p>123<img alt="smal image" align="left" src="small.png" width="100" height="99" title="image"></p>
        </body>
        </html>
        "#;
        let document = Transformer::parse_with_loader(
            &Bytes::from(document_html),
            disk_image_loader("test/data"),
        )?;
        info!("{:#?}", document);
        let result = Transformer::generate_with_saver(&document, disk_image_saver("test/data"))?;
        info!("{}", String::from_utf8(result.to_vec())?);
        Ok(())
    }

    #[test]
    fn test_parse_html() -> anyhow::Result<()> {
        init_logger();
        let document_html = r#"

            <html>
              <head>
                <title>Chew dad's slippers - this is a title element</title>
              </head>
              <body>
                <h1>
                  Instead of drinking water from the cat bowl, make sure to steal water from
                  the toilet
                </h1>
                <h2>Chase the red dot</h2>
                <p>
                  Munch, munch, chomp, chomp hate dogs. Spill litter box, scratch at owner,
                  destroy all furniture, especially couch get scared by sudden appearance of
                  cucumber cat is love, cat is life fat baby cat best buddy little guy for
                  catch eat throw up catch eat throw up bad birds jump on fridge. Purr like
                  a car engine oh yes, there is my human woman she does best pats ever that
                  all i like about her hiss meow .
                </p>
                <p>
                  Dead stare with ears cocked when owners are asleep, cry for no apparent
                  reason meow all night. Plop down in the middle where everybody walks favor
                  packaging over toy. Sit on the laptop kitty pounce, trip, faceplant.
                </p>

                <p>
                    <img src="small.png" alt="Picture alt2" title="Picture title2" />
                </p>

              </body>
            </html>
        "#;
        let document = Transformer::parse_with_loader(
            &Bytes::from(document_html),
            disk_image_loader("test/data"),
        )?;
        info!("{:#?}", document);
        let markdown =
            markdown::Transformer::generate_with_saver(&document, disk_image_saver("test/data"))?;
        info!("{}", String::from_utf8(markdown.to_vec())?);
        Ok(())
    }
}

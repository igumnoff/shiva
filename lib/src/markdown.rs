use crate::core::Element::{Header, Hyperlink, List, Table, Text};
use crate::core::*;
use bytes::Bytes;
use comrak::arena_tree::Node;
use comrak::Arena;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, TextMergeStream};
use std::cell::RefCell;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document> {
        Transformer::parse_with_loader(document, disk_image_loader("."))
    }

    fn generate(document: &Document) -> anyhow::Result<Bytes> {
        Transformer::generate_with_saver(document, disk_image_saver("."))
    }
}

struct ImageSaver<F>
where
    F: Fn(&Bytes, &str) -> anyhow::Result<()>,
{
    pub function: F,
}
impl TransformerWithImageLoaderSaverTrait for Transformer {
    fn parse_with_loader<F>(document: &Bytes, image_loader: F) -> anyhow::Result<Document>
    where
        F: Fn(&str) -> anyhow::Result<Bytes>,
        Self: Sized,
    {
        fn create_element_list(children: Option<Vec<ListItem>>, numbered: bool) -> Element {
            Element::List {
                elements: children.unwrap_or(vec![]),
                numbered,
            }
        }

        fn process_element_creation(
            current_element: &mut Option<Element>,
            mut new_el: Element,
            list_depth: &mut i32,
        ) {
            match current_element.as_mut() {
                Some(element) => match element {
                    Element::List { elements, numbered } => {
                        let mut list_elements = elements;

                        for _ in 1..*list_depth {
                            let last_index = list_elements.len() - 1;
                            if let Element::List {
                                elements: ref mut inner_els,
                                ..
                            } = list_elements[last_index].element
                            {
                                list_elements = inner_els;
                            } else {
                                panic!("Expected a nested list structure at the specified depth");
                            }
                        }
                        match &new_el {
                            Element::Hyperlink { .. } | Element::Header { .. } => {
                                if let Some(ListItem { element }) = list_elements.last() {
                                    if let Text { .. } = element {
                                        list_elements.pop();
                                    }
                                }
                            }

                            _ => {}
                        }

                        if matches!(new_el, Element::List { .. }) {
                            let list_item_children = ListItem {
                                element: create_element_list(None, *numbered),
                            };

                            if let Element::List {
                                ref mut elements, ..
                            } = new_el
                            {
                                let list_item_el = list_elements
                                    .pop()
                                    .expect("should have a list item as last element");
                                elements.push(list_item_el);
                                elements.push(list_item_children);
                                *list_depth += 1;
                            }
                        }

                        let li = ListItem { element: new_el };
                        list_elements.push(li);
                    }
                    _ => {}
                },
                None => {
                    *current_element = Some(new_el);
                }
            }
        }

        let document_str = std::str::from_utf8(document)?;
        let mut doc_elements: Vec<Element> = Vec::new();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        options.insert(Options::ENABLE_MATH);
        options.insert(Options::ENABLE_GFM);

        let parser = Parser::new_ext(document_str, options);
        let md_iterator = TextMergeStream::new(parser);

        let mut current_element: Option<Element> = None;
        let mut list_depth = 0;
        let mut table_element: Option<(bool, Element)> = None;
        for event in md_iterator {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {
                            if !matches!(current_element, Some(Element::List { .. })) {
                                process_element_creation(
                                    &mut current_element,
                                    Element::Paragraph { elements: vec![] },
                                    &mut list_depth,
                                );
                            }
                        }
                        Tag::Heading { level, .. } => {
                            let level = match level {
                                HeadingLevel::H1 => 1,
                                HeadingLevel::H2 => 2,
                                HeadingLevel::H3 => 3,
                                HeadingLevel::H4 => 4,
                                HeadingLevel::H5 => 5,
                                HeadingLevel::H6 => 6,
                            };
                            process_element_creation(
                                &mut current_element,
                                Element::Header {
                                    level,
                                    text: "".to_string(),
                                },
                                &mut list_depth,
                            );
                        }
                        Tag::List(numbered) => {
                            let numbered = numbered.is_some();

                            let list_el = List {
                                elements: vec![],
                                numbered,
                            };

                            process_element_creation(
                                &mut current_element,
                                list_el,
                                &mut list_depth,
                            );

                            list_depth += 1;
                        }
                        Tag::Item => {
                            let list_li = Text {
                                text: "".to_string(),
                                size: 14,
                            };

                            process_element_creation(
                                &mut current_element,
                                list_li,
                                &mut list_depth,
                            );
                        }
                        Tag::Table(_) => {
                            let table_el = Table {
                                headers: vec![],
                                rows: vec![],
                            };

                            table_element = Some((false, table_el));
                        }
                        Tag::TableHead => {
                            if let Some(table) = table_element.as_mut() {
                                table.0 = true;
                            }
                        }
                        Tag::Image {
                            dest_url, title, ..
                        } => {
                            let img_type = dest_url.to_string();
                            let bytes = image_loader(&dest_url)?;
                            let img_el = Element::Image(ImageData::new(
                                bytes,
                                title.to_string(),
                                title.to_string(),
                                img_type,
                                "".to_string(),
                                ImageDimension::default(),
                            ));
                            // Before image there is paragraph tag (likely because alt text is in paragraph )
                            current_element = None;
                            process_element_creation(&mut current_element, img_el, &mut list_depth);
                        }
                        Tag::Link {
                            dest_url, title, ..
                        } => {
                            let link_element = Hyperlink {
                                title: title.to_string(),
                                url: dest_url.to_string(),
                                alt: "alt".to_string(),
                                size: 14,
                            };
                            process_element_creation(
                                &mut current_element,
                                link_element,
                                &mut list_depth,
                            );
                        }

                        _rest => {
                            // warn!("The tag parsing is not implemented {:#?}", rest);
                        }
                    }
                }
                Event::Text(text) => {
                    if let Some(curr_el) = current_element.as_mut() {
                        match curr_el {
                            Element::Paragraph { ref mut elements } => {
                                elements.push(Element::Text {
                                    text: text.to_string(),
                                    size: 14,
                                })
                            }
                            Element::Header { text: el_text, .. } => {
                                el_text.push_str(&text);
                            }
                            Element::List { elements, .. } => {
                                let mut li_vec_to_insert = elements;
                                for _ in 1..list_depth {
                                    let last_index = li_vec_to_insert.len() - 1;
                                    if let Element::List {
                                        elements: ref mut inner_els,
                                        ..
                                    } = li_vec_to_insert[last_index].element
                                    {
                                        li_vec_to_insert = inner_els;
                                    } else {
                                        panic!("Expected a nested list structure at the specified depth");
                                    }
                                }

                                let li = li_vec_to_insert.last_mut().unwrap();

                                match &mut li.element {
                                    Text {
                                        text: element_text, ..
                                    } => {
                                        element_text.push_str(&text);
                                    }
                                    Hyperlink { title, .. } => {
                                        *title = text.to_string();
                                    }
                                    Header {
                                        text: header_text, ..
                                    } => {
                                        *header_text = text.to_string();
                                    }
                                    _ => {}
                                }
                            }
                            Element::Image(image) => image.set_image_alt(&text),
                            Element::Hyperlink { alt, .. } => {
                                *alt = alt.to_string();
                            }
                            _ => {}
                        }
                    }
                    match table_element {
                        Some(ref mut t_el) => {
                            if let (is_header, Element::Table { headers, rows }) = t_el {
                                if *is_header {
                                    headers.push(TableHeader {
                                        element: Text {
                                            text: text.to_string(),
                                            size: 14,
                                        },
                                        width: 30.,
                                    })
                                } else {
                                    let last_row = rows.last_mut();

                                    match last_row {
                                        Some(tr) => {
                                            if tr.cells.len() == headers.len() {
                                                rows.push(TableRow {
                                                    cells: vec![TableCell {
                                                        element: Text {
                                                            text: text.to_string(),
                                                            size: 14,
                                                        },
                                                    }],
                                                });
                                            } else {
                                                tr.cells.push(TableCell {
                                                    element: Text {
                                                        text: text.to_string(),
                                                        size: 14,
                                                    },
                                                });
                                            }
                                        }
                                        None => {
                                            rows.push(TableRow {
                                                cells: vec![TableCell {
                                                    element: Text {
                                                        text: text.to_string(),
                                                        size: 14,
                                                    },
                                                }],
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                Event::End(tag) => match tag {
                    TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::Link | TagEnd::Image => {
                        if !matches!(current_element, Some(Element::List { .. })) {
                            let curr_el = current_element.take();
                            if let Some(curr_el) = curr_el {
                                match curr_el {
                                    List { .. } => current_element = Some(curr_el),
                                    _ => {
                                        doc_elements.push(curr_el);
                                    }
                                }
                            }
                        }
                    }
                    TagEnd::List(_) => {
                        list_depth -= 2;
                        if list_depth <= 0 {
                            list_depth = 0;
                            let curr_el = current_element.take();
                            if let Some(curr_el) = curr_el {
                                doc_elements.push(curr_el);
                            }
                        }
                    }
                    TagEnd::TableHead => {
                        if let Some((is_header, _t_el)) = &mut table_element {
                            *is_header = false;
                        }
                    }
                    TagEnd::Table => {
                        if let Some((_, t_el)) = table_element.take() {
                            doc_elements.push(t_el);
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }

        Ok(Document::new(doc_elements))
    }

    fn generate_with_saver<F>(document: &Document, image_saver: F) -> anyhow::Result<Bytes>
    where
        F: Fn(&Bytes, &str) -> anyhow::Result<()>,
    {
        use comrak::nodes::LineColumn;
        use comrak::{format_commonmark, Arena, Options};
        use std::cell::RefCell;

        let arena = Arena::new();

        let root = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            LineColumn { line: 0, column: 0 },
        ))));

        let image_num = RefCell::new(0);

        let image_saver = ImageSaver {
            function: &image_saver,
        };

        let all_elements: Vec<&Element> = document.get_all_elements();

        for element in all_elements {
            let node = element_to_ast_node(&arena, element, &image_num, &image_saver)?;
            root.append(node);
        }

        let mut md = vec![];

        format_commonmark(root, &Options::default(), &mut md)?;

        Ok(Bytes::from(md))
    }
}

use comrak::nodes::{
    Ast, AstNode, LineColumn, NodeHeading, NodeLink, NodeList, NodeTable, NodeValue, TableAlignment,
};

fn is_parent_list(list_item: &ListItem) -> bool {
    if let Element::List { elements, .. } = &list_item.element {
        let first = elements.first();
        let second = elements.get(1);

        if let (Some(first), Some(last)) = (first, second) {
            let is_text = matches!(first.element, Element::Text { .. });
            let is_list = matches!(last.element, Element::List { .. });

            return is_text && is_list;
        }
    }

    false
}

fn create_item_node<'a>(arena: &'a Arena<AstNode<'a>>, numbered: bool) -> &'a AstNode<'a> {
    let item_node = arena.alloc(Node::new(RefCell::new(Ast::new(
        NodeValue::Item(NodeList {
            list_type: if numbered {
                comrak::nodes::ListType::Ordered
            } else {
                comrak::nodes::ListType::Bullet
            },
            start: 0,
            tight: true,
            ..Default::default()
        }),
        LineColumn { line: 0, column: 0 },
    ))));

    item_node
}

fn create_list_node<'a>(arena: &'a Arena<AstNode<'a>>, numbered: bool) -> &'a AstNode<'a> {
    let node = arena.alloc(Node::new(RefCell::new(Ast::new(
        NodeValue::List(NodeList {
            list_type: if numbered {
                comrak::nodes::ListType::Ordered
            } else {
                comrak::nodes::ListType::Bullet
            },
            start: 1,
            tight: true,
            bullet_char: b'-',
            marker_offset: 0,
            delimiter: comrak::nodes::ListDelimType::Period,
            ..Default::default()
        }), // Empty list item
        LineColumn { line: 0, column: 0 },
    ))));

    node
}

fn text_to_paragraph(element: Element) -> Element {
    if let Element::Text { text, .. } = element {
        Element::Paragraph {
            elements: vec![Element::Text {
                text: text.to_string(),
                size: 14,
            }],
        }
    } else {
        element
    }
}

fn element_to_ast_node<'a, F>(
    arena: &'a Arena<AstNode<'a>>,
    element: &Element,
    image_num: &RefCell<i32>,
    image_saver: &ImageSaver<F>,
) -> anyhow::Result<&'a AstNode<'a>>
where
    F: Fn(&Bytes, &str) -> anyhow::Result<()>,
{
    match element {
        Element::Text { text, .. } => {
            let node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Text(text.to_string()),
                LineColumn { line: 0, column: 0 },
            ))));

            Ok(node)
        }

        Element::Header { level, text } => {
            let heading = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Heading(NodeHeading {
                    level: *level as u8,
                    setext: false,
                }),
                LineColumn { line: 0, column: 0 },
            ))));
            let text_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Text(text.clone()),
                LineColumn { line: 0, column: 0 },
            ))));
            heading.append(text_node);
            Ok(heading)
        }

        Element::Paragraph { elements } => {
            let paragraph = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Paragraph,
                LineColumn { line: 0, column: 0 },
            ))));

            for child_element in elements {
                let child_node = element_to_ast_node(arena, child_element, image_num, image_saver)?;
                paragraph.append(child_node);
            }
            Ok(paragraph)
        }

        Element::List { elements, numbered } => {
            let list_node = create_list_node(arena, *numbered);
            for list_item in elements {
                let item_node = create_item_node(arena, *numbered);

                if is_parent_list(list_item) {
                    if let Element::List { elements, .. } = &list_item.element {
                        let first = elements.first();
                        let second = elements.get(1);

                        if let (Some(parent), Some(children)) = (first, second) {
                            let children_node = element_to_ast_node(
                                arena,
                                &children.element,
                                image_num,
                                image_saver,
                            )?;

                            let parent_element = text_to_paragraph(parent.element.clone());

                            let list_item_content = element_to_ast_node(
                                arena,
                                &parent_element,
                                image_num,
                                image_saver,
                            )?;

                            item_node.append(list_item_content);
                            item_node.append(children_node);

                            list_node.append(item_node);
                        }
                    }
                } else {
                    let list_item_element = text_to_paragraph(list_item.element.clone());

                    let list_item_content =
                        element_to_ast_node(arena, &list_item_element, image_num, image_saver)?;
                    item_node.append(list_item_content);
                    list_node.append(item_node);
                }
            }
            Ok(list_node)
        }

        Element::Image(image_data) => {
            *image_num.borrow_mut() += 1;
            let image_extension = image_data.image_type().to_extension();
            let image_filename = format!("image{}{}", image_num.borrow(), image_extension);

            (image_saver.function)(image_data.bytes(), &image_filename)?;

            let image_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Image(NodeLink {
                    url: image_filename.clone(),
                    title: image_data.title().to_string(),
                }),
                LineColumn { line: 0, column: 0 },
            ))));

            Ok(image_node)
        }

        Element::Hyperlink {
            title, url, alt, ..
        } => {
            let link_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Link(NodeLink {
                    url: url.clone(),
                    title: alt.clone(),
                }),
                LineColumn { line: 0, column: 0 },
            ))));
            let text_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Text(title.clone()),
                LineColumn { line: 0, column: 0 },
            ))));
            link_node.append(text_node);
            Ok(link_node)
        }

        Element::Table { headers, rows } => {
            let num_columns = headers.len() as u32;
            let num_rows = rows.len() as u32 + 1;

            let alignments = vec![TableAlignment::None; num_columns as usize];

            let table_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Table(NodeTable {
                    alignments,
                    num_columns: num_columns as usize,
                    num_rows: num_rows as usize,
                    num_nonempty_cells: 0, // Adjust as needed
                }),
                LineColumn { line: 0, column: 0 },
            ))));

            // Header row
            let header_row_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::TableRow(true), // Indicate header row
                LineColumn { line: 0, column: 0 },
            ))));
            for header in headers {
                let cell_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::TableCell,
                    LineColumn { line: 0, column: 0 },
                ))));
                let cell_content =
                    element_to_ast_node(arena, &header.element, image_num, image_saver)?;
                cell_node.append(cell_content);
                header_row_node.append(cell_node);
            }
            table_node.append(header_row_node);

            // Data rows
            for row in rows {
                let row_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::TableRow(false), // Indicate data row
                    LineColumn { line: 0, column: 0 },
                ))));
                for cell in &row.cells {
                    let cell_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                        NodeValue::TableCell,
                        LineColumn { line: 0, column: 0 },
                    ))));
                    let cell_content =
                        element_to_ast_node(arena, &cell.element, image_num, image_saver)?;
                    cell_node.append(cell_content);
                    row_node.append(cell_node);
                }
                table_node.append(row_node);
            }

            Ok(table_node)
        }

        _ => {
            let node = arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Text("".to_string()),
                LineColumn { line: 0, column: 0 },
            ))));
            Ok(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::str;

    use log::info;
    use serde_xml_rs::to_string;

    use crate::core::*;
    use crate::html;
    use crate::markdown::*;

    #[test]
    fn test() -> anyhow::Result<()> {
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

bla

Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
        let parsed = Transformer::parse_with_loader(
            &document.as_bytes().into(),
            disk_image_loader("test/data"),
        );
        let document_string = std::str::from_utf8(document.as_bytes())?;
        info!("{}", document_string);
        assert!(parsed.is_ok());
        let parsed_document = parsed.unwrap();

        info!("==========================");
        info!("{:#?}", parsed_document);
        info!("==========================");
        let generated_result =
            Transformer::generate_with_saver(&parsed_document, disk_image_saver("test/data"));
        assert!(generated_result.is_ok());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes)?;
        info!("{}", generated_text);
        Ok(())
    }

    #[test]
    fn test_parse_header() {
        let document = r#"
# First header

## Second Header

### Third Header
            "#;

        let elements = vec![
            Header {
                level: 1,
                text: "First header".to_string(),
            },
            Header {
                level: 2,
                text: "Second Header".to_string(),
            },
            Header {
                level: 3,
                text: "Third Header".to_string(),
            },
        ];
        let result_doc = Document::new(elements);

        let parsed = Transformer::parse(&document.as_bytes().into()).unwrap();

        assert_eq!(parsed, result_doc)
    }

    #[test]
    fn test_parse_table() {
        let document = r#"
| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |
          "#;
        let elements = vec![Table {
            headers: vec![
                TableHeader {
                    element: Text {
                        text: "Syntax".to_string(),
                        size: 14,
                    },
                    width: 30.0,
                },
                TableHeader {
                    element: Text {
                        text: "Description".to_string(),
                        size: 14,
                    },
                    width: 30.0,
                },
            ],
            rows: vec![
                TableRow {
                    cells: vec![
                        TableCell {
                            element: Text {
                                text: "Header".to_string(),
                                size: 14,
                            },
                        },
                        TableCell {
                            element: Text {
                                text: "Title".to_string(),
                                size: 14,
                            },
                        },
                    ],
                },
                TableRow {
                    cells: vec![
                        TableCell {
                            element: Text {
                                text: "Paragraph".to_string(),
                                size: 14,
                            },
                        },
                        TableCell {
                            element: Text {
                                text: "Text".to_string(),
                                size: 14,
                            },
                        },
                    ],
                },
            ],
        }];

        let result_doc = Document::new(elements);
        let parsed = Transformer::parse_with_loader(
            &document.as_bytes().into(),
            disk_image_loader("test/data"),
        )
        .unwrap();

        assert_eq!(parsed, result_doc)
    }

    #[test]
    fn test_html_to_markdown_to_cdm() -> anyhow::Result<()> {
        let input = r#"
            <html>
              <head>
                <title>Chew dad's slippers</title>
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

                <ul>
                  <li>Line item 1</li>
                  <li>Line item 2</li>
                  <li>Line item 3</li>
                </ul>
              </body>
            </html>
        "#;
        let input = &input.as_bytes().into();
        let doc_from_html =
            html::Transformer::parse_with_loader(input, disk_image_loader("test/data"))?;
        info!("{:#?}", doc_from_html);
        let parsed_html_bytes =
            Transformer::generate_with_saver(&doc_from_html, disk_image_saver("test/data"))?;

        let doc_from_markdown =
            Transformer::parse_with_loader(&parsed_html_bytes, disk_image_loader("test/data"));
        info!("{:#?}", doc_from_markdown);
        info!("{}", std::str::from_utf8(&parsed_html_bytes)?);
        assert!(true);

        Ok(())
    }
}

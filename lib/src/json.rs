use crate::core::{Document, Element, ImageAlignment, ImageData, ImageDimension, ImageType, ListItem, PageDimensions, PageFormat, TableCell, TableHeader, TableRow, TransformerTrait};
use bytes::Bytes;
use serde_json::Value;
use std::str::FromStr;
pub struct Transformer;

impl TransformerTrait for Transformer {

        fn parse(document: &Bytes) -> anyhow::Result<Document> {
            // Преобразуем Bytes в строку
            let data_str = std::str::from_utf8(document)?;
            let json: Value = serde_json::from_str(data_str)?;

            // Проверяем, что корневой элемент - объект
            let root = json.as_object().ok_or_else(|| anyhow::anyhow!("Root element is not a JSON object"))?;

            // Initialize dimensions
            let PageDimensions {
                mut page_width,
                mut page_height,
                mut page_margin_top,
                mut page_margin_bottom,
                mut page_margin_left,
                mut page_margin_right,
            } = PageFormat::default().dimensions();
            
            // Mapping of JSON keys to corresponding dimensions
            let mappings = vec![
                ("page_width", &mut page_width),
                ("page_height", &mut page_height),
                ("top_page_indent", &mut page_margin_top),
                ("bottom_page_indent", &mut page_margin_bottom),
                ("left_page_indent", &mut page_margin_left),
                ("right_page_indent", &mut page_margin_right),
            ];
                
            // Iterate through the mappings and update values if they exist
            for (key, target) in mappings {
                if let Some(value) = root.get(key).and_then(|v| v.as_f64()) {
                    *target = value as f32;
                } else {
                    return Err(anyhow::anyhow!("Missing or invalid '{}'", key));
                }
            }
                
            // Извлекаем элементы
            let elements = parse_elements(&root.get("elements").ok_or_else(|| anyhow::anyhow!("Missing 'elements' field"))?.clone())?;
            // Извлекаем заголовки и нижние колонтитулы страницы
            let page_header = parse_elements(&root.get("page_header").unwrap_or(&Value::Array(vec![])))?;
            let page_footer = parse_elements(&root.get("page_footer").unwrap_or(&Value::Array(vec![])))?;

            let page_custom_format = PageFormat::Custom(PageDimensions {
                page_width,
                page_height,
                page_margin_top,
                page_margin_bottom,
                page_margin_left,
                page_margin_right,
            });
    
            let document = Document::new_with_dimensions(page_header, elements, page_footer, page_custom_format);
            Ok(document)
        }



    fn generate(document: &Document) -> anyhow::Result<Bytes> {
            use serde_json::{Value, Map};
            // Helper function to serialize an Element into serde_json::Value
            fn serialize_element(element: &Element) -> Value {
                match element {
                    Element::Text { text, size } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Text".to_string()));
                        map.insert("text".to_string(), Value::String(text.clone()));
                        map.insert("size".to_string(), Value::Number((*size).into()));
                        Value::Object(map)
                    },
                    Element::Header { level, text } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Header".to_string()));
                        map.insert("level".to_string(), Value::Number((*level).into()));
                        map.insert("text".to_string(), Value::String(text.clone()));
                        Value::Object(map)
                    },
                    Element::Paragraph { elements } => {
                        let elements_json = elements.iter().map(serialize_element).collect();
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Paragraph".to_string()));
                        map.insert("elements".to_string(), Value::Array(elements_json));
                        Value::Object(map)
                    },
                    Element::Table { headers, rows } => {
                        let headers_json: Vec<Value> = headers.iter().map(|h| {
                            let mut h_map = Map::new();
                            h_map.insert("element".to_string(), serialize_element(&h.element));
                            h_map.insert("width".to_string(), Value::Number(serde_json::Number::from_f64(h.width as f64).unwrap()));
                            Value::Object(h_map)
                        }).collect();

                        let rows_json: Vec<Value> = rows.iter().map(|r| {
                            let cells_json: Vec<Value> = r.cells.iter().map(|c| serialize_element(&c.element)).collect();
                            let mut row_map = Map::new();
                            row_map.insert("cells".to_string(), Value::Array(cells_json));
                            Value::Object(row_map)
                        }).collect();

                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Table".to_string()));
                        map.insert("headers".to_string(), Value::Array(headers_json));
                        map.insert("rows".to_string(), Value::Array(rows_json));
                        Value::Object(map)
                    },
                    Element::List { elements, numbered } => {
                        let elements_json: Vec<Value> = elements.iter().map(|item| {
                            let mut item_map = Map::new();
                            item_map.insert("element".to_string(), serialize_element(&item.element));
                            Value::Object(item_map)
                        }).collect();

                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("List".to_string()));
                        map.insert("numbered".to_string(), Value::Bool(*numbered));
                        map.insert("elements".to_string(), Value::Array(elements_json));
                        Value::Object(map)
                    },
                    Element::Image(image_data) => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Image".to_string()));
                        // Encode image bytes to base64 for JSON representation
                        map.insert("bytes".to_string(), Value::String(base64::encode(&image_data.bytes())));
                        map.insert("title".to_string(), Value::String(image_data.title().to_string()));
                        map.insert("alt".to_string(), Value::String(image_data.alt().to_string()));
                        map.insert("image_type".to_string(), Value::String(image_data.image_type().to_string()));


                        map.insert("align".to_string(), Value::String(image_data.align().to_string().to_lowercase()));

                        let mut size_map = Map::new();
                        if let Some(width) = &image_data.size().width {
                            size_map.insert("width".to_string(), Value::String(width.clone()));
                        }
                        if let Some(height) = &image_data.size().height {
                            size_map.insert("height".to_string(), Value::String(height.clone()));
                        }
                        map.insert("size".to_string(), Value::Object(size_map));
                        Value::Object(map)
                    },
                    Element::Hyperlink { title, url, alt, size } => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String("Hyperlink".to_string()));
                        map.insert("title".to_string(), Value::String(title.clone()));
                        map.insert("url".to_string(), Value::String(url.clone()));
                        map.insert("alt".to_string(), Value::String(alt.clone()));
                        map.insert("size".to_string(), Value::Number((*size).into()));
                        Value::Object(map)
                    },
                }
            }

            // Serialize the main Document
            let mut doc_map = Map::new();

            // Serialize elements
            let elements_json: Vec<Value> = document.get_detail().into_iter().map(serialize_element).collect();
            doc_map.insert("elements".to_string(), Value::Array(elements_json));

            // Serialize page dimensions and indents
            doc_map.insert("page_width".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_width as f64).unwrap()));
            doc_map.insert("page_height".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_height as f64).unwrap()));
            doc_map.insert("left_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_margin_left as f64).unwrap()));
            doc_map.insert("right_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_margin_right as f64).unwrap()));
            doc_map.insert("top_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_margin_top as f64).unwrap()));
            doc_map.insert("bottom_page_indent".to_string(), Value::Number(serde_json::Number::from_f64(document.page_format.dimensions().page_margin_bottom as f64).unwrap()));

            // Serialize page headers
            let page_header_json: Vec<Value> = document.get_page_header().into_iter().map(serialize_element).collect();
            doc_map.insert("page_header".to_string(), Value::Array(page_header_json));

            // Serialize page footers
            let page_footer_json: Vec<Value> = document.get_page_footer().into_iter().map(serialize_element).collect();
            doc_map.insert("page_footer".to_string(), Value::Array(page_footer_json));

            // Create the final JSON value
            let doc_value = Value::Object(doc_map);

            // Serialize the JSON value to a string
            let json_string = doc_value.to_string();

            // Convert the string to Bytes and return
            Ok(Bytes::from(json_string))
        }

}

// Функция для разбора массива элементов
fn parse_elements(value: &Value) -> anyhow::Result<Vec<Element>> {
    let array = value.as_array().ok_or_else(|| anyhow::anyhow!("'elements' is not an array"))?;
    let mut elements = Vec::new();

    for item in array {
        let element = parse_element(item)?;
        elements.push(element);
    }

    Ok(elements)
}

// Функция для разбора отдельного элемента
fn parse_element(value: &Value) -> anyhow::Result<Element> {
    let obj = value.as_object().ok_or_else(|| anyhow::anyhow!("Element is not an object"))?;
    let type_str = obj.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Element missing 'type' field"))?;

    match type_str {
        "Text" => {
            let text = obj.get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Text element missing 'text' field"))?
                .to_string();
            let size = obj.get("size")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow::anyhow!("Text element missing or invalid 'size' field"))? as u8;
            Ok(Element::Text { text, size })
        },
        "Header" => {
            let level = obj.get("level")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow::anyhow!("Header element missing or invalid 'level' field"))? as u8;
            let text = obj.get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Header element missing 'text' field"))?
                .to_string();
            Ok(Element::Header { level, text })
        },
        "Paragraph" => {
            let elements = parse_elements(&obj.get("elements").ok_or_else(|| anyhow::anyhow!("Paragraph missing 'elements' field"))?.clone())?;
            Ok(Element::Paragraph { elements })
        },
        "Table" => {
            let headers = parse_table_headers(&obj.get("headers").ok_or_else(|| anyhow::anyhow!("Table missing 'headers' field"))?.clone())?;
            let rows = parse_table_rows(&obj.get("rows").ok_or_else(|| anyhow::anyhow!("Table missing 'rows' field"))?.clone())?;
            Ok(Element::Table { headers, rows })
        },
        "List" => {
            let numbered = obj.get("numbered")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| anyhow::anyhow!("List element missing or invalid 'numbered' field"))?;
            let list_items = obj.get("elements")
                .ok_or_else(|| anyhow::anyhow!("List element missing 'elements' field"))?;
            let items_array = list_items.as_array().ok_or_else(|| anyhow::anyhow!("List 'elements' is not an array"))?;
            let mut list_elements = Vec::new();
            for item in items_array {
                let list_item = parse_list_item(item)?;
                list_elements.push(list_item);
            }
            Ok(Element::List { elements: list_elements, numbered })
        },
        "Image" => {
            let bytes_str = obj.get("bytes")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Image element missing 'bytes' field"))?;
            let bytes = Bytes::from(base64::decode(bytes_str)?);
            let title = obj.get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Image element missing 'title' field"))?
                .to_string();
            let alt = obj.get("alt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Image element missing 'alt' field"))?
                .to_string();
            let image_type_str = obj.get("image_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Image element missing 'image_type' field"))?;
            let image_type = ImageType::from_str(image_type_str)
                .map_err(|_| anyhow::anyhow!("Invalid image_type: {}", image_type_str))?;
            let align_str = obj.get("align")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Image element missing 'align' field"))?;
            let align = ImageAlignment::from_str(align_str)
                .map_err(|_| anyhow::anyhow!("Invalid align: {}", align_str))?;
            let size_obj = obj.get("size").ok_or_else(|| anyhow::anyhow!("Image element missing 'size' field"))?;
            let width = size_obj.get("width")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let height = size_obj.get("height")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let size = ImageDimension { width, height };
            Ok(Element::Image(ImageData::new(
                bytes,
                title,
                alt,
                image_type.to_extension().to_string(), // Updated to use to_extension
                align_str.to_string(),
                size
            ))
            )
        },
        "Hyperlink" => {
            let title = obj.get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Hyperlink element missing 'title' field"))?
                .to_string();
            let url = obj.get("url")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Hyperlink element missing 'url' field"))?
                .to_string();
            let alt = obj.get("alt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Hyperlink element missing 'alt' field"))?
                .to_string();
            let size = obj.get("size")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow::anyhow!("Hyperlink element missing or invalid 'size' field"))? as u8;
            Ok(Element::Hyperlink { title, url, alt, size })
        },
        _ => Err(anyhow::anyhow!("Unknown element type: {}", type_str)),
    }
}

// Функция для разбора заголовков таблицы
fn parse_table_headers(value: &Value) -> anyhow::Result<Vec<TableHeader>> {
    let headers_array = value.as_array().ok_or_else(|| anyhow::anyhow!("'headers' is not an array"))?;
    let mut headers = Vec::new();

    for header in headers_array {
        let header_obj = header.as_object().ok_or_else(|| anyhow::anyhow!("Header is not an object"))?;
        let element = parse_element(&header_obj.get("element").ok_or_else(|| anyhow::anyhow!("Header missing 'element' field"))?.clone())?;
        let width = header_obj.get("width")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| anyhow::anyhow!("Header missing or invalid 'width' field"))? as f32;
        headers.push(TableHeader { element, width });
    }

    Ok(headers)
}

// Функция для разбора строк таблицы
fn parse_table_rows(value: &Value) -> anyhow::Result<Vec<TableRow>> {
    let rows_array = value.as_array().ok_or_else(|| anyhow::anyhow!("'rows' is not an array"))?;
    let mut rows = Vec::new();

    for row in rows_array {
        let row_obj = row.as_object().ok_or_else(|| anyhow::anyhow!("Row is not an object"))?;
        let cells = row_obj.get("cells")
            .ok_or_else(|| anyhow::anyhow!("Row missing 'cells' field"))?;
        let cells_array = cells.as_array().ok_or_else(|| anyhow::anyhow!("Row 'cells' is not an array"))?;
        let mut table_cells = Vec::new();
        for cell in cells_array {
            let cell_element = parse_element(cell)?;
            table_cells.push(TableCell { element: cell_element });
        }
        rows.push(TableRow { cells: table_cells });
    }

    Ok(rows)
}

// Функция для разбора элементов списка
fn parse_list_item(value: &Value) -> anyhow::Result<ListItem> {
    let obj = value.as_object().ok_or_else(|| anyhow::anyhow!("ListItem is not an object"))?;
    let element = parse_element(&obj.get("element").ok_or_else(|| anyhow::anyhow!("ListItem missing 'element' field"))?.clone())?;
    Ok(ListItem { element })
}


#[cfg(test)]
mod tests {
    use crate::core::{disk_image_loader, TransformerWithImageLoaderSaverTrait};
    use crate::json::{ TransformerTrait};

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

Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
        println!("{}", document);
        println!("==========================");
        let parsed_r = crate::markdown::Transformer::parse_with_loader(&document.as_bytes().into(), disk_image_loader("test/data"));
        let parsed = parsed_r?;
        println!("==========================");
        println!("{:#?}", parsed);
        println!("==========================");
        let generated_result = crate::json::Transformer::generate(&parsed);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes)?;
        println!("{}", generated_text);
        println!("==========================");
        let parsed_r2 = crate::json::Transformer::parse(&generated_bytes);
        let parsed2 = parsed_r2?;
        // generate markdown
        let generated_result2 = crate::text::Transformer::generate(&parsed2);
        assert!(generated_result2.is_ok());
        let generated_bytes2 = generated_result2?;
        let generated_text2 = std::str::from_utf8(&generated_bytes2)?;
        println!("{}", generated_text2);
        Ok(())
    }
}

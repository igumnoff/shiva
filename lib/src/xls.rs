use crate::core::Element::{ Table, Text};
use crate::core::*;
use bytes::Bytes;
use calamine::{ open_workbook_from_rs, Reader, Xls};
use std::collections::HashMap;
use std::io::Cursor;

pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, _images: &HashMap<String, Bytes>) -> anyhow::Result<Document>
        where Self: Sized
    {
        let cursor = Cursor::new(document.clone());

        let mut workbook: Xls<Cursor<Bytes>> = open_workbook_from_rs(cursor).expect(
            "Cannot open xls file from bytes"
        );

        let mut data: Vec<Element> = Vec::new();

        for sheet_name in workbook.sheet_names().clone() {
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let mut table_rows: Vec<TableRow> = Vec::new();
                    let mut table_headers: Vec<TableHeader> = Vec::new();
                    let mut is_first_row = true;

                    for row in range.rows() {
                        if is_first_row {
                            table_headers = row
                                .iter()
                                .map(|header| TableHeader {
                                    element: Text {
                                        text: header.to_string(),
                                        size: 8,
                                    },
                                    width: 10.0,
                                })
                                .collect();
                            is_first_row = false;
                        } else {
                            let cells = row
                                .iter()
                                .map(|header| TableCell {
                                    element: Text {
                                        text: header.to_string(),
                                        size: 8,
                                    },
                                })
                                .collect();
                            table_rows.push(TableRow { cells });
                        }
                    }
                    data.push(Table {
                        headers: table_headers.clone(),
                        rows: table_rows.clone(),
                    });
                }
                Err(err) => {
                    println!("Error reading sheet {}: {}", sheet_name, err);
                }
            }
        }

        Ok(Document::new(data))
    }

    fn generate(_document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)>
        where Self: Sized
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{ Read };
    use std::collections::HashMap;
    use anyhow::Ok;
    use bytes::Bytes;
    use crate::text;
    use crate::xls::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let path = "test/data/document.xls";
        let mut file = File::open(path).expect("Cannot open xls file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let bytes = Bytes::from(buffer);
        let images = HashMap::new();

        let parsed = Transformer::parse(&bytes, &images)?;

        println!("Parsed document: {:#?}", parsed);

        let generated_result = text::Transformer::generate(&parsed);
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);


        Ok(())
    }

}

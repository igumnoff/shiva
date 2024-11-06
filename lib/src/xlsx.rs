use crate::core::Element::{Table, Text};
use crate::core::*;
use bytes::Bytes;
use calamine::{open_workbook_from_rs, Reader, Xlsx};
use log::error;
use rust_xlsxwriter::*;
use std::io::Cursor;
pub struct Transformer;

impl TransformerTrait for Transformer {
    fn parse(document: &Bytes) -> anyhow::Result<Document>
    where
        Self: Sized,
    {
        let cursor = Cursor::new(document.clone());

        let mut workbook: Xlsx<Cursor<Bytes>> =
            open_workbook_from_rs(cursor).expect("Cannot open xlsx file from bytes");

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
                    error!("Error reading sheet {}: {}", sheet_name, err);
                }
            }
        }

        Ok(Document::new(data))
    }

    fn generate(document: &Document) -> anyhow::Result<Bytes>
    where
        Self: Sized,
    {
        let mut workbook = Workbook::new();
        fn generate_element(element: &Element, workbook: &mut Workbook) -> anyhow::Result<()> {
            if let Table { headers, rows } = element {
                let worksheet = workbook.add_worksheet();
                let mut row_index = 1;
                let mut col_index = 0;
                for header in headers {
                    if let Text { text, .. } = header.element.clone() {
                        worksheet.write_string(0, col_index, text)?;
                        col_index += 1;
                    }
                }

                for row in rows {
                    let mut col_index = 0;
                    for cell in row.cells.iter() {
                        if let Text { text, .. } = cell.element.clone() {
                            worksheet.write_string(row_index, col_index, text)?;
                            col_index += 1;
                        }
                    }
                    row_index += 1;
                }
            }
            Ok(())
        }
        for element in &document.get_detail() {
            generate_element(element, &mut workbook)?;
        }
        let xlsx_data = workbook.save_to_buffer()?;
        Ok(Bytes::from(xlsx_data))
    }
}

#[cfg(test)]
mod tests {
    use crate::xlsx::*;
    use anyhow::Ok;
    use bytes::Bytes;
    use log::{info, debug};
    use std::fs::File;
    use std::io::Read;
    use crate::core::tests::init_logger;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        init_logger();
        let path = "test/data/document.xlsx";
        let mut file = File::open(path).expect("Cannot open xlsx file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let bytes = Bytes::from(buffer);
        let parsed = Transformer::parse(&bytes)?;

        debug!("Parsed document: {:?}", parsed);

        Ok(())
    }

    #[test]
    fn test_generate() -> anyhow::Result<()> {
        init_logger();
        let path = "test/data/document.xlsx";
        let mut file = File::open(path).expect("Cannot open xlsx file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let bytes = Bytes::from(buffer);
        let parsed = Transformer::parse(&bytes)?;

        let generated_data: Result<bytes::Bytes, anyhow::Error> = Transformer::generate(&parsed);

        let bytes_to_write = generated_data?;
        std::fs::write("test/data/test_document.xlsx", bytes_to_write)?;

        info!("Excel file created successfully!");

        Ok(())
    }
}

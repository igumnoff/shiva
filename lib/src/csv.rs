use crate::core::{
    Document,
    Element::{Table, Text},
    TableCell, TableHeader, TableRow, TransformerTrait,
};
use bytes::Bytes;
pub struct Transformer;

#[allow(unused)]

impl TransformerTrait for Transformer {
    /// Parses CSV data from a `Bytes` object and converts it into a structured `Document`.
    /// This function is used to interpret CSV data, typically transforming it into a format
    /// that can be more easily manipulated within the system.
    ///
    /// # Arguments
    /// * `document` - The CSV data encapsulated in a `Bytes` instance.
    ///
    /// # Returns
    /// A result containing a `Document` if successful, or an `anyhow::Error` in case of failure.
    fn parse(document: &Bytes) -> anyhow::Result<Document> {
        // Deserialize the CSV data into a nested Vec structure.
        let document = serialize_csv(document)?;

        // Check if the document is empty and return an empty `Document` if so.
        if document.is_empty() {
            return Ok(Document::new(Vec::new()));
        }

        // Create an iterator over the rows of the CSV data.
        let mut document_iter = document.iter();

        // Extract the first row, which contains the headers.
        let headings: &Vec<String> = document_iter.next().unwrap();
        let mut headers: Vec<TableHeader> = Vec::new();

        // Process each header name to create `TableHeader` elements.
        for name in headings {
            headers.push(TableHeader {
                element: Text {
                    text: name.clone(),
                    size: 8, // Default font size
                },
                width: 10.0, // Default width, can be adjusted as needed
            });
        }

        // Prepare to collect the table rows.
        let mut rows: Vec<TableRow> = Vec::with_capacity(document.len() - 1);

        // Process each subsequent row in the CSV data.
        for lines in document_iter {
            let mut curr_row: Vec<TableCell> = Vec::with_capacity(headers.len());

            // Create a `TableCell` for each cell in the row.
            for cell in lines {
                curr_row.push(TableCell {
                    element: Text {
                        text: cell.clone(),
                        size: 8, // Default font size
                    },
                });
            }

            // Add the completed row to the rows collection.
            rows.push(TableRow { cells: curr_row });
        }

        // Construct the `Document` with the table created from the CSV data.
        Ok(Document::new(vec![Table { headers, rows }]))
    }

    fn generate(document: &Document) -> anyhow::Result<Bytes> {
        let elements = document.get_all_elements();

        let mut data: Vec<Vec<String>> = Vec::new();

        for element in elements {
            if let Table { headers, rows } = element {
                // Create a new vector for the header row
                let mut header_line = Vec::new();
                for header in headers {
                    if let Text { text, size: _ } = &header.element {
                        header_line.push(text.clone())
                    }
                }
                // Push header row to data
                data.push(header_line);

                // Iterate over each row
                for row in rows {
                    let mut curr_line = Vec::new(); // This must be inside the loop
                    for cell in &row.cells {
                        if let Text { text, size: _ } = &cell.element {
                            curr_line.push(text.clone())
                        }
                    }
                    // Push each row to data
                    data.push(curr_line);
                }
            }
        }

        // Serialize the data into CSV format and convert it to Bytes
        let csv_bytes = deserialize_csv(&data)?;

        // Return Bytes and an empty HashMap for images or additional data
        Ok(csv_bytes)
    }
}

fn serialize_csv(csv_data: &Bytes) -> anyhow::Result<Vec<Vec<String>>> {
    // Convert Bytes to a UTF-8 string slice
    let data_str = std::str::from_utf8(csv_data)?;

    // Create a CSV reader from a string slice
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // We consider that we have no headers so that they'll be preserved
        .from_reader(data_str.as_bytes());

    let mut data = Vec::new();

    // Iterate through each record
    for result in rdr.records() {
        let record = result?; // Get the record or an error
        let row: Vec<String> = record.iter().map(String::from).collect(); // Convert StringRecord to Vec<String>
        data.push(row); // Push the row into the data vector
    }

    Ok(data)
}

fn deserialize_csv(data: &Vec<Vec<String>>) -> anyhow::Result<Bytes> {
    // Create a CSV writer that writes into a string
    let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);

    // Iterate over the data and write each row to the CSV writer
    for row in data {
        wtr.write_record(row)?;
    }

    // After writing all data, we consume the writer to get the underlying string
    let csv_data = String::from_utf8(wtr.into_inner()?)?;

    Ok(bytes::Bytes::from(csv_data))
}

#[cfg(test)]
mod tests {
    use log::info;

    use crate::core::TransformerTrait;
    use crate::csv::{self, deserialize_csv, serialize_csv};
    use crate::markdown;

    #[test]
    fn test() -> anyhow::Result<()> {
        let document = r#"StudentID,Name,Math,Science,English
1,"John,Doe",88,92,85
2,"Jane Smith",94,95,91
3,"Emily ""Johnson""",,88,83
4,Robert Robinson,99,96,97"#;
        let parsed = csv::Transformer::parse(&document.as_bytes().into());

        assert!(parsed.is_ok());

        let parsed = parsed?;
        {
            let generated = markdown::Transformer::generate(&parsed)?;
            let generated_string = std::str::from_utf8(&generated)?;
            info!("{}", generated_string);
        }

        let generated = csv::Transformer::generate(&parsed)?;
        let generated_string = std::str::from_utf8(&generated)?;

        assert_eq!(
            generated_string,
            // this assures that new changes made to test string won't
            // affect the result of tests if the implementation is correct
            deserialize_csv(&serialize_csv(&bytes::Bytes::from(document))?)?
        );

        Ok(())
    }
}

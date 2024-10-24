use bytes::Bytes;
use clap::{Parser, ValueHint};
use shiva::core::{Document, DocumentType};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    name = "shiva",
    author,
    version,
    about = "CLI Shiva: Converting documents from any format to any",
    long_about = None
)]
struct Args {
    #[arg(
        value_name = "INPUT_FILE",
        help = &format!(
            "Input file (possible formats: {})",
            DocumentType::supported_extensions().join(", ")
        ),
        value_hint = ValueHint::FilePath
    )]
    input_file: String,

    #[arg(
        value_name = "OUTPUT_FILE",
        help = &format!(
            "Output file (possible formats: {})",
            DocumentType::supported_extensions().join(", ")
        ),
        value_hint = ValueHint::FilePath
    )]
    output_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input_path = Path::new(&args.input_file);
    let output_path = Path::new(&args.output_file);

    let supported_formats = DocumentType::supported_extensions();

    let input_format = match input_path.extension() {
        Some(ext) => ext.to_str().ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid extension of the input file. Supported formats are: {}",
                supported_formats.join(", ")
            )
        })?,
        None => {
            return Err(anyhow::anyhow!(
                "The input file has no extension. Supported formats are: {}",
                supported_formats.join(", ")
            ))
        }
    };

    let output_format = match output_path.extension() {
        Some(ext) => ext.to_str().ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid output file extension. Supported formats are: {}",
                supported_formats.join(", ")
            )
        })?,
        None => {
            return Err(anyhow::anyhow!(
                "The output file has no extension. Supported formats are: {}",
                supported_formats.join(", ")
            ))
        }
    };

    let input_doc_type = DocumentType::from_extension(input_format).ok_or_else(|| {
        anyhow::anyhow!(
            "Unsupported input file format '{}'. Supported formats are: {}",
            input_format,
            supported_formats.join(", ")
        )
    })?;

    let output_doc_type = DocumentType::from_extension(output_format).ok_or_else(|| {
        anyhow::anyhow!(
            "Unsupported output file format '{}'. Supported formats are: {}",
            output_format,
            supported_formats.join(", ")
        )
    })?;

    let input_vec = std::fs::read(&args.input_file)?;
    let input_bytes = Bytes::from(input_vec);

    let document = Document::parse(&input_bytes, input_doc_type)?;

    let output = document.generate(output_doc_type)?;

    std::fs::write(&args.output_file, output)?;

    Ok(())
}

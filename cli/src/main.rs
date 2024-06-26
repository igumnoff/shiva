use bytes::Bytes;
use clap::Parser;
use shiva::core::{Document, DocumentType};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name="shiva", author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input_file: String,

    #[arg(long)]
    output_file: String,

    #[arg(long, value_parser = DocumentType::variants_as_str() )]
    input_format: String,

    #[arg(long, value_parser = DocumentType::variants_as_str() )]
    output_format: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input_vec = std::fs::read(
        args.input_file
    )?;
    let input_bytes = Bytes::from(input_vec);

    let document = Document::parse(&input_bytes, DocumentType::from_str(args.input_format.as_str())?);

    let output = document?.generate(DocumentType::from_str(args.output_format.as_str())?)?;

    let file_name = args.output_file;
    std::fs::write(file_name, output)?;

    Ok(())
}
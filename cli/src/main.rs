use bytes::Bytes;
use clap::{Parser, ValueEnum};
use shiva::core::{TransformerTrait};

#[derive(Parser, Debug)]
#[command(name="shiva", author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input_file: String,

    #[arg(long)]
    output_file: String,

    #[arg(long)]
    input_format: DocumentType,

    #[arg(long)]
    output_format: DocumentType,
}


#[derive(Debug, Parser, Clone, ValueEnum)]
pub enum DocumentType {
    HTML,
    Markdown,
    Text,
    PDF,
    JSON,
    CSV,
    RTF,
    DOCX,
    XML,
    XLS,
    XLSX,
    ODS,
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input_vec = std::fs::read(
        args.input_file
    )?;
    let input_bytes = Bytes::from(input_vec);

    let document = match args.input_format {
        DocumentType::Markdown => shiva::markdown::Transformer::parse(&input_bytes)?,
        DocumentType::HTML => shiva::html::Transformer::parse(&input_bytes)?,
        DocumentType::Text => shiva::text::Transformer::parse(&input_bytes)?,
        DocumentType::PDF => shiva::pdf::Transformer::parse(&input_bytes)?,
        DocumentType::JSON => shiva::json::Transformer::parse(&input_bytes)?,
        DocumentType::CSV => shiva::csv::Transformer::parse(&input_bytes)?,
        DocumentType::RTF => shiva::rtf::Transformer::parse(&input_bytes)?,
        DocumentType::DOCX => shiva::docx::Transformer::parse(&input_bytes)?,
        DocumentType::XML => shiva::xml::Transformer::parse(&input_bytes)?,
        DocumentType::XLS => shiva::xls::Transformer::parse(&input_bytes)?,
        DocumentType::XLSX => shiva::xlsx::Transformer::parse(&input_bytes)?,
        DocumentType::ODS => shiva::ods::Transformer::parse(&input_bytes)?,
    };

    let output = match args.output_format {
        DocumentType::Text => shiva::text::Transformer::generate(&document)?,
        DocumentType::HTML => shiva::html::Transformer::generate(&document)?,
        DocumentType::Markdown => shiva::markdown::Transformer::generate(&document)?,
        DocumentType::PDF => shiva::pdf::Transformer::generate(&document)?,
        DocumentType::JSON => shiva::json::Transformer::generate(&document)?,
        DocumentType::CSV => shiva::csv::Transformer::generate(&document)?,
        DocumentType::RTF => shiva::rtf::Transformer::generate(&document)?,
        DocumentType::DOCX => shiva::docx::Transformer::generate(&document)?,
        DocumentType::XML => shiva::xml::Transformer::generate(&document)?,
        DocumentType::XLS => shiva::xls::Transformer::generate(&document)?,
        DocumentType::XLSX => shiva::xlsx::Transformer::generate(&document)?,
        DocumentType::ODS => shiva::ods::Transformer::generate(&document)?,
    };

    let file_name = args.output_file;
    std::fs::write(file_name, output)?;

    Ok(())
}

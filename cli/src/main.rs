use bytes::Bytes;
use clap::{Parser, ValueEnum};
use shiva::core::TransformerTrait;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name="shiva", author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input_file: Option<String>,

    #[arg(long)]
    output_file: Option<String>,

    #[arg(long)]
    input_format: InputFormat,

    #[arg(long)]
    output_format: InputFormat,
}

#[derive(Debug, Parser, Clone, ValueEnum)]
enum InputFormat {
    Markdown,
    Html,
    Text,
    Pdf,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input_vec = std::fs::read(
        args.input_file
            .ok_or(anyhow::anyhow!("No input file provided"))?,
    )?;
    let input_bytes = Bytes::from(input_vec);

    let document = match args.input_format {
        InputFormat::Markdown => shiva::markdown::Transformer::parse(&input_bytes, &HashMap::new())?,
        InputFormat::Html => shiva::html::Transformer::parse(&input_bytes, &HashMap::new())?,
        InputFormat::Text => shiva::text::Transformer::parse(&input_bytes, &HashMap::new())?,
        InputFormat::Pdf => shiva::pdf::Transformer::parse(&input_bytes, &HashMap::new())?,
    };

    let output = match args.output_format {
        InputFormat::Text => shiva::text::Transformer::generate(&document)?,
        InputFormat::Html => shiva::html::Transformer::generate(&document)?,
        InputFormat::Markdown => shiva::markdown::Transformer::generate(&document)?,
        InputFormat::Pdf => shiva::pdf::Transformer::generate(&document)?,
    };

    let file_name = args
        .output_file
        .ok_or(anyhow::anyhow!("No output file provided"))?;
    std::fs::write(file_name, output.0)?;

    Ok(())
}

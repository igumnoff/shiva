use std::collections::HashMap;
use bytes::Bytes;
use clap::{Parser, ValueEnum};
use shiva::core::TransformerTrait;


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

    let input_vec = std::fs::read(args.input_file.ok_or(anyhow::anyhow!("No input file provided"))?)?;
    let input_bytes = Bytes::from(input_vec);


    let document = match args.input_format {
        InputFormat::Markdown => {
            use shiva::markdown::Transformer;
            Transformer::parse(&input_bytes, &HashMap::new())?
                    }
        InputFormat::Html => {
            todo!()
        }
        InputFormat::Text => {
            todo!()
        }
        InputFormat::Pdf => {
            todo!()
        }
    };


    let output = match args.output_format {
        InputFormat::Text => {
            use shiva::text::Transformer;
            Transformer::generate(&document)?
        }
        InputFormat::Html => {
            todo!()
        }
        InputFormat::Markdown => {
            todo!()
        }
        InputFormat::Pdf => {
            todo!()
        }

    };

    Ok(())


}
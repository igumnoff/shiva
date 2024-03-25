use clap::Parser;


#[derive(Parser, Debug)]
#[command(name="shiva", author, version, about, long_about = None)]
struct Args {

    #[arg(long)]
    input_file: Option<String>,

    #[arg(long)]
    output_file: Option<String>,

    #[arg(long)]
    input_format: String,

    #[arg(long)]
    output_format: String,



}
fn main() {


    let args = Args::parse();


}
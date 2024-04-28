# Shiva

![shiva](https://github.com/igumnoff/shiva/raw/HEAD/logo.png)

**Shiva library: Implementation in Rust of a parser and generator for documents of any type**

## Features
- [x] Common Document Model (CDM) for all document types
- [x] Parsers produce CDM
- [x] Generators consume CDM

## Supported document types

| Document type | Parse | Generate |
|---------------|-------|----------|
| Plain text    | +     | +        |
| Markdown      | +     | +        |
| HTML          | +     | +        |
| PDF           | +     | +        |
| JSON          | +     | +        |
| XML           | -     | -        |
| DOC           | -     | -        |
| XLS           | -     | -        |


## Parse document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink | PageHeader | PageFooter |
|---------------|--------|-----------|------|-------|-------|-----------|------------|------------|
| Plain text    | -      | +         | -    | -     | -     | -         | -          | -          |
| Markdown      | +      | +         | +    | +     | +     | +         | -          | -          |
| HTML          | +      | +         | +    | +     | +     | +         | -          | -          |
| PDF           | -      | +         | +    | -     | -     | -         | -          | -          |
| JSON          | +      | +         | +    | +     | -     | +         | +          | +          |

## Generate document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink | PageHeader | PageFooter |
|---------------|--------|-----------|------|-------|-------|-----------|------------|------------|
| Plain text    | +      | +         | +    | +     | -     | +         | +          | +          |
| Markdown      | +      | +         | +    | +     | +     | +         | +          | +          |
| HTML          | +      | +         | +    | +     | +     | +         | -          | -          |
| PDF           | +      | +         | +    | +     | -     | +         | +          | +          |
| JSON          | +      | +         | +    | +     | -     | +         | +          | +          |

## Usage Shiva library

Cargo.toml
```toml
[dependencies]
shiva = {  version = "0.3.1", features = ["html", "markdown", "text", "pdf", "json"] }
```

main.rs
```rust
fn main() {
    let input_vec = std::fs::read("input.html").unwrap();
    let input_bytes = bytes::Bytes::from(input_vec);
    let document = shiva::html::Transformer::parse(&input_bytes, &HashMap::new()).unwrap();
    let output_bytes = shiva::markdown::Transformer::generate(&document, &HashMap::new()).unwrap();
    std::fs::write("out.md", output_bytes).unwrap();
}
```


## Shiva CLI
### Install Rust for Linux/MacOS
```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Install Rust for Windows
```bash
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
```
### Build executable Shiva
```bash
gti clone https://github.com/igumnoff/shiva.git
cd shiva/cli
cargo build --release
```
### Run executable shiva
```bash
cd ./target/release/
./shiva --input-format=markdown --output-format=html --input-file=README.md --output-file=README.html
```

## Contributing
I would love to see contributions from the community. If you experience bugs, feel free to open an issue. If you would like to implement a new feature or bug fix, please follow the steps:
1. Contact with me via telegram: @ievkz
2. Confirm e-mail invitation in repository
3. Do "git clone"
4. Create branch with your assigned issue
5. Create pull request to main branch

## Who uses Shiva
- [Metatron library: Implementation in Rust of a report generation](https://github.com/igumnoff/metatron)



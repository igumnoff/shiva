# Shiva

![shiva](https://github.com/igumnoff/shiva/raw/HEAD/logo.png)

**Shiva library: Implementation in Rust of a parser and generator for documents of any type**

## Features
- [x] Common Document Model (CDM) for all document types
- [x] Parsers produce CDM
- [x] Generators consume CDM

## Common Document Model

![Common Document Model](https://github.com/igumnoff/shiva/raw/HEAD/CDM.png)


## Supported document types

| Document type | Parse | Generate |
|---------------|-------|----------|
| Plain text    | +     | +        |
| Markdown      | +     | +        |
| HTML          | +     | +        |
| PDF           | +     | +        |
| JSON          | +     | +        |
| XML           | +     | +        |
| CSV           | +     | +        |
| RTF           | +     | +        |
| DOCX          | +     | +        |
| XLS           | +     | -        |
| XLSX          | +     | +        |
| ODS           | +     | +        |
| Typst         | -     | +        |


## Parse document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink | PageHeader | PageFooter |
|---------------|--------|-----------|------|-------|-------|-----------|------------|------------|
| Plain text    | -      | +         | -    | -     | -     | -         | -          | -          |
| Markdown      | +      | +         | +    | +     | +     | +         | -          | -          |
| HTML          | +      | +         | +    | +     | +     | +         | -          | -          |
| PDF           | -      | +         | +    | -     | -     | -         | -          | -          |
| DOCX          | +      | +         | +    | +     | -     | +         | -          | -          |
| RTF           | +      | +         | +    | +     | -     | +         | +          | +          |
| JSON          | +      | +         | +    | +     | -     | +         | +          | +          |
| XML           | +      | +         | +    | +     | +     | +         | +          | +          |
| CSV           | -      | -         | -    | +     | -     | -         | -          | -          |
| XLS           | -      | -         | -    | +     | -     | -         | -          | -          |
| XLSX          | -      | -         | -    | +     | -     | -         | -          | -          |
| ODS           | -      | -         | -    | +     | -     | -         | -          | -          |

## Generate document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink | PageHeader | PageFooter |
|---------------|--------|-----------|------|-------|-------|-----------|------------|------------|
| Plain text    | +      | +         | +    | +     | -     | +         | +          | +          |
| Markdown      | +      | +         | +    | +     | +     | +         | +          | +          |
| HTML          | +      | +         | +    | +     | +     | +         | -          | -          |
| PDF           | +      | +         | +    | +     | +     | +         | +          | +          |
| DOCX          | +      | +         | +    | +     | +     | +         | -          | -          |
| RTF           | +      | +         | +    | +     | +     | +         | -          | -          |
| JSON          | +      | +         | +    | +     | -     | +         | +          | +          |
| XML           | +      | +         | +    | +     | +     | +         | +          | +          |
| CSV           | -      | -         | -    | +     | -     | -         | -          | -          |
| XLSX          | -      | -         | -    | +     | -     | -         | -          | -          |
| ODS           | -      | -         | -    | +     | -     | -         | -          | -          |
| Typst         | +      | +         | +    | +     | +     | +         | +          | +          |



## Usage Shiva library

Cargo.toml
```toml
[dependencies]
shiva = {  version = "1.4.4", features = ["html", "markdown", "text", "pdf", "json", 
    "csv", "rtf", "docx", "xml", "xls", "xlsx", "ods", "typst"] }
```

main.rs
```rust
fn main() {
    let input_vec = std::fs::read("input.html").unwrap();
    let input_bytes = bytes::Bytes::from(input_vec);
    let document = shiva::html::Transformer::parse(&input_bytes).unwrap();
    let output_bytes = shiva::markdown::Transformer::generate(&document).unwrap();
    std::fs::write("out.md", output_bytes).unwrap();
}
```


## Shiva CLI & Server
### Build executable Shiva CLI and Shiva Server
```bash
git clone https://github.com/igumnoff/shiva.git
cd shiva/cli
cargo build --release
```
### Run executable Shiva CLI
```bash
cd ./target/release/
./shiva README.markdown README.html
```

### Run Shiva Server
```bash
cd ./target/release/
./shiva-server --port=8080 --host=127.0.0.1
```

## Who uses Shiva
- [Metatron library: Implementation in Rust of a report generation](https://github.com/igumnoff/metatron)


## Contributing
I would love to see contributions from the community. If you experience bugs, feel free to open an issue. If you would like to implement a new feature or bug fix, please follow the steps:

1. Do fork 
2. Add comment to the [issue](https://github.com/igumnoff/shiva/issues) that you are going to work on it
3. Create pull request


If you would like add new document type, you need to implement the following traits:

### Required: shiva::core::TransformerTrait
```rust
pub trait TransformerTrait {
    fn parse(document: &Bytes) -> anyhow::Result<Document>;
    fn generate(document: &Document) -> anyhow::Result<Bytes>;
}

```


### Optional: shiva::core::TransformerWithImageLoaderSaverTrait (If images store outside of document for example: HTML, Markdown)

```rust
pub trait TransformerWithImageLoaderSaverTrait {
    fn parse_with_loader<F>(document: &Bytes,  image_loader: F) -> anyhow::Result<Document>
        where F: Fn(&str) -> anyhow::Result<Bytes>;
    fn generate_with_saver<F>(document: &Document,  image_saver: F) -> anyhow::Result<Bytes>
        where F: Fn(&Bytes, &str) -> anyhow::Result<()>;
}
```


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Shiva by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

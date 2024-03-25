# Shiva

![shiva](logo.png)

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
| PDF           | +     | -        |
| JSON          | -     | -        |
| XML           | -     | -        |
| DOC           | -     | -        |
| XLS           | -     | -        |


## Parse document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink |
|---------------|--------|-----------|------|-------|-------|-----------|
| Plain text    | -      | +         | -    | -     | -     | -         |
| Markdown      | +      | +         | +    | +     | +     | +         |
| HTML          | +      | +         | +    | +     | +     | +         |
| PDF           | -      | +         | +    | -     | -     | -         |

## Generate document features

| Document type | Header | Paragraph  | List | Table | Image | Hyperlink |
|---------------|--------|------------|------|-------|-------|-----------|
| Plain text    | +      | +          | +    | +     | -     | +         |
| Markdown      | +      | +          | +    | +     | +     | +         |
| HTML          | +      | +          | +    | +     | +     | +         |
| PDF           | -      | -          | -    | -     | -     | -         |

## Usage Shiva library

Cargo.toml
```toml
[dependencies]
shiva = "0.1.0"
```

main.rs
```rust
use shiva::*;
```
TODO

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
cd ../target/release/
./shiva --help
```
TODO
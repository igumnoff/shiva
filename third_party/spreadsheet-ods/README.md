
[![crates.io](https://img.shields.io/crates/v/spreadsheet-ods.svg)](https://crates.io/crates/spreadsheet-ods)
[![Documentation](https://docs.rs/spreadsheet-ods/badge.svg)](https://docs.rs/spreadsheet_ods)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![](https://tokei.rs/b1/github/thscharler/spreadsheet-ods)

spreadsheet-ods - Read and write ODS files
====

This crate can read and write back ODS spreadsheet files. 

Not all the specification is implemented yet. And there are parts for 
which there is no public API, but which are preserved as raw xml. More 
details in the documentation.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
spreadsheet-ods = "0.17.0"
```

Or use `cargo add`

```sh
$ cargo add spreadsheet-ods
```

## Features

* `use_decimal`: Add conversions for rust_decimal. Internally the values are
  stored as f64 nonetheless.

* Locales 
  * all_locales = [ "locale_de_AT", "locale_en_US" ]
  * locale_de_AT
  * locale_en_US

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Changes

[changes.md](https://github.com/thscharler/spreadsheet-ods/blob/master/changes.md)

## Contributing

I welcome all people who want to contribute.  

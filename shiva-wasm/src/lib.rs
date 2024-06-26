mod utils;

use wasm_bindgen::prelude::*;

use shiva::{core::TransformerTrait, core::DocumentType as FileFormat, *};

use crate::utils::set_panic_hook;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(catch)]
pub fn convert(
    file: Vec<u8>,
    input_format: FileFormat,
    output_format: FileFormat,
) -> Result<Vec<u8>, JsValue> {
    set_panic_hook();
    let parsed_file;
    match input_format {
        FileFormat::Text => {
            parsed_file = match text::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Markdown => {
            parsed_file = match markdown::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::DOCX => {
            parsed_file = match docx::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::HTML => {
            parsed_file = match html::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Json => {
            parsed_file = match json::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::XML => {
            parsed_file = match xml::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::CSV => {
            parsed_file = match csv::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::RTF => {
            parsed_file = match rtf::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::XLS => {
            parsed_file = match xls::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::XLSX => {
            parsed_file = match xlsx::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::ODS => {
            parsed_file = match ods::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
    }

    match output_format {
        FileFormat::Text => {
            let generated = match text::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Text err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Markdown => {
            let generated = match markdown::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Markdown err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::DOCX => {
            let generated = match docx::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Docx err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::HTML => {
            let generated = match html::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Html err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Json => {
            let generated = match json::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Json err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::XML => {
            let generated = match xml::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Xml err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::CSV => {
            let generated = match csv::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::RTF => {
            let generated = match rtf::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::XLS => {
            let generated = match xls::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::XLSX => {
            let generated = match xlsx::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::ODS => {
            let generated = match ods::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }

    }
}

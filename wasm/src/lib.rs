mod utils;

use wasm_bindgen::prelude::*;

use shiva::{core::TransformerTrait, *};

use crate::utils::set_panic_hook;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileFormat {
    Markdown = 0,
    Docx = 1,
    Html = 2,
    Text = 3,
    Json = 4,
    Xml = 5,
    Csv = 6,
    Rtf = 7,
    Xls = 8,    
    Xlsx = 9,    
    Ods = 10,
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
        FileFormat::Docx => {
            parsed_file = match docx::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Html => {
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
        FileFormat::Xml => {
            parsed_file = match xml::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Csv => {
            parsed_file = match csv::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Rtf => {
            parsed_file = match rtf::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Xls => {
            parsed_file = match xls::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Xlsx => {
            parsed_file = match xlsx::Transformer::parse(&file.into()) {
                Ok(parse_result) => parse_result,
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
        FileFormat::Ods => {
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
        FileFormat::Docx => {
            let generated = match docx::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Docx err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Html => {
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
        FileFormat::Xml => {
            let generated = match xml::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Xml err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Csv => {
            let generated = match csv::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Rtf => {
            let generated = match rtf::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Xls => {
            let generated = match xls::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Xlsx => {
            let generated = match xlsx::Transformer::generate(&parsed_file) {
                Ok(res) => res,
                Err(err) => {
                    log!(" FileFormat::Csv err {:#?}", err);
                    return Err(err.to_string().into());
                }
            };
            return Ok(generated.to_vec());
        }
        FileFormat::Ods => {
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

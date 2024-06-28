mod utils;

use wasm_bindgen::prelude::*;

use shiva::{core::DocumentType as FileFormat, core::Document};

use crate::utils::set_panic_hook;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn convert(
    file: Vec<u8>,
    input_format: FileFormat,
    output_format: FileFormat,
) -> Result<Vec<u8>, JsValue> {
    set_panic_hook();

    let parsed_file = match Document::parse(&file.into(),input_format) {
        Ok(parse_result) => parse_result,
        Err(e) => {
            return Err(e.to_string().into());
        }
    };

    let generated = match parsed_file.generate(output_format) {
        Ok(res) => res,
        Err(err) => {
            log!(" FileFormat::{} err {:#?}",output_format, err);
            return Err(err.to_string().into());
        }
    };
    return Ok(generated.to_vec());
}

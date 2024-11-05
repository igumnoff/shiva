use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::info;
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    ExtensionMissing,      //the file extension is missing
    FailBytes,             //binary conversion error
    UnsupportedFormat,     //unsupported format
    FailParseDocument,     //document parsing error
    FailConvertFile,       //file conversion error
    FailHeader,            //error creating the header of the converted file
    NoFilesToConvertInZip, //there are no files to convert in the zip archive
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("-->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

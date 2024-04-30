use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    NoExtension,
    FailBytes,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

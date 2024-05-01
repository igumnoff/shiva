use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use strum_macros::AsRefStr;


pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, Clone, AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    ExtensionMissing, //отсутствует расширение файла
    FailBytes, //ошибка преобразования в двоичный файл
    UnsupportedFormat, //неподдерживаемый формат
    FailParseDocument, //ошибка парсинга документа
    FailConvertFile, //ошибка конвертации файла
    FailHeader, //ошибка создания заголовка конвертированного файла
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

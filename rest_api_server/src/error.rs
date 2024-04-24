use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LoginFail,

    // --Ошибки аутентификации
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    TicketDeleteFailNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}", "INTO_RES");

//Создаём ответ
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        //вставка ошибки в ответ сервера
        response.extensions_mut().insert(self);

        response

    }
}

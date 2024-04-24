use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, AsRefStr)]
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

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            //Ошибки auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            //Ошибки модели
            Self::TicketDeleteFailNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            //Ошибки клиента
            _ => (StatusCode::INTERNAL_SERVER_ERROR,
                  ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)] //отключает ошибку правильности написания значений
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>; //Этот код позволяет использовать более короткое и удобное имя Result<T> вместо полного core::result::Result<T, Error> в других частях кода

#[derive(Clone, Debug, AsRefStr, Serialize)] //Clone: Позволяет создавать клоны (копии) объектов этого типа
//Debug: Позволяет использовать отладочный вывод для объектов этого типа (например, с помощью println!("{:?}", obj))
//AsRefStr: Добавляет реализацию трейта AsRef<str>, что позволяет получать ссылку на строковое представление объекта

#[serde(tag = "type", content = "data")] //tag = "type": Будет использоваться поле с именем "type" для хранения информации о типе объекта (т.е. о том, какой именно вариант перечисления (enum) представляет данный объект)
//content = "data": Остальные данные объекта будут храниться в поле с именем "data"
pub enum Error { //Определяет перечисление, которое представляет различные типы ошибок, которые могут возникать в приложении
    LoginFail, //Ошибка, возникающая при неудачной попытке входа в систему
    AuthFailNoAuthTokenCookie, //Ошибка аутентификации, связанная с отсутствием cookie с токеном авторизации
    AuthFailTokenWrongFormat, //Ошибка аутентификации, связанная с неправильным форматом токена авторизации
    AuthFailCtxNotInRequestExt, //Ошибка аутентификации, связанная с отсутствием контекста авторизации в запросе
    TicketDeleteFailNotFound { id: u64 }, //Ошибка при попытке удалить билет, который не был найден. Этот вариант ошибки содержит дополнительное поле id типа u64, которое хранит идентификатор билета
}


//Перечисление, которое представляет различные типы ошибок, которые могут быть возвращены клиенту приложения
#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)] //Отключает предупреждение компилятора о том, что названия вариантов перечисления должны быть в стиле CamelCase. Это позволяет использовать более понятные названия вариантов
pub enum ClientError {
    LOGIN_FAIL, //Ошибка, связанная с неудачной попыткой входа в систему
    NO_AUTH, //Ошибка, связанная с отсутствием или неправильным форматом токена авторизации
    INVALID_PARAMS, //Ошибка, связанная с некорректными параметрами
    SERVICE_ERROR, //Прочие, не перечисленные ошибки сервиса
}


//Реализует метод into_response для перечисления Error, который позволяет преобразовать значение типа Error в HTTP-ответ
impl IntoResponse for Error { //Создает реализацию трейта IntoResponse для типа Error, что означает, что тип Error теперь обладает методом into_response
    fn into_response(self) -> Response { //Принимает self (владение объектом Error) и возвращает объект типа Response
        println!("-->> {:<12} - {self:?}", "INTO_RES"); //Выводит отладочное сообщение, указывающее на начало преобразования объекта Error в ответ

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response(); //Создается переменная с ошибкой из перечисления Error

        //вставка ошибки в ответ сервера
        response.extensions_mut().insert(self); //Объект Error (ошибка) вставляется в расширения (extensions) объекта response. Это позволяет сохранить информацию об ошибке внутри объекта ответа

        response //Возвращает объект response, который содержит информацию об ошибке и будет использоваться для формирования HTTP-ответа

    }
}


///Реализация методов для типа Error, возвращает кортеж, содержащий HTTP-статус и тип ошибки клиента, в зависимости от типа ошибки
//другими словами - определяем, какой HTTP-статус и тип ошибки клиента должны быть возвращены в зависимости от конкретного типа ошибки Error
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)] //Отключает предупреждение компилятора о недостижимых шаблонах в match выражении
        match self { //Сопоставляет различные варианты ошибок типа Error и возвращает соответствующие HTTP-статус и тип ошибки клиента
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            Self::TicketDeleteFailNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            //Прочие ошибки не указанные в структуре Error
            _ => (StatusCode::INTERNAL_SERVER_ERROR,
                  ClientError::SERVICE_ERROR,
            ),
        }
    }
}
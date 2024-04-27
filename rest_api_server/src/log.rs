use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use crate::{Error, Result};
use crate::ctx::Ctx;
use crate::error::ClientError;

//Структура записи лога при возникновении ошибки
#[skip_serializing_none] //Указывает сериализатору, что поля со значением None должны быть пропущены при сериализации объекта
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    //атрибуты юзера из контекста
    user_id: Option<u64>,

    //атрибуты http запросов
    req_path: String,
    req_method: String,

    //атрибуты ошибок
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>
}


//Записывает информацию о запросе и связанных с ним ошибках в журнал (log).
pub async fn log_request(
    uuid: Uuid, //Уникальный идентификатор запроса
    req_method: Method, //HTTP-метод запроса (GET, POST and other)
    uri: Uri, //URI запроса
    ctx: Option<Ctx>, //Контекст запроса
    service_error: Option<&Error>, //Ошибка, возникшая на стороне сервиса (получаем из функции main::main_response_mapper)
    client_error: Option<ClientError> //Ошибка, которая будет возвращена клиенту (получаем из функции main::main_response_mapper)
) -> Result<()> {
    //Создает время возникновения ошибки
    let timestamp = SystemTime::now() //Получает текущее системное время
        .duration_since(UNIX_EPOCH) //Вычисляет длительность между текущим временем и началом эпохи Unix (1 января 1970 года 00:00:00 UTC)
        .unwrap() //Извлекает значение из Result, предполагая, что оно не содержит ошибку
        .as_millis(); //Преобразует длительность в миллисекунды

    //Создает строковое представление ошибки
    let error_type = service_error
        .map(|se| se.as_ref() //Получает ссылку на объект ошибки se из service_error
            .to_string()); //Преобразовывает в строку

    //Преобразует объект service_error в значение JSON, извлекает поле "data" из этого JSON-объекта и возвращает его в виде опционального значения error_data
    let error_data = serde_json::to_value(service_error) //Преобразует объект service_error в значение JSON
    .ok() //Извлекает значение из Result, если оно успешно, или возвращает None, если произошла ошибка
    .and_then(|mut v| v //Получает мутабельную ссылку на значение JSON, созданное на предыдущем шаге
        .get_mut("data") //Получает мутабельную ссылку на поле "data" в этом значении JSON
        .map(|v| v.take())); //Извлекает значение из поля "data" и перемещает его (т.е. "взять" его) из значения JSON

//Создает экземпляр структуры из данных полученных из входных параметров
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!("    ->> log_request: \n{}", json!(log_line));

    Ok(())
}
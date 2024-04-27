pub use self::error::{Error, Result};

use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Json, middleware, Router};
use axum::http::{Method, Uri};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::log::log_request;

mod log; //Подключает файл log.rs
mod ctx; //Подключает файл ctx.rs
mod error; //Подключает файл error.rs
mod model; //Подключает файл model.rs
mod web; //Подключает файл mod.rs для подключения всех перечисленных в нём файлов

#[tokio::main] //Атрибут, указывающий, что функция main() будет использовать Tokio runtime
async fn main() -> Result<()> { //Объявление асинхронной функции main(), которая возвращает Result<()>. Это означает, что функция может выполняться асинхронно и может вернуть либо () в случае успеха, либо ошибку типа Result

    let mc = ModelController::new().await?; //Создание нового экземпляра ModelController с помощью его асинхронного метода new(), который возвращает Future. Оператор await ожидает завершения этой операции. Затем ? обрабатывает ошибку, если она возникает при инициализации ModelController

    // Создание маршрутов для API с помощью функции routes() из модуля web::routes_tickets
    let routes_apis = web::routes_tickets::routes(mc.clone()) //Вызываем функцию routes() из модуля web::routes_tickets. Эта функция создает и возвращает набор маршрутов для работы с билетами. Метод clone() используется для создания копии ModelController, чтобы передать его в функцию routes(), не передавая исходный объект по ссылке.
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth)); // добавление промежуточного слоя маршрутам с помощью метода route_layer(). В качестве аргумента передается промежуточная функция, созданная с помощью middleware::from_fn(), которая в свою очередь вызывает функцию mw_require_auth из модуля web::mw_auth. mw_require_auth отвечает за аутентификацию пользователей при доступе к API.

    //Создание маршрутов, их объединение и добавление промежуточных слоев
    let routes_all = Router::new() //Создание нового объекта маршрутизатора (router) с помощью статического метода new()
        .merge(routes_hello()) //Этот вызов добавляет маршруты, возвращаемые функцией routes_hello(), к текущему маршрутизатору. routes_hello() возвращает набор маршрутов для обработки запросов, связанных с приветствием на сервере
        .merge(web::routes_login::routes()) //Этот вызов добавляет маршруты для авторизации, возвращаемые функцией routes() из модуля web::routes_login, к текущему маршрутизатору
        .nest("/api", routes_apis) //Этот вызов вкладывает маршруты, содержащиеся в переменной routes_apis, под префикс "/api". То есть, все маршруты, определенные в routes_apis, будут доступны по адресу, начинающемуся с "/api"
        .layer(middleware::map_response(main_response_mapper)) //Здесь добавляется промежуточный слой, который применяет функцию main_response_mapper ко всем ответам, возвращаемым при обработке запросов. Эта функция используется для обработки и логирования ответов сервера
        .layer(middleware::from_fn_with_state( //Этот вызов добавляет еще один промежуточный слой с помощью функции from_fn_with_state()
            mc.clone(), //Передает в функцию from_fn_with_state текущее состояние (state)
            web::mw_auth::mw_ctx_resolver, //Передает в функцию from_fn_with_state функцию mw_ctx_resolver
        ))
        .layer(CookieManagerLayer::new()) //Здесь добавляется промежуточный слой для управления куками (cookies) с помощью CookieManagerLayer::new(). Этот слой отвечает за обработку и управление cookies, отправляемыми клиенту и принимаемыми от него
        .fallback_service(routes_static()); //Этот вызов устанавливает статический маршрут с помощью функции routes_static() в качестве запасного сервиса (fallback service). Это означает, что если ни один из ранее определенных маршрутов не соответствует запросу, то будет использован этот статический маршрут

    // region:   ---Start server
    let listener = TcpListener::bind("127.0.0.1:8080") //Создание TCP-слушателя на порту 8080 для прослушивания входящих соединений
        .await //Ожидание создания слушателя
        .unwrap(); //Этот вызов распаковывает результат операции, возвращая содержимое Result. В случае успешной привязки к порту, возвращается сам TcpListener. В случае ошибки, вызывается panic!, что может привести к завершению программы с выводом сообщения об ошибке

    println!("->>LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap(); //Запуск сервера Axum, который обрабатывает входящие соединения с помощью определенных маршрутов переменной routes_all
    //endregion: ---Start server

    Ok(())
}

//Функция main_response_mapper используется для обработки и логирования ответов сервера
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER"); //Выводит в терминал информацию о виде запроса

    let uuid = Uuid:: new_v4(); //Создание нового UUID с помощью метода Uuid::new_v4(). UUID используется для идентификации данного запроса

    let service_error = res.extensions().get::<Error>(); //Получение ошибки службы (если она возникнет) из расширений HTTP ответа с помощью метода res.extensions().get::<Error>(). Ошибка сохраняется в переменной service_error
    let client_status_error =
        service_error.map(|se| se.client_status_and_error()); //Преобразование ошибки службы (если она возникнет) в ошибку клиента с помощью метода client_status_and_error(). Результат сохраняется в переменной client_status_error

    // Если есть ошибка клиента возникла, то создаем новый ответ
    let error_response = client_status_error //Создание переменной error_response, которая будет хранить новый ответ в случае наличия ошибки клиента
        .as_ref() //Преобразование client_status_error в ссылку на опциональное значение map(|
        .map(|(status_code, client_error)| { //Применение функции к содержимому опционального значения. Если значение присутствует, функция принимает кортеж (status_code, client_error), где status_code - код статуса ошибки клиента, а client_error - тип ошибки клиента
            let client_error_body = json!({ //Создание JSON-объекта client_error_body, содержащего информацию об ошибке клиента, включая тип ошибки и UUID запроса
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("      ->> client_error_body: {client_error_body}");


            (*status_code, Json(client_error_body)).into_response() //Создаем новый HTTP ответ, который содержит информацию об ошибке клиента в формате JSON
        });

    //Создаем строку лога
let client_error = client_status_error.unzip().1; //Извлечение второго элемента кортежа из client_status_error, который содержит тип ошибки клиента
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await; //Вызов асинхронной функции log_request, которая записывает информацию о запросе в логи. Передает в нее UUID запроса, HTTP метод, URI, контекст, ошибку службы и тип ошибки клиента. Поскольку в этом примере кода мы не используем результат выполнения этой функции, результат присваивается пустой переменной _

    println!(); //Печать пустой строки для разделения записей в терминале
    error_response.unwrap_or(res) //Возврат либо нового ответа с информацией об ошибке, либо исходного ответа, если ошибки нет. Если error_response содержит Some, то он возвращается, в противном случае возвращается исходный ответ res
}

//Создаем маршрут для обслуживания статических файлов
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./"))) /*Если в браузере указать
                                                                      127.0.0.1:8080/src/main.rs, то будет отображен весь код файла main.rs текущего проекта*/
}

//region:    ---Routes Hello

//Создает маршруты для обработки запросов с префиксом "/hello" и "/hello2/:name"
fn routes_hello() -> Router {
    Router::new() //Создает новый роутер для маршрутов
        .route("/hello", get(handler_hello)) //Добавляет маршрут для обработки GET запросов по пути "/hello", используя функцию handler_hello для обработки запроса
        .route("/hello2/:name", get(handler_hello2)) //Добавляет маршрут для обработки GET запросов по пути "/hello2/:name", где ":name" является переменным параметром маршрута. Для обработки запроса используется функция handler_hello2
}

//Структура используется для десериализации параметров запроса, которые передаются в URL. В данном случае, она предназначена для десериализации параметра "name" из запросов
#[derive(Debug, Deserialize)] //Атрибуты derive используются для автоматической реализации определенных траитов для структуры. В данном случае, структура реализует траиты Debug и Deserialize, что позволяет ей быть выводимой в отладочном формате и использоваться для десериализации данных из формата JSON или других форматов.
struct HelloParams {
    name: Option<String>, //Определяет поле name, которое является опциональной строкой (Option<String>). Это означает, что параметр "name" может быть присутствовать или отсутствовать в запросе. Если он присутствует, его значение будет храниться внутри Some, а если отсутствует, будет использовано None
}



async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello - {params:?}", "HANDLER"); //Выводит в терминал сервера информацию о том, что обработчик запроса handler_hello был вызван. Вместе с этим выводится значение параметров запроса, переданных в структуре params

    /*В вебе будет отображено то имя, которое будет передано параметром name.
    Параметром являются символы, которые находятся после знака ?, например для передачи имени Jen
    запрос должен выглядеть как {адрес сервера}/{маршрут}?{параметр} (127.0.0.1:8080/hello?name=Jen)*/
    let name = params.name.as_deref().unwrap_or("World!"); //Извлекается параметр name из структуры params. Если параметр name не определен или его значение равно None, то устанавливается значение "World!". Это значение используется для приветствия, если параметр не указан
    Html(format!("Hello, {name}")) //Формирует HTML-ответ с приветствием, где {name} заменяется значением переменной name
    //в итоге на веб странице отобразиться Hello, Jen. Но если параметр отсутствует -> Hello, World!
}


//Функция handler_hello2 аналогична handler_hello
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, {name}"))
}

/// Оба handler_hello и handler_hello2 являются обработчиками запросов, но различаются в том, как они получают параметры.
//
// handler_hello получает параметры из строки запроса (query string), который обычно выглядит так:
// ?param1=value1&param2=value2. Он использует тип Query для извлечения этих параметров из запроса.
//
// handler_hello2 получает параметры из пути URL, который может содержать переменные,
// вроде /hello2/:name.
// Использует тип Path<String> для извлечения значения переменной name из пути URL.

//endregion: ---Routes Hello

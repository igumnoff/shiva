pub use self::error::{Error, Result};

use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Json, middleware, Router};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx; //модуль контекста
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    //Инициализируем MadelController
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new() //указываем все маршруты
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper)) //Промежуточный слой. На данный момент просто добавляется в терминал результат выполнения функции main_response_mapper
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static()); //статический маршрут нужен для перенаправления в случае отсутствия динамического маршрута

    // region:   ---Start server
    let listener = TcpListener::bind("127.0.0.1:8080") //создаём сервер с прослушиванием порта 8080
        .await
        .unwrap();

    println!("->>LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap();
    //endregion: ---Start server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid:: new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // Если есть ошибка клиента, то создаем новый ответ
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("      ->> client_error_body: {client_error_body}");

            //создаём отчет для client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    println!("    ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    //создаём статический маршрут
    Router::new().nest_service("/", get_service(ServeDir::new("./"))) /*Если в браузере указать
                                                                      127.0.0.1:8080/src/main.rs, то будет отображен весь код файла main.rs текущего проекта*/
}

//region:    ---Routes Hello
fn routes_hello() -> Router {
    //группируем маршруты по типу
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello - {params:?}", "HANDLER"); //выводит в терминал сервера какой HANDLER произвёл обращение к серверу

    /*В вебе будет отображено то имя, которое будет передано параметром name.
    Параметром являются символы, которые находятся после знака ?, например для передачи имени Jen
    запрос должен выглядеть как {адрес сервера}/{маршрут}?{параметр} (127.0.0.1:8080/hello?name=Jen)*/
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello, {name}"))
    //в итоге на веб странице отобразиться Hello, Jen. Но если параметр отсутствует -> Hello, World!
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, {name}"))
}
//endregion: ---Routes Hello

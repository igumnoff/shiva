
pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use tokio::net::TcpListener;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main () {
    let routes_all = Router::new() //указываем все маршруты
        .merge(routes_hello())
        .fallback_service(routes_static()); //статический маршрут нужен для перенаправления в случае отсутствия динамического маршрута

    // region:   ---Start server
    let listener = TcpListener::bind("127.0.0.1:8080")//создаём сервер с прослушиванием порта 8080
        .await
        .unwrap();

    println!("->>LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap();
    //endregion: ---Start server
}

fn routes_static() -> Router { //создаём статический маршрут
Router::new().nest_service("/", get_service(ServeDir::new("./"))) /*Если в браузере указать
127.0.0.1:8080/src/main.rs, то будет отображен весь код файла main.rs текущего проекта*/
}

//region:    ---Routes Hello
fn routes_hello() -> Router { //группируем маршруты по типу
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
pub use self::error::Result;
use crate::web::routes_files::{handler_convert_file};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use tokio::net::TcpListener;

mod error;

mod web;



#[tokio::main]
async fn main() -> Result<()> {

    let route_test = Router::new().route("/test_server", get(handler_answer_server));

    let route_input_file = Router::new()
        .route("//upload/:output_format", post(handler_convert_file));

    let routes_all = Router::new()
        .merge(route_test)
        .merge(route_input_file)
        .layer(middleware::map_response(main_response_mapper));

    // region:    ---Start Server

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("-->>LISTENING on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: ---Start Server

    Ok(())
}

async fn handler_answer_server() -> impl IntoResponse {
    println!("-->> {:<12} - answer_server", "HANDLER");

    Html("TEST DONE")
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}
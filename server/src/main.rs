pub use self::error::Result;
use crate::web::routes_files::handler_convert_file;
use axum::extract::DefaultBodyLimit;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use clap::{Arg, Command};
use log::info;
use tokio::net::TcpListener;
use tower_http::limit::RequestBodyLimitLayer;

mod error;

mod web;

// multy thread
#[tokio::main]
async fn main() -> Result<()> {
    /*
      //Logging Settings
      env_logger::builder()
          .filter_level(log::LevelFilter::Trace)
          .init();
    */

    // Defining command line arguments
    let matches = Command::new("Server")
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Sets the host address")
                .require_equals(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the port number")
                .require_equals(true)
                .default_value("8080"),
        )
        .get_matches();

    // Extracting argument values
    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let route_test = Router::new().route("/test_server", get(handler_answer_server));

    let route_input_file = Router::new()
        .route("/transform/:output_format", post(handler_convert_file))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024)); //file size limit - 100 Mb

    let routes_all = Router::new()
        .merge(route_test)
        .merge(route_input_file)
        .layer(middleware::map_response(main_response_mapper));

    // region:    ---Start Server

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    info!("-->>LISTENING on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: ---Start Server

    Ok(())
}

async fn handler_answer_server() -> impl IntoResponse {
    info!("-->> {:<12} - answer_server", "HANDLER");

    Html("TEST DONE")
}

async fn main_response_mapper(res: Response) -> Response {
    info!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

    res
}

// Tests

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use anyhow::Error;
    use anyhow::Result;
    use reqwest::multipart;
    use reqwest::Body;
    use std::fs;
    use std::io::Cursor;
    use std::io::Write;
    use tokio::fs::File;
    use log::info;

    #[tokio::test]
    async fn test_server() -> Result<()> {
        let hc = httpc_test::new_client("http://localhost:8080")?;

        hc.do_get("/test_server").await?.print().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_convert_file_md_html_txt() -> Result<(), Error> {
        /*
           //Logging Settings
           env_logger::builder()
               .filter_level(log::LevelFilter::Trace)
               .init();
        */

        //We form all combinations of incoming and outgoing file formats
        let input_formats = vec!["md", "html", "txt"];
        let output_formats = vec!["md", "html", "txt", "pdf", "json", "rtf", "docx"];

        // We go through all the combinations
        for input_format in &input_formats {
            for output_format in &output_formats {
                // Creating a temporary file with test data
                let file_data = " Test file data";
                let file_name = format!("test_file.{}", input_format);
                let mut file = Cursor::new(Vec::new());
                file.write_all(file_data.as_bytes())?;

                // Creating the request body from the contents of the file
                let body = Body::from(file.get_ref().to_vec());

                // Creating a multipart part with a file
                let part = multipart::Part::stream(body)
                    .file_name(file_name)
                    .mime_str(match output_format {
                        &"md" => "text/markdown",
                        &"html" | &"htm" => "text/html",
                        &"txt" => "text/plain",
                        &"pdf" => "application/pdf",
                        &"json" => "application/json",
                        &"rtf" => "application/rtf",
                        &"docx" => "application/docx",
                        _ => return Err(anyhow!("Invalid output format: {}", output_format)),
                    })?;

                // Creating a multipart/form-data with a file and additional data
                let form = multipart::Form::new()
                    .part("file", part)
                    .text("output_format", output_format.to_string());

                // Creating HTTP-client
                let client = reqwest::Client::new();

                info!("sending the test_file.{}", input_format);

                // Sending a POST request to the server with the multipart form
                let mut response = client
                    .post(&format!(
                        "http://localhost:8080/transform/{}",
                        output_format
                    ))
                    .multipart(form)
                    .send()
                    .await
                    .unwrap();

                // Checking the server response
                assert_eq!(response.status(), reqwest::StatusCode::OK);

                if response.status().is_success() {
                    let file_name_2 = format!("test_file.{}", output_format);
                    let mut file = File::create(file_name_2).await?;
                    use tokio::io::AsyncWriteExt;

                    while let Some(chunk) = response.chunk().await? {
                        file.write_all(&chunk).await?;
                    }
                }

                info!(
                    "the file has been successfully converted to the format {}",
                    output_format
                )
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_convert_file_pdf() -> Result<(), Box<dyn std::error::Error>> {
        /*
          //Logging Settings
          env_logger::builder()
              .filter_level(log::LevelFilter::Trace)
              .init();
        */

        // Creating a temporary file with test data
        let file_data = "Test file data";
        let file_name = "test_file.md";
        let mut file = Cursor::new(Vec::new());
        file.write_all(file_data.as_bytes())?;

        // Creating the request body from the contents of the file
        let body = Body::from(file.get_ref().to_vec());

        // Creating a multipart part with a file
        let part = multipart::Part::stream(body)
            .file_name(file_name)
            .mime_str("application/md")?;

        // Creating a multipart/form-data with a file and additional data
        let form = multipart::Form::new()
            .part("file", part)
            .text("output_format", "pdf");

        // Creating HTTP-client
        let client = reqwest::Client::new();

        // Sending a POST request to the server with the multipart form
        let response = client
            .post("http://localhost:8080/transform/pdf")
            .multipart(form)
            .send()
            .await?;

        // Checking the server response
        assert_eq!(response.status(), reqwest::StatusCode::OK);

        // We get the contents of the response in the form bytes
        let pdf_content = response.bytes().await?;

        // We form all combinations of outgoing formats
        let output_formats = vec!["md", "html", "txt", "pdf", "json"];

        // We go through all the combinations
        for output_format in &output_formats {
            // Creating the request body from the contents of the file
            let body = Body::from(pdf_content.clone());

            // Creating a multipart part with a file
            let part = multipart::Part::stream(body)
                .file_name(".pdf")
                .mime_str("application/pdf")?;

            // Creating a multipart/form-data with a file and additional data
            let form = multipart::Form::new()
                .part("file", part)
                .text("output_format", output_format.to_string());

            // Creating HTTP-client
            let client = reqwest::Client::new();

            info!("sending the test_file.pdf");

            // Sending a POST request to the server with the multipart form
            let response = client
                .post(&format!(
                    "http://localhost:8080/transform/{}",
                    output_format
                ))
                .multipart(form)
                .send()
                .await?;

            // Checking the server response
            assert_eq!(response.status(), reqwest::StatusCode::OK);

            info!(
                "the file has been successfully converted to the format {}",
                output_format
            )
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_handler_convert_file_json() -> Result<(), Box<dyn std::error::Error>> {
        /*
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .init();

        */

        let file_data = "Test file data";
        let file_name = "test_file.md";
        let mut file = Cursor::new(Vec::new());
        file.write_all(file_data.as_bytes())?;

        let body = Body::from(file.get_ref().to_vec());

        let part = multipart::Part::stream(body)
            .file_name(file_name)
            .mime_str("application/md")?;

        let form = multipart::Form::new()
            .part("file", part)
            .text("output_format", "json");

        let client = reqwest::Client::new();

        let response = client
            .post("http://localhost:8080/transform/json")
            .multipart(form)
            .send()
            .await?;

        assert_eq!(response.status(), reqwest::StatusCode::OK);

        let pdf_content = response.bytes().await?;

        let output_formats = vec!["md", "html", "txt", "pdf", "json", "rtf"];

        for output_format in &output_formats {
            let body = Body::from(pdf_content.clone());

            let part = multipart::Part::stream(body)
                .file_name(".json")
                .mime_str("application/json")?;

            let form = multipart::Form::new()
                .part("file", part)
                .text("output_format", output_format.to_string());

            let client = reqwest::Client::new();

            info!("sending the test_file.json");

            let response = client
                .post(&format!(
                    "http://localhost:8080/transform/{}",
                    output_format
                ))
                .multipart(form)
                .send()
                .await?;

            assert_eq!(response.status(), reqwest::StatusCode::OK);

            info!(
                "the file has been successfully converted to the format {}",
                output_format
            )
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_upload_zip() -> Result<(), Box<dyn std::error::Error>> {
        info!("start test_upload_zip");
        /*
                env_logger::builder()
                    .filter_level(log::LevelFilter::Trace)
                    .init();
        */
        // We form all combinations of outgoing formats
        let output_formats = vec!["md", "txt", "pdf", "json", "rtf"];

        for output_format in &output_formats {
            // Creating HTTP-client
            let client = reqwest::Client::new();

            // The path of the test file to send to the server
            let file_path = "./test.zip";
            let file_name = "test.zip";
            let file_content = fs::read(file_path)?;

            // Create multipart part for the file
            let file_part = multipart::Part::bytes(file_content)
                .file_name(file_name.to_string())
                .mime_str("application/zip")?;

            // Create multipart form
            let form = multipart::Form::new()
                .part("file", file_part)
                .text("output_format", output_format.to_string());

            let mut response = client
                .post(&format!(
                    "http://localhost:8080/transform/{}",
                    output_format
                ))
                .multipart(form)
                .send()
                .await?;

            assert_eq!(response.status(), reqwest::StatusCode::OK);

            if response.status().is_success() {
                let file_name_2 = format!("unzip_test_file.{}", output_format);
                let mut file = File::create(file_name_2).await?;
                use tokio::io::AsyncWriteExt;

                while let Some(chunk) = response.chunk().await? {
                    file.write_all(&chunk).await?;
                }
            }
        }

        Ok(())
    }
}

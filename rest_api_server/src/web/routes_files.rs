use std::collections::HashMap;
use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::routing::{get, post};
use axum::{Router};
use axum::body::Bytes;
use shiva::core::{Document, TransformerTrait};
use clap::{Parser, ValueEnum};
use axum::body::Body;
use axum::response::{IntoResponse, Response};


pub fn route_input_file() -> Router {
    Router::new()
        .route("/upload/:output_format", post(handler_input_file))
}


async fn handler_input_file(
    multipart: Multipart,
    Path(output_format): Path<&str>,
) -> impl IntoResponse {
    println!("-->> {:<12} - handler_input_file - output_extension_", "HANDLER");

    let data_uploaded_file = upload_file(multipart).await.unwrap();

    build_file_response(data_uploaded_file.file_name,
                        data_uploaded_file.file_extension,
                        data_uploaded_file.file_data,
                        output_format,
    ).await
}

async fn upload_file(mut multipart: Multipart) -> Result<InputFileInfo, Response> {
    let mut fail_name = None;
    let mut fail_extension = None;
    let mut file_data = Bytes::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().unwrap_or("").to_string();

        if name == "file" {
            fail_name = Some(filename.clone());
            fail_extension = filename
                .split(".")
                .last()
                .map(|ext| ext.to_lowercase())
                .filter(|ext| !ext.trim().is_empty())
                .map(String::from);

            if let Some(ref ext) =fail_extension {
                if is_supported_format(ext) {
                    file_data = field.bytes().await.unwrap();
                } else {
                    build_error_response(
                        format!("Unsupported file format: {}", ext).to_string(),
                        StatusCode::BAD_REQUEST).await;
                }
            } else {
                build_error_response(
                    "File has no extension".to_string(),
                    StatusCode::BAD_REQUEST).await;
            }
        }
    }
    let file_name = fail_name.unwrap_or("Convert_shiva".to_string());
    let file_extension = fail_extension.ok_or("File extension not found").unwrap();
    let file_data = file_data;

    Ok(InputFileInfo {
        file_name,
        file_extension,
        file_data,
    })
}


#[derive(Debug)]
struct InputFileInfo {
    file_name: String,
    file_extension: String,
    file_data: Bytes,
}

fn is_supported_format(file_extension: &str) -> bool {
    match file_extension {
        "md" | "html" |"htm" | "txt" | "pdf" => true,

        _ => false,
    }
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Format {
    Markdown,
    Html,
    Text,
    Pdf,
}


async fn build_file_response(filename: String,
                             extension: String,
                             input_file_bytes: Bytes,
                             output_format: &str,
) -> Response {
    let document = match extension.as_str() {
        "md" => {
            let md = shiva::markdown::Transformer::parse(
                &input_file_bytes,
                &HashMap::new(),
            ).unwrap();
            Document::from(md)
        }
        "html" | "htm" => {
            let html = shiva::html::Transformer::parse(
                &input_file_bytes,
                &HashMap::new(),
            ).unwrap();
            Document::from(html)
        }
        "txt" => {
            let text = shiva::text::Transformer::parse(
                &input_file_bytes,
                &HashMap::new(),
            ).unwrap();
            Document::from(text)
        }
        "pdf" => {
            let pdf = shiva::pdf::Transformer::parse(
                &input_file_bytes,
                &HashMap::new(),
            ).unwrap();
            Document::from(pdf)
        }

        _ => return build_error_response(
            "Unsupported input file format".to_string(),
            StatusCode::BAD_REQUEST).await,
    };
    let output = match output_format {
        "md" => {
            let _md = shiva::markdown::Transformer::generate(&document)
                .unwrap();
        }
        "html" | "htm" => {
            let _html = shiva::html::Transformer::generate(&document)
                .unwrap();
        }
        "txt" => {
            let _txt = shiva::text::Transformer::generate(&document)
                .unwrap();
        }
        "pdf" => {
            let _pdf = shiva::pdf::Transformer::generate(&document)
                .unwrap();
        }
        _ => return build_error_response(
            "Unsupported output file format".to_string(),
            StatusCode::BAD_REQUEST).await,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(format!("attachment: filename=\"{}.{}\"", filename, output_format)
            .as_str())
            .unwrap());
    headers.insert(
        "Content-Type",
        match output_format {
            "md" => HeaderValue::from_static("text/markdown"),
            "html" | "htm" => HeaderValue::from_static("text/html"),
            "txt" => HeaderValue::from_static("text/plain"),
            "pdf" => HeaderValue::from_static("application/pdf"),
            _ => HeaderValue::from_static("application/octet-stream"),
        },
    );


    Response::builder()
        .status(StatusCode::OK)
        .headers(headers)
        .body(Body::from(output))
        .unwrap()

}


async fn build_error_response(message: String, status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .body(Body::from(message))
        .unwrap()
}

use std::collections::HashMap;
use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::body::{Body, Bytes};
use axum::response::Response;
use shiva::core::{Document, TransformerTrait};
use clap::{Parser, ValueEnum};
use serde::Serialize;
use crate::error::Error;
use crate::Result;

struct ConvertFile {
    file: (Bytes, HashMap<String, Bytes>),
}

#[derive(Debug, Clone, Serialize)]
struct InputFileInfo {
    file_name: String,
    file_extension: String,
    file_data: Bytes,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Format {
    Markdown,
    Html,
    Text,
    Pdf,
}


pub async fn handler_convert_file(
    Path(output_format): Path<String>,
    multipart: Multipart,
) -> Result<Response<Body>> {
    println!("-->> {:<12} - handler_input_file - output_extension_", "HANDLER");

    let data_uploaded_file = upload_file(multipart).await?;

    let build_file_response = convert_file(
        data_uploaded_file.file_name.clone(),
        data_uploaded_file.file_extension,
        data_uploaded_file.file_data,
        output_format.clone(),
    ).await?;

    let filename = data_uploaded_file.file_name;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Disposition", format!("attachment; filename=\"{}.{}\"", filename, output_format))
        .header("Content-Type", match output_format.as_str() {
            "md" => "text/markdown",
            "html" | "htm" => "text/html",
            "txt" => "text/plain",
            "pdf" => "application/pdf",
            "json" => "application/json",
            _ => "application/octet-stream",
        })
        .body(Body::from(build_file_response.file.0)).unwrap())
}

async fn upload_file(mut multipart: Multipart) -> Result<InputFileInfo> {
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
                    return Err(Error::FailBytes)
                }
            } else {
                return Err(Error::UnsupportedFormat)
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




fn is_supported_format(file_extension: &str) -> bool {
    match file_extension {
        "md" | "html" |"htm" | "txt" | "pdf" => true,

        _ => false,
    }
}


async fn convert_file(
    filename: String,
    extension: String,
    input_file_bytes: Bytes,
    output_format: String,
) -> Result<ConvertFile> {

    // region:    ---Create document
    let document = match extension.as_str() {
        "md" => Document::from(
            shiva::markdown::Transformer::parse(&input_file_bytes, &HashMap::new()).unwrap()
        ),
        "html" | "htm" => Document::from(
            shiva::html::Transformer::parse(&input_file_bytes, &HashMap::new()).unwrap()
        ),
        "txt" => Document::from(
            shiva::text::Transformer::parse(&input_file_bytes, &HashMap::new()).unwrap()
        ),
        "pdf" => Document::from(
            shiva::pdf::Transformer::parse(&input_file_bytes, &HashMap::new()).unwrap()
        ),
        "json" => Document::from(
            shiva::json::Transformer::parse(&input_file_bytes, &HashMap::new()).unwrap()
        ),
        _ => return Err(Error::FailParseDocument),
    };
    // endregion: ---Create document

    // region:    ---Convert file
    let output_bytes = match output_format.as_str() {
        "md" => shiva::markdown::Transformer::generate(&document).unwrap(),
        "html" | "htm" => shiva::html::Transformer::generate(&document).unwrap(),
        "txt" => shiva::text::Transformer::generate(&document).unwrap(),
        "pdf" => shiva::pdf::Transformer::generate(&document).unwrap(),
        "json" => shiva::json::Transformer::generate(&document).unwrap(),
        _ => return Err(Error::FailConvertFile),
    };
    // endregion: ---Convert file

    Ok(ConvertFile {
        file: output_bytes,
    })

}
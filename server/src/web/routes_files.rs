use crate::error::{Error, Result};
use axum::body::Bytes;
use axum::extract::multipart::Field;
use axum::extract::{Multipart, Path};
use axum::response::{IntoResponse, Response};
use futures_util::StreamExt;
use log::{error, info};
use serde::Serialize;
use shiva::core::{Document, TransformerTrait, TransformerWithImageLoaderSaverTrait};
use std::collections::HashMap;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, Serialize)]
struct UploadFileInfo {
    upload_file_name: String,
    upload_file_extension: String,
    upload_file_data: Bytes,
}

#[derive(Debug, Clone)]
struct DownloadFile {
    file_name: String,
    file_data: Bytes,
}

#[derive(Debug, Clone)]
struct UploadFileZip {
    file_name: String,
    file_data: Bytes,
    file_extension: String,
    images: HashMap<String, Bytes>,
}

enum StructUploadFile {
    UploadFile(UploadFileInfo),
    UploadZip(UploadFileZip),
}

impl IntoResponse for DownloadFile {
    fn into_response(self) -> Response {
        use axum::http::HeaderValue;

        let mut res = self.file_data.into_response();
        res.headers_mut().insert(
            "Content-Disposition",
            HeaderValue::from_bytes(self.file_name.as_bytes()).unwrap(),
        );

        res
    }
}

pub async fn handler_convert_file(
    Path(output_format): Path<String>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    match upload_file(multipart).await {
        //depending on the returned structure, we execute the code
        Ok(data_upload_file) => match data_upload_file {
            StructUploadFile::UploadFile(upload_file_info) => {
                let input_extension = upload_file_info.upload_file_extension.clone();
                info!("-->> {:<12} - handler_convert_file input_extension_{input_extension}- output_extension_{output_format}", "HANDLER");

                let build_response_file = convert_file(
                    upload_file_info.upload_file_name,
                    upload_file_info.upload_file_extension,
                    upload_file_info.upload_file_data,
                    output_format,
                )
                .await
                .map_err(|_| Error::FailConvertFile)?;

                Ok(build_response_file)
            }
            StructUploadFile::UploadZip(upload_file_zip) => {
                info!("-->> {:<12} - handler_convert_file input ZIP archive - output_extension_{output_format}", "HANDLER");

                let build_response_file = convert_file_zip(
                    upload_file_zip.file_name,
                    upload_file_zip.file_data,
                    upload_file_zip.file_extension,
                    upload_file_zip.images,
                    output_format,
                )
                .await
                .map_err(|_| Error::FailConvertFile)?;

                Ok(build_response_file)
            }
        },
        Err(e) => Err(e),
    }
}

async fn convert_file_zip(
    file_name: String,
    input_file_data_bytes: Bytes,
    file_extension: String,
    images: HashMap<String, Bytes>,
    output_format: String,
) -> Result<DownloadFile> {
    /*
    info!("upload file name: {}", file_name);
    info!("upload file format: {}", file_extension);
    info!("download file format: {}", output_format);
    info!("Image files from the archive");
    for image_name in images.keys() {
        info!("- {:#?}", image_name);
    }
     */

    let document = match file_extension.as_str() {
        "md" => Document::from(
            shiva::markdown::Transformer::parse_with_loader(
                &input_file_data_bytes,
                memory_image_loader(images),
            )
            .unwrap(),
        ),
        "html" | "htm" => Document::from(
            shiva::html::Transformer::parse_with_loader(
                &input_file_data_bytes,
                memory_image_loader(images),
            )
            .unwrap(),
        ),
        _ => return Err(Error::FailParseDocument),
    };

    let output_bytes = match output_format.as_str() {
        "md" => shiva::markdown::Transformer::generate(&document).unwrap(),
        "html" | "htm" => shiva::html::Transformer::generate(&document).unwrap(),
        "txt" => shiva::text::Transformer::generate(&document).unwrap(),
        "pdf" => shiva::pdf::Transformer::generate(&document).unwrap(),
        "json" => shiva::json::Transformer::generate(&document).unwrap(),
        "rtf" => shiva::rtf::Transformer::generate(&document).unwrap(),
        "docx" => shiva::docx::Transformer::generate(&document).unwrap(),
        _ => return Err(Error::FailConvertFile),
    };

    Ok(DownloadFile {
        file_name,
        file_data: output_bytes,
    })
}

fn memory_image_loader(images: HashMap<String, Bytes>) -> impl Fn(&str) -> anyhow::Result<Bytes> {
    let image_loader = move |image: &str| -> anyhow::Result<Bytes> {
        let bytes = images
            .get(image)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No image: {}", image))?;
        Ok(Bytes::from(bytes))
    };
    image_loader
}

//checking the supported formats in the archive
fn supported_extensions_in_archive(file_extension: &str) -> bool {
    match file_extension {
        "md" | "html" | "htm" | "png" => true,

        _ => false,
    }
}

//unpacking the archive
async fn unpacking(mut field: Field<'_>) -> Result<StructUploadFile> {
    // Creating variables to store archive file data
    let mut file_name = None;
    let mut file_data = None;
    let mut file_extension = None;
    let mut images = HashMap::new();
    let mut found_supported_file = false;

    //reading the contents of the archive
    let mut file_content = Vec::new();
    while let Some(chunk) = field.next().await {
        match chunk {
            Ok(data) => {
                file_content.extend_from_slice(&data);
            }
            Err(e) => {
                error!("Error reading chunk: {:?}", e);
                return Err(Error::FailBytes);
            }
        }
    }

    //creating a cursor to read the archive
    let reader = Cursor::new(file_content);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    //check the extension of each archive file
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        //defining the file name
        let file_name_in_archive = file
            .name()
            .split(".")
            .next()
            .map(|name_file_in_archive| name_file_in_archive.to_lowercase())
            .filter(|name_file_in_archive| !name_file_in_archive.trim().is_empty())
            .map(String::from);

        //we define the extension of each file
        let file_extension_in_archive = file
            .name()
            .split(".")
            .last()
            .map(|ext| ext.to_lowercase())
            .filter(|ext| !ext.trim().is_empty())
            .map(String::from);

        //info!("in ZIP {}.{}", file_name_in_archive.clone().unwrap(), file_extension_in_archive.clone().unwrap());

        //checking the supported format
        if let Some(ref ext) = file_extension_in_archive {
            if supported_extensions_in_archive(&ext) {
                found_supported_file = true;
                let mut file_data_buf = Vec::new();
                file.read_to_end(&mut file_data_buf).unwrap();
                match ext.as_str() {
                    "html" | "htm" | "md" => {
                        file_name = file_name_in_archive;
                        file_data = Some(Bytes::from(file_data_buf));
                        file_extension = file_extension_in_archive.clone();
                    }
                    "png" => {
                        let image_name = file.name().to_string();

                        images.insert(image_name, Bytes::from(file_data_buf));
                    }
                    _ => {
                        return Err(Error::NoFilesToConvertInZip);
                    }
                }
            }
        }
    }

    // If no supported file was found, return an error
    if !found_supported_file {
        return Err(Error::NoFilesToConvertInZip);
    }

    // building the Upload File Zip structure
    let upload_file_zip = UploadFileZip {
        file_name: file_name.unwrap_or("Shiva_convert".to_string()),
        file_data: file_data.ok_or_else(|| Error::FailBytes)?,
        file_extension: file_extension.ok_or_else(|| Error::ExtensionMissing)?,
        images,
    };

    Ok(StructUploadFile::UploadZip(upload_file_zip))
}

async fn upload_file(mut multipart: Multipart) -> Result<StructUploadFile> {
    //create variables in which we will then write the name, file extension and the file itself in binary form
    let mut file_name = None;
    let mut file_extension = None;
    let mut file_data = Bytes::new();

    //defining the file type and its name
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().unwrap_or("").to_string();

        //if the file has the file parameter, then
        if name == "file" {
            //defining the file name without the extension
            file_name = filename
                .split(".")
                .next()
                .map(|upload_name| upload_name.to_lowercase())
                .filter(|upload_name| !upload_name.trim().is_empty())
                .map(String::from);

            //defining the file extension
            file_extension = filename
                .split(".")
                .last()
                .map(|ext| ext.to_lowercase())
                .filter(|ext| !ext.trim().is_empty())
                .map(String::from);

            //matching the file extension
            if let Some(ref ext) = file_extension {
                match ext.as_str() {
                    "zip" => {
                        return unpacking(field).await; //if _zip, start unpacking
                    }

                    _ => {
                        //if not zip, check the supported extension
                        if supported_format(ext).await {
                            file_data = field.bytes().await.unwrap();
                        } else {
                            return Err(Error::UnsupportedFormat);
                        }
                    }
                }
            } else {
                return Err(Error::ExtensionMissing);
            }
        }
    }

    //writing the received data to variables
    let file_name = file_name.unwrap_or("Shiva_convert".to_string());
    let file_extension = file_extension.ok_or("File extension not found").unwrap();
    let file_data = file_data;

    //creating the uploadFile Info structure
    Ok(StructUploadFile::UploadFile(UploadFileInfo {
        upload_file_name: file_name,
        upload_file_extension: file_extension,
        upload_file_data: file_data,
    }))
}

async fn convert_file(
    file_name: String,
    file_extension: String,
    input_file_data_bytes: Bytes,
    output_format: String,
) -> Result<DownloadFile> {
    //info!("upload file name: {}", file_name);
    //info!("upload file format: {}", file_extension);
    //info!("download file format: {}", output_format);

    let document = match file_extension.as_str() {
        "md" => {
            Document::from(shiva::markdown::Transformer::parse(&input_file_data_bytes).unwrap())
        }
        "html" | "htm" => {
            Document::from(shiva::html::Transformer::parse(&input_file_data_bytes).unwrap())
        }
        "txt" => Document::from(shiva::text::Transformer::parse(&input_file_data_bytes).unwrap()),
        "pdf" => Document::from(shiva::pdf::Transformer::parse(&input_file_data_bytes).unwrap()),
        "json" => Document::from(shiva::json::Transformer::parse(&input_file_data_bytes).unwrap()),
        _ => return Err(Error::FailParseDocument),
    };

    let output_bytes = match output_format.as_str() {
        "md" => shiva::markdown::Transformer::generate(&document).unwrap(),
        "html" | "htm" => shiva::html::Transformer::generate(&document).unwrap(),
        "txt" => shiva::text::Transformer::generate(&document).unwrap(),
        "pdf" => shiva::pdf::Transformer::generate(&document).unwrap(),
        "json" => shiva::json::Transformer::generate(&document).unwrap(),
        "rtf" => shiva::rtf::Transformer::generate(&document).unwrap(),
        "docx" => shiva::docx::Transformer::generate(&document).unwrap(),
        _ => return Err(Error::FailConvertFile),
    };

    Ok(DownloadFile {
        file_name,
        file_data: output_bytes,
    })
}

async fn supported_format(file_extension: &str) -> bool {
    match file_extension {
        "md" | "html" | "htm" | "txt" | "pdf" | "json" => true,

        _ => false,
    }
}

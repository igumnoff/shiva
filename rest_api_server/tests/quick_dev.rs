use std::fs::File;
use std::io::{Cursor, Read, Write};
use anyhow::{anyhow, Error, Result};
use reqwest::{Body, multipart};


#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/test_server").await?.print().await?;

    Ok(())
}


#[tokio::test]
async fn test_handler_convert_file_1() -> Result<(), Error> {
/*
    // Настройка логирования
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

 */

    // Формируем все комбинации входящих и исходящих форматов
    let input_formats = vec!["md", "html", "txt"];
    let output_formats = vec!["md", "html", "txt", "pdf", "json"];

    //Проходим по всем комбинациям
    for input_format in &input_formats {
        for output_format in &output_formats {
            // Создаем временный файл с тестовыми данными
            let file_data = "Test file data";
            let file_name = format!("test_file.{}", input_format);
            let mut file = Cursor::new(Vec::new());
            file.write_all(file_data.as_bytes())?;

            // Создаем тело запроса из содержимого файла
            let body = Body::from(file.get_ref().to_vec());

            // Создаем часть multipart с файлом
            let part = multipart::Part::stream(body)
                .file_name(file_name)
                .mime_str(match output_format {
                    &"md" => "text/markdown",
                    &"html" | &"htm" => "text/html",
                    &"txt" => "text/plain",
                    &"pdf" => "application/pdf",
                    &"json" => "application/json",
                    _ => return Err(anyhow!("Invalid output format: {}", output_format)),
                })?;

            // Создаем multipart/form-data с файлом и дополнительными данными
            let form = multipart::Form::new()
                .part("file", part)
                .text("output_format", "txt"); // Здесь указываем желаемый формат конвертации

            // Создаем HTTP-клиента
            let client = reqwest::Client::new();

            // Отправляем POST-запрос на сервер с формой multipart

            println!("отправляем test_file.{}", input_format);

            let response = client
                .post(&format!("http://localhost:8080/upload/{}", output_format)) // Здесь указываем желаемый формат вывода (например, md)
                .multipart(form)
                .send()
                .await.unwrap();

            // Проверяем ответ сервера
            assert_eq!(response.status(), reqwest::StatusCode::OK);

            println!("файл успешно конвертирован в формат {}", output_format)
        }
    }

    Ok(())
}



/*
#[tokio::test]
async fn test_handler_convert_file_2() -> Result<(), Box<dyn std::error::Error>> {

    // Настройка логирования
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    // Создаем PDF
    let (doc, page1, layer1) = PdfDocument::new("test file", Mm(297.0), Mm(210.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    current_layer.use_text("Test file data", 12.0, Mm(100.0), Mm(100.0), &font);

    // Записываем PDF в файл
    let file_name = "test_file.pdf";
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;


    // Читаем содержимое файла в память
    let mut file = File::open(file_name)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Создаем тело запроса из содержимого файла
    let body = Body::from(file_content);

    // Создаем часть multipart с файлом
    let part = multipart::Part::stream(body)
        .file_name(file_name)
        .mime_str("application/pdf")?;

    // Создаем multipart/form-data с файлом и дополнительными данными
    let form = multipart::Form::new()
        .part("file", part)
        .text("output_format", "txt"); // Здесь указываем желаемый формат конвертации

    // Создаем HTTP-клиента
    let client = reqwest::Client::new();

    // Отправляем POST-запрос на сервер с формой multipart
    let response = client
        .post("http://localhost:8080/upload/txt") // Здесь указываем желаемый формат вывода (например, md)
        .multipart(form)
        .send()
        .await?;

    // Проверяем ответ сервера
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    Ok(())
}

 */
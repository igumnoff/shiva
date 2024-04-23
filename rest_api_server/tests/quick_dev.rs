use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?; //в терминале отобразиться Response body

    hc.do_get("/hello2/Mike").await?.print().await?;

    //результат запроса /src/main.rs не отобразиться в терминале сервера, потому что не назначен HANDLER по примеру handler_hello
    hc.do_get("/src/main.rs").await?.print_no_body().await?; //в терминале не отобразиться Response body

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?; //предоставление регистрационных данных (если закоментить то получу ошибку AuthFailNoAuthTokenCookie)

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    //hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}

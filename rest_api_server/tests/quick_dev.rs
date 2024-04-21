use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print_no_body().await?;

    hc.do_get("/hello2/Mike").await?.print_no_body().await?;

    hc.do_get("/src/main.rs").await?.print_no_body().await?;

    Ok(())
}
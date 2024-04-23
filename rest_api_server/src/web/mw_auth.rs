use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //парсинг и проверка cookies
    let (user_id, exp, sing) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    //TODO: проверка компонентов токена

    Ok(next.run(req).await)
}

///Парсинг (анализ) токена, его времени действия и подписи
//Return (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sing) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sing.to_string()))
}

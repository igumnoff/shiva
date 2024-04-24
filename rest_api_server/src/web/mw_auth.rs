use async_trait::async_trait;
use crate::{Error, Result};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("-->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx  = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sing)) => {
            //TODO: проверка компонентов токена
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    //очистка файла куки
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
    {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    //сохранение ctx_result в расширении запросов
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}


// region:    ---Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) -> Result<Self> {
        println!("-->> {:<12} -Ctx", "EXTRACTOR");

       parts
           .extensions
           .get::<Result<Ctx>>()
           .ok_or(Error::AuthFailCtxNotInRequestExt)?
           .clone()
    }
}

// endregion: ---Ctx Extractor

///Парсинг (анализ) токена, его времени действия и подписи
//Return (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sing) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    )
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sing.to_string()))
}

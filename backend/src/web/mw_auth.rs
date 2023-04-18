use crate::{web::AUTH_TOKEN, Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|v| v.value().to_string());

    auth_token.ok_or(Error::AuthFail)?;
    Ok(next.run(req).await)
}

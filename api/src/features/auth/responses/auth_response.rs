use axum::http;
use axum::response;
use axum_extra::extract::cookie;

#[derive(Debug, Clone)]
pub struct AuthResponse<Tokens> {
    tokens: Tokens,
    refresh_cookie: cookie::Cookie<'static>,
}

impl<Tokens> AuthResponse<Tokens> {
    pub fn new(tokens: Tokens, refresh_token: &str, max_age: time::Duration) -> Self {
        let refresh_cookie = cookie::Cookie::build(("refresh_token", refresh_token.to_string()))
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .path("/api/auth")
            .max_age(max_age)
            .build();

        Self {
            tokens,
            refresh_cookie,
        }
    }
}

impl<Tokens> response::IntoResponse for AuthResponse<Tokens>
where
    Tokens: serde::Serialize,
{
    fn into_response(self) -> response::Response {
        let mut response = axum::Json(self.tokens).into_response();

        response.headers_mut().insert(
            http::header::CACHE_CONTROL,
            http::HeaderValue::from_static("no-store"),
        );

        let cookie = self.refresh_cookie.to_string();

        response.headers_mut().append(
            http::header::SET_COOKIE,
            http::HeaderValue::from_str(&cookie).expect("valid cookie header"),
        );

        response
    }
}

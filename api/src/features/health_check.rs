#[utoipa::path(get, path = "/health-check", security(()))]
pub async fn ok() -> &'static str {
    "OK"
}

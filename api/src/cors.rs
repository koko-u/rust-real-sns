use axum::http;
use tower_http::cors;

use crate::config;

pub fn cors_layer(config: &config::Config) -> Result<cors::CorsLayer, http::header::InvalidHeaderValue> {
    let allow_origin = config.allow_origins()?;
    let layer = cors::CorsLayer::new()
        .allow_credentials(true)
        .allow_headers([
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
            http::header::ACCEPT,
        ])
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ])
        .allow_origin(allow_origin);

    Ok(layer)
}

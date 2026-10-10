use axum::response;
use serde_json::json;

use crate::openapi;

#[cfg(feature = "api-doc")]
pub async fn scalar() -> response::Html<String> {
    use utoipa::OpenApi as _;

    let config = json!({
        "content": openapi::ApiDoc::openapi(),
        "orderRequiredPropertiesFirst": false,
        "orderSchemaPropertiesBy": "preserve",
        "theme": "deepSpace",
    });

    response::Html(scalar_api_reference::scalar_html_default(&config))
}

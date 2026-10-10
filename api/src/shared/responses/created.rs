use axum::http;
use axum::response;
use axum::response::Response;

#[derive(Debug, Clone)]
pub struct Created<T> {
    location: String,
    body: T,
}

pub fn created<T>(location: impl Into<String>, body: T) -> Created<T> {
    Created {
        location: location.into(),
        body,
    }
}

impl<T> response::IntoResponse for Created<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        (
            http::StatusCode::CREATED,
            [
                (http::header::LOCATION, self.location),
                (http::header::CONTENT_TYPE, "application/json".into()),
            ],
            axum::Json(self.body),
        )
            .into_response()
    }
}

use axum::http;
use axum::response;

use crate::errors;
use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ApiError {
    #[display("Unauthorized error")]
    #[from(skip)]
    Unauthorized { message: String },
    #[display("Server error: {}", _0)]
    Server(errors::ServerError),
}

impl ApiError {
    pub fn unauthorized(message: &str) -> Self {
        Self::Unauthorized {
            message: message.to_string(),
        }
    }
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        match self {
            Self::Unauthorized { message } => {
                tracing::warn!("Unauthorized error: {message}");
                let problem_details =
                    responses::ProblemDetails::simple("Unauthorized", http::StatusCode::UNAUTHORIZED);
                (http::StatusCode::UNAUTHORIZED, axum::Json(problem_details)).into_response()
            }
            Self::Server(server_error) => server_error.into_response(),
        }
    }
}

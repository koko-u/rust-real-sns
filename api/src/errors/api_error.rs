use axum::http;
use axum::response;

use crate::errors;
use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ApiError {
    #[display("Not found: {}", message)]
    #[from(skip)]
    NotFound { message: String },
    #[display("Unauthorized error")]
    #[from(skip)]
    Unauthorized { message: String },
    #[display("Server error: {}", _0)]
    Server(errors::ServerError),
}

impl ApiError {
    pub fn not_found(message: &str) -> Self {
        Self::NotFound {
            message: message.to_string(),
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::Unauthorized {
            message: message.to_string(),
        }
    }
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        match self {
            Self::NotFound { message } => {
                tracing::info!("Not found: {message}");
                let problem_details =
                    responses::ProblemDetails::simple("Not Found", http::StatusCode::NOT_FOUND);
                (http::StatusCode::NOT_FOUND, axum::Json(problem_details)).into_response()
            }
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

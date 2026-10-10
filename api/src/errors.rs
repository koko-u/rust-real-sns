use axum::http;
use axum::response;

use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ApiError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
    #[display("Not found: {}", message)]
    NotFound { message: String },
    #[display("Unauthorized error")]
    Unauthorized,
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ValidationError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        match &self {
            Self::Database(err) => {
                tracing::error!("Database error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Something went wrong",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Validation(err) => {
                tracing::warn!("Validation error: {err:?}");
                let problem_details = responses::ProblemDetails::from(err);
                (problem_details.status, axum::Json(problem_details)).into_response()
            }
            Self::NotFound { message } => {
                tracing::info!("Not found: {message}");

                (http::StatusCode::NOT_FOUND, axum::Json(self.to_string())).into_response()
            }
            Self::Unauthorized => {
                tracing::warn!("Unauthorized error");
                let problem_details =
                    responses::ProblemDetails::simple("Unauthorized", http::StatusCode::UNAUTHORIZED);
                (http::StatusCode::UNAUTHORIZED, axum::Json(problem_details)).into_response()
            }
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::Database(err) => Self::Database(err),
            ValidationError::Validation(err) => Self::Validation(err),
        }
    }
}

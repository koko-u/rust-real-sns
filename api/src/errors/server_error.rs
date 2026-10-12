use axum::http;
use axum::response;
use rand::rngs;
use tokio::sync;
use tokio::task;

use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ServerError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("S3 error: {}", _0)]
    S3(#[error(source)] Box<aws_sdk_s3::Error>),
    #[display("Presigning config error: {}", _0)]
    Presigning(#[error(source)] aws_sdk_s3::presigning::PresigningConfigError),
    #[display("URL parse error: {}", _0)]
    UrlParse(#[error(source)] url::ParseError),
    #[display("concurrency error: {}", _0)]
    Semaphore(#[error(source)] sync::AcquireError),
    #[display("task error: {}", _0)]
    Task(#[error(source)] task::JoinError),
    #[display("hash error: {}", _0)]
    Hash(#[error(source)] argon2::password_hash::Error),
    #[display("phc error: {}", _0)]
    Phc(#[error(source)] argon2::password_hash::phc::Error),
    #[display("Generate error: {}", _0)]
    Generate(#[error(source)] rngs::SysError),
    #[display("JWT Token error: {}", _0)]
    JwtToken(#[error(source)] jsonwebtoken::errors::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
    #[display("No Refresh Token")]
    NoRefreshToken,
    #[display("Invalid Refresh Token")]
    #[from(skip)]
    InvalidRefreshToken { token_hash: Vec<u8> },
    #[display("Expired Refresh Token")]
    #[from(skip)]
    ExpiredRefreshToken { expires_at: time::UtcDateTime },
    #[display("Expired Session")]
    #[from(skip)]
    ExpiredSession { expires_at: time::UtcDateTime },
}

impl response::IntoResponse for ServerError {
    fn into_response(self) -> response::Response {
        match self {
            /*INTERNAL SERVER ERROR*/
            Self::Database(err) => {
                tracing::error!("Database error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Database Access Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::S3(err) => {
                tracing::error!("S3 error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "S3 Access Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Presigning(err) => {
                tracing::error!("Presigning config error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Presigning Config Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::UrlParse(err) => {
                tracing::error!("URL parse error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "URL Parse Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Semaphore(err) => {
                tracing::error!("Semaphore error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "Semaphore Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Task(err) => {
                tracing::error!("Task error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "Task Join Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Hash(err) => {
                tracing::error!("Hash error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "Generate Hash Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Phc(err) => {
                tracing::error!("PHC Hash Error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "PHC Hash Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Generate(err) => {
                tracing::warn!("Refresh Token Generate error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Refresh Token Generate Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::JwtToken(err) => {
                tracing::warn!("JWT Token error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "JWT Token Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            /*BAD REQUEST*/
            Self::Validation(err) => {
                tracing::warn!("Validation error: {err:?}");
                let problem_details = responses::ProblemDetails::from(err);
                (problem_details.status, axum::Json(problem_details)).into_response()
            }
            Self::NoRefreshToken => {
                tracing::warn!("No Refresh Token");
                let problem_details =
                    responses::ProblemDetails::simple("No Refresh Token", http::StatusCode::BAD_REQUEST);
                (http::StatusCode::BAD_REQUEST, axum::Json(problem_details)).into_response()
            }
            Self::InvalidRefreshToken { token_hash } => {
                tracing::warn!("Invalid Refresh Token: {:?}", token_hash);
                let problem_details =
                    responses::ProblemDetails::simple("Invalid Refresh Token", http::StatusCode::BAD_REQUEST);
                (http::StatusCode::BAD_REQUEST, axum::Json(problem_details)).into_response()
            }
            /*UNAUTHORIZED*/
            Self::ExpiredRefreshToken { expires_at } => {
                tracing::warn!("Expired Refresh Token: {}", expires_at);
                let problem_details = responses::ProblemDetails::simple(
                    "Expired Refresh Token",
                    http::StatusCode::UNAUTHORIZED,
                );

                (http::StatusCode::UNAUTHORIZED, axum::Json(problem_details)).into_response()
            }
            Self::ExpiredSession { expires_at } => {
                tracing::warn!("Expired Session: {}", expires_at);
                let problem_details =
                    responses::ProblemDetails::simple("Expired Session", http::StatusCode::UNAUTHORIZED);
                (http::StatusCode::UNAUTHORIZED, axum::Json(problem_details)).into_response()
            }
        }
    }
}

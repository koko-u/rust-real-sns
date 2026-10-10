use std::borrow;
use std::collections;

use axum::http;

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ProblemDetails {
    pub title: String,
    #[serde(serialize_with = "se_status")]
    #[schema(value_type = String)]
    pub status: http::StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<FieldErrors>,
}

impl ProblemDetails {
    pub fn simple(title: &str, status_code: http::StatusCode) -> Self {
        ProblemDetails {
            title: title.to_string(),
            status: status_code,
            detail: None,
            errors: None,
        }
    }
}

impl<R> From<R> for ProblemDetails
where
    R: borrow::Borrow<garde::Report>,
{
    fn from(report: R) -> Self {
        let report = report.borrow();
        let field_errors = report
            .iter()
            .fold(collections::HashMap::new(), |mut hash_map, (field, error)| {
                hash_map
                    .entry(field.to_string())
                    .and_modify(|e: &mut Vec<_>| e.push(error.to_string()))
                    .or_insert(vec![error.to_string()]);
                hash_map
            });

        ProblemDetails {
            title: "Validation error".to_string(),
            status: http::StatusCode::BAD_REQUEST,
            detail: Some(report.to_string()),
            errors: Some(FieldErrors::from(field_errors)),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, derive_more::From, utoipa::ToSchema)]
pub struct FieldErrors(collections::HashMap<String, Vec<String>>);

fn se_status<S>(status: &http::StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(status.as_str())
}

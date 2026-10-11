use axum::extract;

use crate::errors;
use crate::features::users::models;
use crate::features::users::responses;
use crate::shared;

#[utoipa::path(
    get,
    path = "/me",
    tag = "Profile",
    responses(
        (status = 200, description = "Get current user", body = responses::UserResponse),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
pub async fn get_me(
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
) -> Result<axum::Json<responses::UserResponse>, errors::ApiError> {
    let response = current_user.into();

    Ok(axum::Json(response))
}

use axum::extract;
use axum::http;

use crate::errors;
use crate::features::users::models;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/me/following/{target_id}",
    params(
        ("target_id" = models::UserId, Path, description = "the user id to unfollow")
    ),
    tag = "Users",
    description = "Unfollow a user",
    responses(
        (status = 204, description = "No Content"),
        (status = 401, description = "Unauthorized error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
pub async fn unfollow_user(
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
    extract::Path(target_id): extract::Path<models::UserId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    todo!()
}

use axum::extract;

use crate::errors;
use crate::features::users::models;
use crate::features::users::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{user_id}/following",
    params(
        ("user_id" = models::UserId, Path, description = "User id"),
    ),
    description = "Get following users",
    tag = "Users",
    responses(
        (status = 200, description = "List of following users", body = Vec<responses::PublicUserProfileResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_user_following(
    extract::Path(user_id): extract::Path<models::UserId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
) -> Result<axum::Json<Vec<responses::PublicUserProfileResponse>>, errors::ApiError> {
    todo!()
}

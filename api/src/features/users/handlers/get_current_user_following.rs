use axum::extract;

use crate::errors;
use crate::features::users::models;
use crate::features::users::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/me/following",
    description = "Get current user followings",
    tag = "Users",
    responses(
        (status = 200, description = "List of following users", body = Vec<responses::UserProfileResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_current_user_following(
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
) -> Result<axum::Json<Vec<responses::UserProfileResponse>>, errors::ApiError> {
    todo!()
}

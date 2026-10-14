use axum::extract;

use crate::errors;
use crate::features::users::models;
use crate::features::users::repositories;
use crate::features::users::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/me/following",
    description = "Get current user followings",
    tag = "Users",
    responses(
        (status = 200, description = "List of following users", body = Vec<responses::PublicUserProfileResponse>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_current_user_following(
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
) -> Result<axum::Json<Vec<responses::PublicUserProfileResponse>>, errors::ApiError> {
    let user_profile_rows =
        repositories::select_following_user_profiles(&state.pool, current_user.id).await?;
    let mut user_profiles = vec![];
    for row in user_profile_rows {
        let avatar_url = state
            .storage
            .presigned_url_opt(row.avatar_key.as_deref(), time::Duration::days(1))
            .await?;
        let user_profile = models::UserProfileModel::try_from_row(row, avatar_url)?;
        user_profiles.push(user_profile.into());
    }

    Ok(axum::Json(user_profiles))
}

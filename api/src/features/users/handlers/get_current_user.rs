use axum::extract;

use crate::errors;
use crate::features::users::models;
use crate::features::users::repositories;
use crate::features::users::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/me",
    tag = "Users",
    description = "Get current user profile",
    responses(
        (status = 200, description = "Current User Profile", body = responses::MyUserProfileResponse),
        (status = 401, description = "Unauthorized error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
pub async fn get_current_user(
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
) -> Result<axum::Json<responses::MyUserProfileResponse>, errors::ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(errors::ServerError::Database)?;
    let user_profile_row = repositories::select_by_id_with_profile(&mut conn, current_user.id).await?;
    match user_profile_row {
        Some(user_profile_row) => {
            let avatar_url = state
                .storage
                .presigned_url_opt(user_profile_row.avatar_key.as_deref(), time::Duration::days(1))
                .await?;

            let user_profile = models::UserProfileModel::try_from_row(user_profile_row, avatar_url)?;
            Ok(axum::Json(user_profile.into()))
        }
        None => Err(errors::ServerError::not_found("Current User Profile not found").into()),
    }
}

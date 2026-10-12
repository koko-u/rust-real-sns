use axum::extract;

use crate::errors;
use crate::features::profiles::models;
use crate::features::profiles::repositories;
use crate::features::profiles::responses;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    tag = "Profile",
    responses(
        (status = 200, description = "Current User Profile", body = responses::UserProfileResponse),
        (status = 401, description = "Unauthorized error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
pub async fn get_profile(
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<u_models::CurrentUser>,
) -> Result<axum::Json<responses::UserProfileResponse>, errors::ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(errors::ServerError::Database)?;
    let user_profile_row = repositories::select_by_user_id(&mut conn, current_user.id).await?;
    match user_profile_row {
        Some(user_profile_row) => {
            let avatar_url = match &user_profile_row.avatar_key {
                Some(avatar_key) => {
                    let avatar_url = state
                        .storage
                        .presigned_url(avatar_key, time::Duration::days(1))
                        .await?;
                    Some(avatar_url)
                }
                None => None,
            };

            let user_profile = models::UserProfileModel::try_from_row(user_profile_row, avatar_url)?;
            Ok(axum::Json(user_profile.into()))
        }
        None => Err(errors::ApiError::not_found("Current User Profile not found")),
    }
}

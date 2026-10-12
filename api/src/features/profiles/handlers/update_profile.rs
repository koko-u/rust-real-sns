use axum::extract;
use axum_typed_multipart::TypedMultipart;

use crate::errors;
use crate::features::profiles::commands;
use crate::features::profiles::models::UserProfileModel;
use crate::features::profiles::repositories;
use crate::features::profiles::requests;
use crate::features::profiles::responses;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;
#[utoipa::path(
    put,
    path = "",
    request_body (
        content = requests::UpdateProfileRequest,
        content_type = "multipart/form-data",
    ),
    tag = "Profile",
    responses(
        (status = 200, description = "Updated Profile", body = responses::UserProfileResponse),
        (status = 401, description = "Unauthorized error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
pub async fn update_profile(
    extract::State(state): extract::State<state::AppState>,
    extract::Extension(current_user): extract::Extension<u_models::CurrentUser>,
    TypedMultipart(request): TypedMultipart<requests::UpdateProfileRequest>,
) -> Result<axum::Json<responses::UserProfileResponse>, errors::ApiError> {
    // validation
    let commands::UpdateProfileCommand {
        display_name,
        biography,
        website_url,
        location,
        avatar_data,
        avatar_content_type,
    } = request.validate_into()?;

    // save image data
    let avatar_key = match avatar_data {
        Some(avatar_data) => {
            let key = format!(
                "avatar/{user_id}/{filename}",
                user_id = current_user.id,
                filename = shared::generate_random_filename(&avatar_content_type)
            );
            state
                .storage
                .upload(&key, avatar_content_type.as_ref(), avatar_data)
                .await?;

            Some(key)
        }
        _ => None,
    };

    // save profile
    let user_profile_row = shared::transaction::<_, errors::ServerError, _>(&state.pool, async |tx| {
        repositories::upsert(
            tx,
            current_user.id,
            &display_name,
            biography.as_deref(),
            avatar_key.as_deref(),
            website_url.map(|url| url.to_string()).as_deref(),
            location.as_deref(),
        )
        .await?;

        let user_profile_row = repositories::select_by_user_id(tx.conn(), current_user.id).await?;

        Ok(user_profile_row.expect("user profile row should exist"))
    })
    .await?;

    // get presigned url
    let avatar_url = match avatar_key {
        Some(avatar_key) => {
            let avatar_url = state
                .storage
                .presigned_url(&avatar_key, time::Duration::days(1))
                .await?;
            Some(avatar_url)
        }
        None => None,
    };

    let user_profile = UserProfileModel::try_from_row(user_profile_row, avatar_url)?;

    Ok(axum::Json(user_profile.into()))
}

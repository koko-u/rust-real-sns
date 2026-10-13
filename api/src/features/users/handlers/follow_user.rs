use axum::extract;
use axum::http;

use crate::errors;
use crate::features::follows::repositories as f_repositories;
use crate::features::users::models;
use crate::features::users::repositories;
use crate::features::users::responses;
use crate::shared;
use crate::shared::type_utilities::Either;
use crate::state;
#[utoipa::path(
    put,
    path = "/me/following/{target_id}",
    params(
        ("target_id" = models::UserId, Path, description = "the user id to follow")
    ),
    tag = "Users",
    description = "Follow a user",
    responses(
        (status = 200, description = "User's following users", body = responses::FollowingUserResponse),
        (status = 401, description = "Unauthorized error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    ),
)]
#[axum::debug_handler]
pub async fn follow_user(
    extract::Extension(current_user): extract::Extension<models::CurrentUser>,
    extract::Path(target_id): extract::Path<models::UserId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<
    Either<
        (http::StatusCode, axum::Json<shared::responses::ProblemDetails>),
        axum::Json<responses::FollowingUserResponse>,
    >,
    errors::ApiError,
> {
    if current_user.id == target_id {
        let problem_details = shared::responses::ProblemDetails::simple(
            "Cannot follow yourself",
            http::StatusCode::BAD_REQUEST,
        );
        return Ok(Either::Left((
            http::StatusCode::BAD_REQUEST,
            axum::Json(problem_details),
        )));
    }

    let follower_row_or_skip =
        shared::transaction::<Either<_, _>, errors::ServerError, _>(&state.pool, async |tx| {
            // target user profile
            let Some(follower_row) = repositories::select_by_id_with_profile(tx.conn(), target_id).await?
            else {
                return Err(errors::ServerError::not_found("following user is not found"));
            };

            let rows_affected = f_repositories::insert_optional(tx, current_user.id, target_id).await?;
            if rows_affected == 0 {
                let problem_details = shared::responses::ProblemDetails::simple(
                    "User already following the target user",
                    http::StatusCode::OK,
                );

                return Ok(Either::Left((http::StatusCode::OK, axum::Json(problem_details))));
            }

            Ok(Either::Right(follower_row))
        })
        .await?;

    let response = match follower_row_or_skip {
        Either::Left(left) => Either::Left(left),
        Either::Right(row) => {
            // get presigned url
            let avatar_url = match &row.avatar_key {
                Some(avatar_key) => {
                    let avatar_url = state
                        .storage
                        .presigned_url(avatar_key, time::Duration::days(1))
                        .await?;
                    Some(avatar_url)
                }
                None => None,
            };

            let user_profile = models::UserProfileModel::try_from_row(row, avatar_url)?;
            let response = responses::FollowingUserResponse::from(user_profile);
            Either::Right(axum::Json(response))
        }
    };

    Ok(response)
}

use axum::extract;

use crate::errors;
use crate::features::auth::commands;
use crate::features::auth::requests;
use crate::features::auth::responses;
use crate::features::users::repositories as u_repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "/signup",
    security(()),
    request_body = requests::SignupRequest,
    tag = "Auth",
    responses(
        (
            status = 201,
            description = "Created User",
            body = responses::SignupResponse,
            headers(
                ("Location" = String, description = "Created resource location url")
            ),
        ),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn signup(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::SignupRequest>,
) -> Result<shared::responses::Created<responses::SignupResponse>, errors::ApiError> {
    let commands::SignupCommand {
        username,
        email,
        password,
    } = request.validate_into(&state.pool).await?;

    let password_hash = state.passwords.hash(&password).await?;

    let row = shared::transaction::<_, errors::ServerError, _>(&state.pool, async |tx| {
        // create user
        let created_row = u_repositories::insert(tx, &username, &email, &password_hash).await?;

        Ok(created_row)
    })
    .await?;

    let response = responses::SignupResponse {
        message: format!(
            "Successfully Signed by the user {username}",
            username = row.username
        ),
    };
    Ok(shared::responses::created("/api/users/me", response))
}

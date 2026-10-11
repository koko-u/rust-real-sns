use axum::extract;

use crate::errors;
use crate::features::auth::commands;
use crate::features::auth::models;
use crate::features::auth::requests;
use crate::features::auth::responses;
use crate::features::auth::services;
use crate::features::sessions::repositories as s_repositories;
use crate::features::users::repositories as u_repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "/login",
    request_body = requests::LoginRequest,
    tag = "Auth",
    responses(
        (
            status = 200,
            description = "Created Access Token",
            body = responses::LoginResponse,
            headers(
                ("Set-Cookie" = String, description = "refresh_token=abc123; Path=/api/auth; HttpOnly; Secure")
            ),
        ),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn login(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::LoginRequest>,
) -> Result<responses::AuthResponse<responses::LoginResponse>, errors::ApiError> {
    let commands::LoginCommand { username, password } = request.validate_into()?;
    // 1. username からユーザーを取得する
    let Some(user) = u_repositories::select_by_username(&state.pool, &username).await? else {
        tracing::warn!("Invalid username: {}", username);
        return Err(errors::ApiError::Unauthorized);
    };

    // 2. パスワードを検証する
    let verified = state.passwords.verify(&password, &user.password_hash).await?;
    if !verified {
        tracing::warn!("Invalid password: {}", password);
        return Err(errors::ApiError::Unauthorized);
    }

    let now = time::OffsetDateTime::now_utc();
    let expires_at = now + state.jwt_config.refresh_token_expiration;

    let access_token_pair = shared::transaction::<_, errors::ServerError, _>(&state.pool, async |tx| {
        // 3. session id を生成する
        let session = s_repositories::insert(tx, user.id.into(), expires_at).await?;
        let session_id = session.id.into();
        // 4. refresh token と access token を生成する
        let access_token_pair =
            services::issue_refresh_token(tx, now, expires_at, &state.jwt_config, session_id, user.id.into())
                .await?;

        Ok(access_token_pair)
    })
    .await?;

    let models::AccessTokenPair {
        access_token,
        refresh_token,
    } = access_token_pair;
    // レスポンスボディ
    let response = responses::LoginResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_config.jwt_token_expiration.whole_seconds(),
    };
    // Set-Cookie を含めた axum のレスポンスデータ
    let response = responses::AuthResponse::new(
        response,
        &refresh_token,
        state.jwt_config.refresh_token_expiration,
    );
    Ok(response)
}

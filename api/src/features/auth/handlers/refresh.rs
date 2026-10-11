use axum::extract;
use axum_extra::extract as ex_extract;

use crate::errors;
use crate::features::auth::models;
use crate::features::auth::responses;
use crate::features::auth::services;
use crate::features::refresh_tokens::repositories as rt_repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "Auth",
    params(
        ("refresh_token" = String, Cookie, description = "Refresh Token")
    ),
    responses(
        (
            status = 200,
            description = "New Access Token",
            body = responses::RefreshResponse,
            headers(
                ("Set-Cookie" = String, description = "refresh_token=abc123; Path=/api/auth; HttpOnly; Secure")
            ),
        ),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn refresh(
    cookie_jar: ex_extract::CookieJar,
    extract::State(state): extract::State<state::AppState>,
) -> Result<responses::AuthResponse<responses::RefreshResponse>, errors::ApiError> {
    // 1. Cookie から Refresh Token を取得する
    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(errors::ServerError::NoRefreshToken)?;
    // 2. トークンを SHA-256 でハッシュ化する
    use sha2::Digest as _;
    let refresh_token_hash = sha2::Sha256::digest(refresh_token.as_bytes()).to_vec();

    let now = time::OffsetDateTime::now_utc();
    let access_token_pair = shared::transaction::<_, errors::ServerError, _>(&state.pool, async |tx| {
        // 3. refresh_tokens.token_hash から該当レコードを検索する
        let refresh_token_row = rt_repositories::select_by_token_hash(tx.conn(), &refresh_token_hash).await?;
        // 4. トークンとセッションの有効性を検証する
        match refresh_token_row {
            Some(row) if row.expires_at < now => Err(errors::ServerError::ExpiredRefreshToken {
                expires_at: row.expires_at.to_utc(),
            }),
            Some(row) if row.session_expires_at < now => Err(errors::ServerError::ExpiredSession {
                expires_at: row.session_expires_at.to_utc(),
            }),
            Some(row) => {
                // 5. 現在のトークンを consumed_at で使用済みにする
                rt_repositories::update_consumed_at(tx, row.id.into(), now).await?;
                // 6. 同じセッションに新しい Refresh Token のハッシュを登録する
                // 4. refresh token と access token を生成する
                let access_token_pair = services::issue_refresh_token(
                    tx,
                    now,
                    row.session_expires_at,
                    &state.jwt_config,
                    row.session_id.into(),
                    row.session_user_id.into(),
                )
                .await?;
                Ok(access_token_pair)
            }
            None => Err(errors::ServerError::InvalidRefreshToken {
                token_hash: refresh_token_hash,
            }),
        }
    })
    .await?;

    // 7. トランザクションを確定し、新しい JWT と Cookie を返す
    let models::AccessTokenPair {
        access_token,
        refresh_token,
    } = access_token_pair;
    // レスポンスボディ
    let response = responses::RefreshResponse {
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

// 1. Cookie から Refresh Token を取得する
// 2. トークンを SHA-256 でハッシュ化する
// 3. refresh_tokens.token_hash から該当レコードを検索する
// 4. トークンとセッションの有効性を検証する
// 5. 現在のトークンを consumed_at で使用済みにする
// 6. 同じセッションに新しい Refresh Token のハッシュを登録する
// 7. トランザクションを確定し、新しい JWT と Cookie を返す

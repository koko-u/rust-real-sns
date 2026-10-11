use axum::extract;
use axum::http;
use axum_extra::extract as ex_extract;

use crate::errors;
use crate::features::auth::responses;
use crate::features::refresh_tokens::repositories as rt_repositories;
use crate::features::sessions::repositories as s_repositories;
use crate::shared;
use crate::state;
#[utoipa::path(
    post,
    path = "/logout",
    params(
        ("refresh_token" = String, Cookie, description = "Refresh Token")
    ),
    security(()),
    tag = "Auth",
    responses(
        (status = 204, description = "No Content"),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn logout(
    cookie_jar: ex_extract::CookieJar,
    extract::State(state): extract::State<state::AppState>,
) -> Result<(http::StatusCode, responses::AuthResponse<()>), errors::ApiError> {
    // 1. Cookie から Refresh Token を取得する
    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(errors::ServerError::NoRefreshToken)?;
    // 2. トークンを SHA-256 でハッシュ化する
    use sha2::Digest as _;
    let refresh_token_hash = sha2::Sha256::digest(refresh_token.as_bytes()).to_vec();

    tracing::info!("refresh_token: {refresh_token}");
    tracing::info!(
        "refresh_token_hash: 0x{}",
        refresh_token_hash
            .iter()
            .map(|byte| format!("{byte:02X}"))
            .collect::<String>()
    );

    let now = time::OffsetDateTime::now_utc();
    shared::transaction(&state.pool, async |tx| {
        // 3. refresh_tokens.token_hash から該当レコードを検索する
        let refresh_token_row = rt_repositories::select_by_token_hash(tx.conn(), &refresh_token_hash).await?;
        match refresh_token_row {
            Some(row) if row.expires_at < now => Err(errors::ServerError::ExpiredRefreshToken {
                expires_at: row.expires_at.to_utc(),
            }),
            Some(row) if row.session_expires_at < now => Err(errors::ServerError::ExpiredSession {
                expires_at: row.session_expires_at.to_utc(),
            }),
            Some(row) => {
                // 4. sessions の revoked_at を更新する。
                s_repositories::update_revoked_at(tx, row.session_id.into(), now).await?;
                // 5. 同じセッションに属する refresh_tokens をすべて失効させる。
                rt_repositories::update_revoked_at(tx, row.session_id.into(), now).await?;
                Ok(())
            }
            None => Err(errors::ServerError::InvalidRefreshToken {
                token_hash: refresh_token_hash,
            }),
        }
    })
    .await?;

    // 6. レスポンスで refresh token の Cookie を削除する。
    let response = responses::AuthResponse::new((), "", time::Duration::ZERO);
    // 7. 204 No Content を返却する。
    Ok((http::StatusCode::NO_CONTENT, response))
}

// 1. リクエストの Cookie から refresh token を取得する。
// 2. refresh token のハッシュを計算して、refresh_tokens テーブルから対象を特定する。
// 3. 対応する session_id を取得する。
// 4. sessions の revoked_at を更新する。
// 5. 同じセッションに属する refresh_tokens をすべて失効させる。
// 6. レスポンスで refresh token の Cookie を削除する。
// 7. 204 No Content を返却する。

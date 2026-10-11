use crate::errors;
use crate::features::auth::models;
use crate::features::refresh_tokens::repositories as rt_repositories;
use crate::features::sessions::models as s_models;
use crate::features::users::models as u_models;
use crate::shared;
use crate::state;

/*let now = time::OffsetDateTime::now_utc();*/

pub async fn issue_refresh_token(
    tx: &mut shared::Tx<'_>,
    now: time::OffsetDateTime,
    expires_at: time::OffsetDateTime,
    jwt_config: &state::JwtConfig,
    session_id: s_models::SessionId,
    user_id: u_models::UserId,
) -> Result<models::AccessTokenPair, errors::ServerError> {
    use secrecy::ExposeSecret as _;

    let base_claims = models::AccessTokenClaims {
        subject: "".to_string(),
        session_id: "".to_string(),
        issuer: jwt_config.jwt_issuer.clone(),
        audience: jwt_config.jwt_audience.clone(),
        issued_at: now.unix_timestamp(),
        expiration_time: (now + jwt_config.jwt_token_expiration).unix_timestamp(),
    };
    let encoding_key =
        jsonwebtoken::EncodingKey::from_secret(jwt_config.jwt_secret_key.expose_secret().as_bytes());

    // 4. refresh token を生成する
    let shared::RefreshTokenPair {
        refresh_token,
        refresh_token_hash,
    } = shared::generate_refresh_token()?;
    // 5. sessions テーブルと refresh_tokens テーブルにデータを保存する
    rt_repositories::insert(tx, session_id, &refresh_token_hash, expires_at).await?;
    // 6. JWT を生成する
    let claims = models::AccessTokenClaims {
        subject: user_id.to_string(),
        session_id: session_id.to_string(),
        ..base_claims
    };
    // 7. refresh token cookie
    let access_token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)?;

    Ok(models::AccessTokenPair {
        access_token,
        refresh_token,
    })
}

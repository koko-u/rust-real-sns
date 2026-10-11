use axum::extract;
use axum::http;
use axum::middleware;
use axum::response;
use secrecy::ExposeSecret;

use crate::errors;
use crate::features::auth::models as a_models;
use crate::features::sessions::models as s_models;
use crate::features::sessions::repositories as s_repositories;
use crate::features::users::models as u_models;
use crate::features::users::repositories as u_repositories;
use crate::state;

pub async fn auth_middleware(
    extract::State(state): extract::State<state::AppState>,
    mut request: extract::Request,
    next: middleware::Next,
) -> Result<response::Response, errors::ApiError> {
    // Authorization header から Bearer トークンを取り出す
    let token = extract_bearer_token(&mut request)?;

    // JWT の検証（HS256 の例）
    let secret = state.jwt_config.jwt_secret_key.expose_secret().as_bytes();
    let key = jsonwebtoken::DecodingKey::from_secret(secret);

    let validation = create_jwt_validation(&state.jwt_config);

    let decoded = jsonwebtoken::decode::<a_models::AccessTokenClaims>(token, &key, &validation)
        .map_err(|_| errors::ApiError::unauthorized("Invalid JWT token"))?;

    // check issuer
    decoded
        .claims
        .issuer
        .eq_ignore_ascii_case(state.jwt_config.jwt_issuer.as_str())
        .ok_or(errors::ApiError::unauthorized("Invalid JWT issuer"))?;

    // check subject is user_id
    let user_id = decoded
        .claims
        .subject
        .parse::<u_models::UserId>()
        .map_err(|_| errors::ApiError::unauthorized("Invalid JWT subject"))?;
    let Some(user_row) = u_repositories::select_by_id(&state.pool, user_id).await? else {
        return Err(errors::ApiError::unauthorized("Invalid JWT subject"));
    };

    // get session data (which is not expired)
    let session_id = decoded
        .claims
        .session_id
        .parse::<s_models::SessionId>()
        .map_err(|_| errors::ApiError::unauthorized("Invalid JWT session_id"))?;
    let exists = s_repositories::exists_by_id(&state.pool, session_id).await?;
    if !exists {
        return Err(errors::ApiError::unauthorized("Invalid JWT session_id"));
    }

    // Handler 用の認証情報
    let auth_user = u_models::CurrentUser {
        id: user_id,
        session_id,
        username: user_row.username,
        email: user_row.email,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

fn extract_bearer_token(request: &mut extract::Request) -> Result<&str, errors::ApiError> {
    // Authorization ヘッダの取得
    let authorization = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(errors::ApiError::unauthorized("No Authorization header"))?;

    // Bearer トークンを取り出す
    let (scheme, token) = authorization
        .split_once(' ')
        .ok_or(errors::ApiError::unauthorized("Invalid Authorization header"))?;

    if !scheme.eq_ignore_ascii_case("Bearer") || token.is_empty() {
        return Err(errors::ApiError::unauthorized("Invalid Authorization header"));
    }

    Ok(token)
}

fn create_jwt_validation(jwt_config: &state::JwtConfig) -> jsonwebtoken::Validation {
    let audience = jwt_config.jwt_audience.clone();
    let issuer = jwt_config.jwt_issuer.clone();

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default());
    validation.set_required_spec_claims(&["sub", "sid", "iss", "aud", "iat", "exp"]);
    validation.set_audience(&[audience]);
    validation.set_issuer(&[issuer]);
    validation.validate_aud = true;
    validation.validate_exp = true;

    validation
}

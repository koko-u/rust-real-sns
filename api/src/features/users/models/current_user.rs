use axum::extract;
use axum::http::request;
use axum_keycloak_auth::decode;

use crate::errors;
use crate::features::users::models;
use crate::keycloak_auth;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrentUser {
    pub id: models::UserId,
    pub roles: Vec<keycloak_auth::AppRole>,
    pub username: String,
    pub email: String,
}

impl<S> extract::FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = errors::ApiError;

    async fn from_request_parts(parts: &mut request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .extensions
            .get::<decode::KeycloakToken<keycloak_auth::AppRole>>()
            .ok_or(errors::ApiError::Unauthorized)?;

        let id = token
            .subject
            .parse::<models::UserId>()
            .map_err(|_| errors::ApiError::Unauthorized)?;

        let roles = token.roles.iter().map(|r| r.role()).cloned().collect();
        let decode::ProfileAndEmail { profile, email } = &token.extra;

        Ok(Self {
            id,
            roles,
            username: profile.preferred_username.to_string(),
            email: email.email.to_string(),
        })
    }
}

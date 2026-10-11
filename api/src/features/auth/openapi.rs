use crate::features::auth::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(login, logout, refresh, signup))]
pub struct AuthApi;

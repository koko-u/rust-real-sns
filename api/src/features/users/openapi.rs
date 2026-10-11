use crate::features::users::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(get_me))]
pub struct UsersApi;

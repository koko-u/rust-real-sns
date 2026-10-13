use crate::features::auth::openapi::AuthApi;
use crate::features::health_check::*;
use crate::features::users::openapi::UsersApi;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
    ok
    ),
    nest(
        (path = "/api/auth", api = AuthApi),
        (path = "/api/users", api = UsersApi),
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;

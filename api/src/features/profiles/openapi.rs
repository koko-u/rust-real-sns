use crate::features::profiles::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(get_profile, update_profile,))]
pub struct ProfilesApi;

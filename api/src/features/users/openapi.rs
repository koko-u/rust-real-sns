use crate::features::users::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    follow_user,
    get_current_user,
    get_current_user_following,
    get_user_followers,
    get_user_following,
    unfollow_user,
    update_current_user_profile,
))]
pub struct UsersApi;

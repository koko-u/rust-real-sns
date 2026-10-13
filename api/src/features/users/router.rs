use axum::routing;

use crate::features::users::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/users/me",
            routing::MethodRouter::new()
                .get(handlers::get_current_user)
                .put(handlers::update_current_user_profile),
        )
        .route(
            "/users/me/following",
            routing::get(handlers::get_current_user_following),
        )
        .route(
            "/users/me/following/{target_id}",
            routing::MethodRouter::new()
                .put(handlers::follow_user)
                .delete(handlers::unfollow_user),
        )
        .route(
            "/users/{user_id}/following",
            routing::get(handlers::get_user_following),
        )
        .route(
            "/users/{user_id}/followers",
            routing::get(handlers::get_user_followers),
        )
}

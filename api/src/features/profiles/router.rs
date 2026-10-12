use axum::routing;

use crate::features::profiles::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new().route(
        "/profiles",
        routing::MethodRouter::new()
            .get(handlers::get_profile)
            .put(handlers::update_profile),
    )
}

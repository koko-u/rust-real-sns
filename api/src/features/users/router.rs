use axum::routing;

use crate::features::users::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new().route("/users/me", routing::get(handlers::get_me))
}

use axum::routing;

use crate::features::auth::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route("/signup", routing::post(handlers::signup))
        .route("/login", routing::post(handlers::login))
        .route("/refresh", routing::post(handlers::refresh))
        .route("/logout", routing::post(handlers::logout))
}

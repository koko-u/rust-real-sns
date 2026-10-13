use axum::middleware;
use axum::routing;

use crate::features::auth;
use crate::features::health_check;
use crate::features::users;
use crate::middlewares;
use crate::state;

pub fn app_router(state: state::AppState) -> axum::Router<state::AppState> {
    let protected_routes = axum::Router::new().merge(users::router());
    let protected_routes = protected_routes.layer(middleware::from_fn_with_state(
        state.clone(),
        middlewares::auth_middleware,
    ));

    axum::Router::new()
        .route("/health-check", routing::get(health_check::ok))
        .nest("/api/auth", auth::router())
        .nest("/api", protected_routes)
}

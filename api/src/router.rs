use axum::routing;
use axum_keycloak_auth::layer;
use axum_keycloak_auth::role;

use crate::features::health_check;
use crate::features::product_brands;
use crate::state;

pub fn app_router<R>(auth_layer: layer::KeycloakAuthLayer<R>) -> axum::Router<state::AppState>
where
    R: role::Role + 'static,
{
    let protected_routes = axum::Router::new()
        .merge(product_brands::router());
    // let protected_routes = protected_routes.layer(auth_layer);

    axum::Router::new()
        .route("/health-check", routing::get(health_check::ok))
        .nest("/api", protected_routes)        
}

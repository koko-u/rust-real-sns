use axum::routing;

use crate::features::product_brands::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/brands",
            routing::MethodRouter::new()
                .get(handlers::get_brands)
                .post(handlers::create_brand),
        )
        .route(
            "/brands/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_brand_by_id)
                .put(handlers::update_brand)
                .delete(handlers::delete_brand),
        )
}

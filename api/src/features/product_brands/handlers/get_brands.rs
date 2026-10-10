use axum::extract;

use crate::errors;
use crate::features::product_brands::models;
use crate::features::product_brands::repositories;
use crate::features::product_brands::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    description = "Get All Product Brands",
    tag = "Product Brands",
    responses(
        (status = 200, description = "List of Product Brands", body = Vec<responses::ProductBrand>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_brands(
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::ProductBrand>>, errors::ApiError> {
    let products = repositories::select_all(&state.pool).await?;

    let response = products
        .into_iter()
        .map(models::ProductBrand::from)
        .map(responses::ProductBrand::from)
        .collect();

    Ok(axum::Json(response))
}

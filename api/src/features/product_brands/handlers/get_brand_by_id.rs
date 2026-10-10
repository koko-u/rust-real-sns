use axum::extract;

use crate::errors;
use crate::features::product_brands::models;
use crate::features::product_brands::repositories;
use crate::features::product_brands::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = models::ProductBrandId, Path, description = "Product Brand Id")
    ),
    description = "Get One Product brand by Id",
    tag = "Product Brands",
    responses(
        (status = 200, description = "Product for supplied id", body = responses::ProductBrand),
        (status = 404, description = "the Product is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_brand_by_id(
    extract::Path(id): extract::Path<models::ProductBrandId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::ProductBrand>, errors::ApiError> {
    let product = repositories::select_by_id(&state.pool, id).await?;

    match product {
        Some(product) => {
            let product: models::ProductBrand = product.into();
            let product: responses::ProductBrand = product.into();
            Ok(axum::Json(product))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Product with Id={id} not found"),
        }),
    }
}

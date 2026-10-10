use axum::extract;

use crate::errors;
use crate::features::product_brands::models;
use crate::features::product_brands::repositories;
use crate::features::product_brands::requests;
use crate::features::product_brands::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::ProductBrandId, Path, description = "Product brand Id")
    ),
    request_body = requests::UpdateBrand,
    tag = "Product Brands",
    responses(
        (status = 200, description = "the Updated Product", body = responses::ProductBrand),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_brand(
    extract::Path(id): extract::Path<models::ProductBrandId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateBrand>,
) -> Result<axum::Json<responses::ProductBrand>, errors::ApiError> {
    // validate request
    let command = request.validate_into(id, &state.pool).await?;

    let updated_brand = shared::transaction(&state.pool, async |tx| {
        // update product brand
        let row = repositories::update(tx, &command).await?;

        match row {
            Some(row) => Ok(models::ProductBrand::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update product, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(axum::Json(updated_brand.into()))
}

use axum::extract;

use crate::errors;
use crate::features::product_brands::models;
use crate::features::product_brands::repositories;
use crate::features::product_brands::requests;
use crate::features::product_brands::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreateBrand,
    tag = "Product Brands",
    responses(
        (status = 201, description = "Created new Product", body = responses::ProductBrand),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_brand(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateBrand>,
) -> Result<shared::responses::Created<responses::ProductBrand>, errors::ApiError> {
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let command = request.validate_into(&state.pool).await?;

    let created_brand: models::ProductBrand = shared::transaction(&state.pool, async |tx| {
        // create brand
        let created_row = repositories::insert(tx, &command).await?;

        match created_row {
            Some(row) => Ok(row.into()),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot create product brand, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    let location = format!("/api/brands/{}", created_brand.id);
    Ok(shared::responses::created(location, created_brand.into()))
}

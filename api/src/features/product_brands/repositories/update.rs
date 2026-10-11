use into_inner::IntoInner;

use crate::errors;
use crate::features::product_brands::commands;
use crate::features::product_brands::rows;
use crate::shared;

pub async fn update(
    tx: &mut shared::Tx<'_>,
    command: &commands::UpdateBrand,
) -> Result<Option<rows::ProductBrandRow>, errors::ServerError> {
    let commands::UpdateBrand { id, name } = command;

    let row = sqlx::query_file_as!(
        rows::ProductBrandRow,
        "sql/product_brands/update.sql",
        id.into_inner(),
        name,
    )
    .fetch_optional(tx.conn())
    .await?;

    Ok(row)
}

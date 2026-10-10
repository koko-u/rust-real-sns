use crate::features::product_brands::models;
use crate::features::product_brands::rows;
use crate::shared;

pub async fn delete(
    tx: &mut shared::Tx<'_>,
    id: models::ProductBrandId,
) -> Result<Option<rows::ProductBrandRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::ProductBrandRow,
        "sql/product_brands/delete.sql",
        id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}

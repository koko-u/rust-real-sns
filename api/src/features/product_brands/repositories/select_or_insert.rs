use crate::features::product_brands::rows;
use crate::shared;

pub async fn select_or_insert(
    tx: &mut shared::Tx<'_>,
    name: &str,
) -> Result<rows::ProductBrandRow, sqlx::Error> {
    sqlx::query_file_as!(
        rows::ProductBrandRow,
        "sql/product_brands/select_or_insert.sql",
        name
    )
    .fetch_one(tx.conn())
    .await
}

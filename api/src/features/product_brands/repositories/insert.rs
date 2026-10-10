use crate::features::product_brands::commands;
use crate::features::product_brands::rows;
use crate::shared;

pub async fn insert(
    tx: &mut shared::Tx<'_>,
    command: &commands::CreateBrand,
) -> Result<Option<rows::ProductBrandRow>, sqlx::Error> {
    let commands::CreateBrand { name } = command;

    sqlx::query_file_as!(rows::ProductBrandRow, "sql/product_brands/insert.sql", name,)
        .fetch_optional(tx.conn())
        .await
}

use crate::features::product_brands::models;
use crate::features::product_brands::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::ProductBrandId,
) -> Result<Option<rows::ProductBrandRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::ProductBrandRow,
        "sql/product_brands/select_by_id.sql",
        id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}

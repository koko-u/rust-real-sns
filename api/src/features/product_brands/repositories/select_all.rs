use crate::errors;
use crate::features::product_brands::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::ProductBrandRow>, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(rows::ProductBrandRow, "sql/product_brands/select_all.sql")
        .fetch_all(conn.as_mut())
        .await?;

    Ok(rows)
}

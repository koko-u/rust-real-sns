use crate::features::product_brands::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::ProductBrandRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::ProductBrandRow, "sql/product_brands/select_all.sql")
        .fetch_all(conn.as_mut())
        .await
}

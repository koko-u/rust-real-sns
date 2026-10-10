use crate::features::product_brands::models;

pub async fn exists_by_name_except_id(
    pool: &sqlx::PgPool,
    id: models::ProductBrandId,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!(
        "sql/product_brands/exists_by_name_except_id.sql",
        id.into_inner(),
        name
    )
    .fetch_one(conn.as_mut())
    .await
}

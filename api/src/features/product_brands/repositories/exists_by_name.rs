pub async fn exists_by_name(pool: &sqlx::PgPool, name: &str) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!("sql/product_brands/exists_by_name.sql", name)
        .fetch_one(conn.as_mut())
        .await
}

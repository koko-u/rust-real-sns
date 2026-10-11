use crate::errors;

pub async fn exists_by_name(pool: &sqlx::PgPool, name: &str) -> Result<bool, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let exists = sqlx::query_file_scalar!("sql/product_brands/exists_by_name.sql", name)
        .fetch_one(conn.as_mut())
        .await?;

    Ok(exists)
}

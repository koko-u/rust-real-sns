use crate::errors;

pub async fn exists_by_email(pool: &sqlx::PgPool, email: &str) -> Result<bool, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let exists = sqlx::query_file_scalar!("sql/users/exists_by_email.sql", email)
        .fetch_one(conn.as_mut())
        .await?;

    Ok(exists)
}

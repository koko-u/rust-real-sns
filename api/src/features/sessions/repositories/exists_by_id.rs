use crate::errors;
use crate::features::sessions::models;

pub async fn exists_by_id(pool: &sqlx::PgPool, id: models::SessionId) -> Result<bool, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let exists = sqlx::query_file_scalar!("sql/sessions/exists_by_id.sql", id.into_inner())
        .fetch_one(conn.as_mut())
        .await?;

    Ok(exists)
}

use crate::errors;
use crate::features::users::models;
use crate::features::users::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::UserId,
) -> Result<Option<rows::UserRow>, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let row = sqlx::query_file_as!(rows::UserRow, "sql/users/select_by_id.sql", id.into_inner())
        .fetch_optional(conn.as_mut())
        .await?;

    Ok(row)
}

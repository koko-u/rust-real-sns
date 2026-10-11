use crate::errors;
use crate::features::users::rows;

pub async fn select_by_username(
    pool: &sqlx::PgPool,
    username: &str,
) -> Result<Option<rows::UserWithPasswordHashRow>, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let row = sqlx::query_file_as!(
        rows::UserWithPasswordHashRow,
        "sql/users/select_by_username.sql",
        username
    )
    .fetch_optional(conn.as_mut())
    .await?;

    Ok(row)
}

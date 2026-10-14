use crate::errors;
use crate::features::users::models;
use crate::features::users::rows;

pub async fn select_following_user_profiles(
    pool: &sqlx::PgPool,
    id: models::UserId,
) -> Result<Vec<rows::UserProfileRow>, errors::ServerError> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(
        rows::UserProfileRow,
        "sql/users/select_following_user_profiles.sql",
        id.into_inner()
    )
    .fetch_all(conn.as_mut())
    .await?;

    Ok(rows)
}

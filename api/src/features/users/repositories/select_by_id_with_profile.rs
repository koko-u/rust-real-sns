use crate::errors;
use crate::features::users::models;
use crate::features::users::rows;

pub async fn select_by_id_with_profile(
    conn: &mut sqlx::PgConnection,
    id: models::UserId,
) -> Result<Option<rows::UserProfileRow>, errors::ServerError> {
    let row = sqlx::query_file_as!(
        rows::UserProfileRow,
        "sql/users/select_by_id_with_profile.sql",
        id.into_inner()
    )
    .fetch_optional(conn)
    .await?;

    Ok(row)
}

use crate::errors;
use crate::features::profiles::rows;
use crate::features::users::models as u_models;

pub async fn select_by_user_id(
    conn: &mut sqlx::PgConnection,
    user_id: u_models::UserId,
) -> Result<Option<rows::UserProfileRow>, errors::ServerError> {
    let row = sqlx::query_file_as!(
        rows::UserProfileRow,
        "sql/profiles/select_by_user_id.sql",
        user_id.into_inner()
    )
    .fetch_optional(conn)
    .await?;

    Ok(row)
}

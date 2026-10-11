use crate::errors;
use crate::features::sessions::rows;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    expires_at: time::OffsetDateTime,
) -> Result<rows::SessionRow, errors::ServerError> {
    let row = sqlx::query_file_as!(
        rows::SessionRow,
        "sql/sessions/insert.sql",
        user_id.into_inner(),
        expires_at
    )
    .fetch_one(tx.conn())
    .await?;

    Ok(row)
}

use crate::errors;
use crate::features::sessions::models;
use crate::shared;

pub async fn update_revoked_at(
    tx: &mut shared::Tx<'_>,
    id: models::SessionId,
    revoked_at: time::OffsetDateTime,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!("sql/sessions/update_revoked_at.sql", id.into_inner(), revoked_at,)
        .execute(tx.conn())
        .await?;

    Ok(result.rows_affected())
}

use crate::errors;
use crate::features::sessions::models as s_models;
use crate::shared;

pub async fn update_revoked_at(
    tx: &mut shared::Tx<'_>,
    session_id: s_models::SessionId,
    revoked_at: time::OffsetDateTime,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/refresh_tokens/update_revoked_at.sql",
        session_id.into_inner(),
        revoked_at,
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}

use crate::errors;
use crate::features::sessions::models as s_models;
use crate::shared;

pub async fn insert(
    tx: &mut shared::Tx<'_>,
    session_id: s_models::SessionId,
    token_hash: &[u8],
    expires_at: time::OffsetDateTime,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/refresh_tokens/insert.sql",
        session_id.into_inner(),
        token_hash,
        expires_at,
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}

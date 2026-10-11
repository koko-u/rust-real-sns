use crate::errors;
use crate::features::refresh_tokens::models as rt_models;
use crate::shared;

pub async fn update_consumed_at(
    tx: &mut shared::Tx<'_>,
    id: rt_models::RefreshTokenId,
    consumed_at: time::OffsetDateTime,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/refresh_tokens/update_consumed_at.sql",
        id.into_inner(),
        consumed_at,
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}

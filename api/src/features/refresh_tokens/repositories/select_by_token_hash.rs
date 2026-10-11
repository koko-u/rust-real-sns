use crate::errors;
use crate::features::refresh_tokens::rows;

pub async fn select_by_token_hash(
    conn: &mut sqlx::PgConnection,
    token_hash: &[u8],
) -> Result<Option<rows::RefreshTokenRow>, errors::ServerError> {
    // let mut conn = pool.acquire().await?;
    let row = sqlx::query_file_as!(
        rows::RefreshTokenRow,
        "sql/refresh_tokens/select_by_token_hash.sql",
        token_hash
    )
    .fetch_optional(conn.as_mut())
    .await?;

    Ok(row)
}

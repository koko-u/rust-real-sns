use crate::errors;
use crate::features::users::rows;
use crate::shared;

pub async fn insert(
    tx: &mut shared::Tx<'_>,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<rows::UserRow, errors::ServerError> {
    let row = sqlx::query_file_as!(
        rows::UserRow,
        "sql/users/insert.sql",
        username,
        email,
        password_hash,
    )
    .fetch_one(tx.conn())
    .await?;

    Ok(row)
}

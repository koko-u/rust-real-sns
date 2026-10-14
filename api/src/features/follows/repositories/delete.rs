use crate::{errors, shared};

pub async fn delete(
    tx: &mut shared::Tx<'_>,
    follower_id: crate::features::users::models::UserId,
    followee_id: crate::features::users::models::UserId,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/follows/delete.sql",
        follower_id.into_inner(),
        followee_id.into_inner()
    )
    .execute(tx.conn())
    .await?;

    Ok(result.rows_affected())
}

use crate::errors;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    follower_id: u_models::UserId,
    followee_id: u_models::UserId,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/follows/insert_optional.sql",
        follower_id.into_inner(),
        followee_id.into_inner()
    )
    .execute(tx.conn())
    .await?;

    Ok(result.rows_affected())
}

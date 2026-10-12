use crate::errors;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn upsert(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    display_name: &str,
    biography: Option<&str>,
    avatar_key: Option<&str>,
    website_url: Option<&str>,
    location: Option<&str>,
) -> Result<u64, errors::ServerError> {
    let result = sqlx::query_file!(
        "sql/profiles/upsert.sql",
        user_id.into_inner(),
        display_name,
        biography,
        avatar_key,
        website_url,
        location
    )
    .execute(tx.conn())
    .await?;

    Ok(result.rows_affected())
}

use crate::features::users::models as u_models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UserProfileRow {
    pub id: u_models::UserId,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub biography: Option<String>,
    pub avatar_key: Option<String>,
    pub website_url: Option<String>,
    pub location: Option<String>,
}

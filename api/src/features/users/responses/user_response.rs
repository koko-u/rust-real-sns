use crate::features::users::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

impl From<models::CurrentUser> for UserResponse {
    fn from(value: models::CurrentUser) -> Self {
        Self {
            id: value.id.into_inner(),
            session_id: value.session_id.into_inner(),
            username: value.username,
            email: value.email,
        }
    }
}

use crate::features::sessions::models as s_models;
use crate::features::users::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrentUser {
    pub id: models::UserId,
    pub session_id: s_models::SessionId,
    pub username: String,
    pub email: String,
}

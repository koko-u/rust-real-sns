use into_inner::IntoInner;

use crate::features::users::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct PublicUserProfileResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<url::Url>,
    pub website_url: Option<url::Url>,
}

impl From<models::UserProfileModel> for PublicUserProfileResponse {
    fn from(user_profile: models::UserProfileModel) -> Self {
        Self {
            id: user_profile.id.into_inner(),
            username: user_profile.username,
            display_name: user_profile.display_name,
            avatar_url: user_profile.avatar_url,
            website_url: user_profile.website_url,
        }
    }
}

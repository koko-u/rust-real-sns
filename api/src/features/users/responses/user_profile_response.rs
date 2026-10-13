use crate::features::users::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct UserProfileResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub biography: Option<String>,
    pub avatar_url: Option<url::Url>,
    pub website_url: Option<url::Url>,
    pub location: Option<String>,
}

impl From<models::UserProfileModel> for UserProfileResponse {
    fn from(model: models::UserProfileModel) -> Self {
        Self {
            id: model.id.into_inner(),
            username: model.username,
            email: model.email,
            display_name: model.display_name,
            biography: model.biography,
            avatar_url: model.avatar_url,
            website_url: model.website_url,
            location: model.location,
        }
    }
}

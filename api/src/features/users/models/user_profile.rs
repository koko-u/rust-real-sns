use crate::errors;
use crate::features::users::models;
use crate::features::users::rows;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserProfileModel {
    pub id: models::UserId,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub biography: Option<String>,
    pub avatar_url: Option<url::Url>,
    pub website_url: Option<url::Url>,
    pub location: Option<String>,
}

impl UserProfileModel {
    pub fn try_from_row(
        row: rows::UserProfileRow,
        avatar_url: Option<url::Url>,
    ) -> Result<Self, errors::ServerError> {
        let website_url = match &row.website_url {
            Some(website_url) => {
                let website_url = website_url.parse::<url::Url>()?;
                Some(website_url)
            }
            None => None,
        };
        Ok(Self {
            id: row.id,
            username: row.username,
            email: row.email,
            display_name: row.display_name,
            biography: row.biography,
            avatar_url,
            website_url,
            location: row.location,
        })
    }
}

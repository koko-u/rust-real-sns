use axum::body;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateUserProfileCommand {
    pub display_name: String,
    pub biography: Option<String>,
    pub website_url: Option<url::Url>,
    pub location: Option<String>,
    pub avatar_data: Option<body::Bytes>,
    pub avatar_content_type: mime::Mime,
}

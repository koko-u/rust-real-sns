use axum_keycloak_auth::role;

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    derive_more::Display,
    derive_more::IsVariant,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AppRole {
    #[display("admin")]
    Admin,
    #[display("user")]
    User,
    #[display("client_role({})", _0)]
    Client(ClientRole),
    #[display("unknown: {}", _0)]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    derive_more::Display,
    derive_more::IsVariant,
    serde::Serialize,
    utoipa::ToSchema,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum ClientRole {
    #[display("reader")]
    Reader,
    #[display("writer")]
    Writer,
}

impl From<String> for AppRole {
    fn from(value: String) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "admin" => Self::Admin,
            "user" => Self::User,
            "reader" => Self::Client(ClientRole::Reader),
            "writer" => Self::Client(ClientRole::Writer),
            _ => Self::Unknown(value),
        }
    }
}

impl role::Role for AppRole {}

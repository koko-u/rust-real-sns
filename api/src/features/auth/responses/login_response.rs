#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

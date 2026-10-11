#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenClaims {
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "sid")]
    pub session_id: String,
    #[serde(rename = "iss")]
    pub issuer: String,
    #[serde(rename = "aud")]
    pub audience: String,
    #[serde(rename = "iat")]
    pub issued_at: i64,
    #[serde(rename = "exp")]
    pub expiration_time: i64,
}

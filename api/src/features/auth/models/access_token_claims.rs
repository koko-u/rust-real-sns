#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenClaims {
    /// The subject of the token, the user Id.
    #[serde(rename = "sub")]
    pub subject: String,

    /// The session Id.
    #[serde(rename = "sid")]
    pub session_id: String,

    /// JWT Issuer application Id
    #[serde(rename = "iss")]
    pub issuer: String,

    /// JWT Audience application Id
    #[serde(rename = "aud")]
    pub audience: String,

    /// JWT Issued at time
    #[serde(rename = "iat")]
    pub issued_at: i64,

    /// JWT Expiration time
    #[serde(rename = "exp")]
    pub expiration_time: i64,
}

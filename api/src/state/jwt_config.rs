use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub jwt_secret_key: SecretString,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_token_expiration: time::Duration,
    pub refresh_token_expiration: time::Duration,
}

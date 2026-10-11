#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

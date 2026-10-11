use base64::engine;
use rand::rngs;

pub struct RefreshTokenPair {
    pub refresh_token: String,
    pub refresh_token_hash: Vec<u8>,
}

pub fn generate_refresh_token() -> Result<RefreshTokenPair, rngs::SysError> {
    use base64::Engine as _;
    use rand::TryRng as _;
    use sha2::Digest as _;

    let mut bytes = [0u8; 32];

    rngs::SysRng.try_fill_bytes(&mut bytes)?;

    let refresh_token = engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    let refresh_token_hash = sha2::Sha256::digest(refresh_token.as_bytes()).to_vec();

    Ok(RefreshTokenPair {
        refresh_token,
        refresh_token_hash,
    })
}

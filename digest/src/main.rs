use std::env;

use sha2::Digest;

fn main() {
    let Some(refresh_token) = env::args().nth(1) else {
        eprintln!("Refresh token not provided");
        return;
    };
    let refresh_token_hash = sha2::Sha256::digest(refresh_token.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<String>();

    println!("Refresh token hash: 0x{refresh_token_hash}");
}

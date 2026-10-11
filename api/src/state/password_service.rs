use tokio::sync;
use tokio::task;

use crate::errors;
use crate::shared;

#[derive(Debug, Clone)]
pub struct PasswordService {
    semaphore: std::sync::Arc<sync::Semaphore>,
}

impl PasswordService {
    pub fn new(semaphore_size: shared::SemaphoreSize) -> Self {
        Self {
            semaphore: std::sync::Arc::new(sync::Semaphore::new(semaphore_size.into_inner())),
        }
    }

    pub async fn hash(&self, password: &str) -> Result<String, errors::ServerError> {
        // セマフォから実行許可を獲得します
        let permit = self.semaphore.clone().acquire_owned().await?;

        // 引数の password を task に渡すために改めてヒープに確保する
        let password = password.to_string();
        let hash = task::spawn_blocking(move || {
            use argon2::PasswordHasher as _;
            // セマフォの実行許可を移動することで、ハッシュの計算が終わるまでセマフォを保持します
            let _permit = permit;
            let argon = argon2::Argon2::default();
            let hash = argon.hash_password(password.as_bytes());

            hash.map(|hash| hash.to_string())
        })
        .await??;

        Ok(hash)
    }

    pub async fn verify(&self, password: &str, password_hash: &str) -> Result<bool, errors::ServerError> {
        // セマフォから実行許可を獲得します
        let permit = self.semaphore.clone().acquire_owned().await?;

        // 引数の password を task に渡すために改めてヒープに確保する
        let password = password.to_string();
        let password_hash = password_hash.to_string();
        let verified = task::spawn_blocking(move || {
            use argon2::PasswordVerifier as _;
            // セマフォの実行許可を移動することで、ハッシュの計算が終わるまでセマフォを保持します
            let _permit = permit;
            let argon = argon2::Argon2::default();
            match argon2::password_hash::phc::PasswordHash::new(&password_hash) {
                Ok(hash) => {
                    let verified = argon.verify_password(password.as_bytes(), &hash);
                    Ok(verified.is_ok())
                }
                Err(error) => return Err(errors::ServerError::Phc(error)),
            }
        })
        .await??;

        Ok(verified)
    }
}

pub mod config;
pub mod cors;
pub mod openapi;
pub mod router;
pub mod state;

mod errors;
mod features;
mod middlewares;
mod shared;

#[cfg(test)]
mod tests {
    use rand::rngs;
    use tokio::sync;
    use tokio::task;

    use crate::errors;

    #[test]
    fn find_size() {
        use std::mem::size_of;

        println!("ServerError: {}", size_of::<errors::ServerError>());
        println!("sqlx::Error: {}", size_of::<sqlx::Error>());
        println!("aws_sdk_s3::Error: {}", size_of::<aws_sdk_s3::Error>());
        println!("sync::AcquireError: {}", size_of::<sync::AcquireError>());
        println!("task::JoinError: {}", size_of::<task::JoinError>());
        println!(
            "argon2::password_hash::Error: {}",
            size_of::<argon2::password_hash::Error>()
        );
        println!(
            "argon2::password_hash::phc::Error: {}",
            size_of::<argon2::password_hash::phc::Error>()
        );
        println!("rngs::SysError: {}", size_of::<rngs::SysError>());
        println!(
            "jsonwebtoken::Error: {}",
            size_of::<jsonwebtoken::errors::Error>()
        );
        println!("garde::Report: {}", size_of::<garde::Report>());
    }
}

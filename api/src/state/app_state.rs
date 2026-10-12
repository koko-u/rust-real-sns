use sqlx::postgres;

use crate::config;
use crate::state;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub passwords: state::PasswordService,
    pub jwt_config: std::sync::Arc<state::JwtConfig>,
    pub s3_client: aws_sdk_s3::Client,
}

impl AppState {
    pub async fn new(config: &config::Config) -> Result<Self, sqlx::Error> {
        let database_url = config.database_url();
        let max_connections = config.max_connections();
        let semaphore_size = config.semaphore_size();
        let jwt_config = config.jwt_config();

        let pool = postgres::PgPoolOptions::new()
            .max_connections(max_connections.into_inner())
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    // When directly invoking `Executor` methods,
                    // it is possible to execute multiple statements with one call.
                    let v = sqlx::query_scalar!("SELECT version()").fetch_one(conn).await?;
                    tracing::info!("{v:?}");
                    Ok(())
                })
            })
            .connect(&database_url)
            .await?;

        let passwords = state::PasswordService::new(semaphore_size);

        let config = config.s3_config().await;
        let s3_client = aws_sdk_s3::Client::from_conf(config);

        Ok(Self {
            pool,
            passwords,
            jwt_config: std::sync::Arc::new(jwt_config),
            s3_client,
        })
    }
}

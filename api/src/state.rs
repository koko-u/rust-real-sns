use sqlx::postgres;

use crate::shared;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

impl AppState {
    pub async fn new(
        database_url: &str,
        max_connections: shared::MaxConnections,
    ) -> Result<Self, sqlx::Error> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(max_connections.into())
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    // When directly invoking `Executor` methods,
                    // it is possible to execute multiple statements with one call.
                    let v = sqlx::query_scalar!("SELECT version()").fetch_one(conn).await?;
                    tracing::info!("{v:?}");
                    Ok(())
                })
            })
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

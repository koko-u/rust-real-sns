use api::auth;
use api::config;
use api::cors;
use api::router;
use api::state;
use tower_http::trace;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
    let config = config::Config::new()?;

    let state = state::AppState::new(&config.database_url(), config.max_connections()).await?;
    let auth_layer = auth::create_auth_layer::<auth::AppRole>(&config);
    let cors_layer = cors::cors_layer(&config)?;

    let app = router::app_router(auth_layer)
        .layer(cors_layer)
        .layer(trace::TraceLayer::new_for_http())
        .with_state(state);

    #[cfg(feature = "api-doc")]
    let app = {
        use api::openapi;
        use axum::routing;

        app.merge(
            axum::Router::new().route("/scalar", routing::get(openapi::scalar))
        )
    };

    let listener = tokio::net::TcpListener::bind(config.addrs()).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

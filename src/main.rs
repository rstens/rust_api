use axum::{routing::{get, post}, Router, serve};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

use rust_api_template::config::AppConfig;
use rust_api_template::db::{connect_with_retry, DbState};
use rust_api_template::routes::{create_user, get_users, health_check};

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("create SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("create SIGTERM handler");
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        let _ = signal::ctrl_c().await;
    }
    tracing::info!("🔻 Shutdown signal received");
}

#[tokio::main]
async fn main() -> Result<(), rust_api_template::errors::AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::from_env();
    let pool = connect_with_retry(&config.database_url).await?;

    let state = DbState { pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(get_users))
        .with_state(state)
        .layer(TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO)))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id());

    let listener = TcpListener::bind(&config.server_addr).await?;
    info!("🚀 API running at {}", config.server_addr);

    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

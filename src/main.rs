mod config;
mod db;
mod errors;
mod routes;

use axum::{Router, routing::{get, post}};
use tower_http::trace::{TraceLayer, DefaultOnResponse};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tracing::{info, Level};
use hyper::server::Server;
use crate::{
    config::AppConfig,
    db::{DbState, connect_with_retry},
    routes::{health_check, create_user, get_users},
    errors,
};

#[tokio::main]
async fn main() -> Result<(), errors::AppError> {
    let config = AppConfig::from_env();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .json()
        .init();

    let pool = connect_with_retry(&config.database_url).await?;
    let state = DbState { pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(get_users))
        .with_state(state)
        .layer(TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO)))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id());

    info!("🚀 API running at {}", config.server_addr);

    Server::bind(&config.server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

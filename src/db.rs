use crate::errors::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct DbState {
    pub pool: PgPool,
}

pub async fn connect_with_retry(db_url: &str) -> Result<PgPool, AppError> {
    let strategy = ExponentialBackoff::from_millis(300).take(6);
    let pool = Retry::spawn(strategy, || async {
        match PgPoolOptions::new().max_connections(10).connect(db_url).await {
            Ok(pool) => { info!("✅ Connected to Postgres"); Ok(pool) }
            Err(err) => { warn!("DB connection failed: {}. Retrying...", err); Err(err) }
        }
    }).await?;
    Ok(pool)
}

pub struct User {
    pub id: Uuid,
    pub name: String,
}

pub async fn insert_user(pool: &PgPool, name: &str) -> Result<User, AppError> {
    let rec = sqlx::query!(r#"INSERT INTO users (id, name) VALUES ($1, $2) RETURNING id, name"#,
        Uuid::new_v4(), name).fetch_one(pool).await?;
    Ok(User { id: rec.id, name: rec.name })
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    let rows = sqlx::query!(r#"SELECT id, name FROM users ORDER BY name"#)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| User { id: r.id, name: r.name }).collect())
}

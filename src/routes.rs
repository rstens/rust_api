use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db, db::DbState, errors::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn create_user(
    State(state): State<DbState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = db::insert_user(&state.pool, &payload.name).await?;
    Ok(Json(UserResponse { id: user.id, name: user.name }))
}

pub async fn get_users(
    State(state): State<DbState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = db::list_users(&state.pool).await?;
    Ok(Json(
        users
            .into_iter()
            .map(|u| UserResponse { id: u.id, name: u.name })
            .collect(),
    ))
}

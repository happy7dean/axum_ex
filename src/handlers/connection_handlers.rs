use crate::db::connection_manager::ConnectionManager;
use crate::db::types::{ConnectionInfo, PoolOptions as DbPoolOptions, DatabaseType};
use crate::error::AppError;
use axum::response::IntoResponse;
use axum::{Extension, Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ConnectionRequest {
    pub db_type: String,
    pub connection_string: String,
    #[serde(default)]
    pub pool_options: DbPoolOptions,
}

#[derive(Debug, Deserialize, Default)]
pub struct PoolOptions {
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout_seconds: u64,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_seconds: u64,
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime_seconds: u64,
}

fn default_max_connections() -> u32 {
    5
}

fn default_min_connections() -> u32 {
    0
}

fn default_acquire_timeout() -> u64 {
    30
}

fn default_idle_timeout() -> u64 {
    300
}

fn default_max_lifetime() -> u64 {
    1800
}

#[derive(Debug, Deserialize)]
pub struct ConnectionIdRequest {
    pub connection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub db_type: String,
    pub connection_string: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pool_options: DbPoolOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub id: String,
    pub connection_string: String,
}

#[axum::debug_handler]
pub async fn create_connection(
    State(manager): State<ConnectionManager>,
    Json(payload): Json<CreateConnectionRequest>,
) -> Result<Json<ConnectionResponse>, AppError> {
    let db_type = match payload.db_type.to_uppercase().as_str() {
        "POSTGRESQL" => DatabaseType::PostgreSQL,
        "ORACLE" => DatabaseType::Oracle,
        "MYSQL" => DatabaseType::MySQL,
        "MSSQL" => DatabaseType::MSSQL,
        "REDIS" => DatabaseType::Redis,
        _ => return Err(AppError::validation_error("Unsupported database type".into())),
    };

    let connection_string = payload.connection_string.clone();
    let connection_info = ConnectionInfo {
        db_type,
        connection_string: payload.connection_string,
        username: payload.username,
        password: payload.password,
        pool_options: payload.pool_options,
    };

    let id = manager.add_connection(connection_info).await?;
    
    Ok(Json(ConnectionResponse {
        id,
        connection_string,
    }))
}

pub async fn delete_connection(
    Extension(manager): Extension<ConnectionManager>,
    Json(payload): Json<ConnectionIdRequest>,
) -> Result<impl IntoResponse, AppError> {
    manager.remove_connection(&payload.connection_id).await;
    Ok(Json(ConnectionResponse { 
        id: payload.connection_id,
        connection_string: String::new() 
    }))
}

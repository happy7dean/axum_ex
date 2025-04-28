use crate::db::connection_manager::ConnectionManager;
use crate::db::connection::Connection;
use crate::error::AppError;
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    pub query: String,
    pub connection_id: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub rows: Vec<serde_json::Value>,
    pub affected_rows: Option<u64>,
}

#[axum::debug_handler]
pub async fn execute_sql(
    State(manager): State<ConnectionManager>,
    Json(payload): Json<SqlQuery>,
) -> Result<impl IntoResponse, AppError> {
    info!("Executing SQL query with connection ID: {}", payload.connection_id);
    let connection = manager
        .get_connection(&payload.connection_id)
        .await
        .ok_or_else(|| AppError::validation_error("Invalid connection ID".into()))?;

    // Execute the query
    let results = connection
        .execute_query(&payload.query)
        .await
        .map_err(|e| AppError::database_error(e.to_string()))?;

    Ok(Json(QueryResult { 
        rows: results,
        affected_rows: None, // This will be implemented when we add support for non-SELECT queries
    }))
} 
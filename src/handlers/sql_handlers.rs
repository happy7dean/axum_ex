use crate::db::connection_manager::ConnectionManager;
use crate::db::connection::Connection;
use crate::error::AppError;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    pub query: String,
    pub connection_id: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub rows: Vec<serde_json::Value>,
}

#[axum::debug_handler]
pub async fn execute_sql(
    Extension(manager): Extension<ConnectionManager>,
    Json(payload): Json<SqlQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Get the connection for the given ID
    let connection = manager
        .get_connection(&payload.connection_id)
        .await
        .ok_or_else(|| AppError::validation_error("Invalid connection ID".into()))?;

    // Validate that the query is a SELECT statement
    if !payload.query.trim().to_uppercase().starts_with("SELECT") {
        return Err(AppError::validation_error(
            "Only SELECT queries are allowed".into(),
        ));
    }

    // Execute the query
    let results = connection
        .execute_query(&payload.query)
        .await
        .map_err(|e| AppError::database_error(e.to_string()))?;

    Ok(Json(QueryResult { rows: results }))
} 
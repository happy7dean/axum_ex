use crate::error::AppError;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row, Column, TypeInfo};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<HashMap<String, serde_json::Value>>,
}

pub async fn execute_sql(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<SqlQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Validate that the query is a SELECT statement
    if !payload.query.trim().to_uppercase().starts_with("SELECT") {
        return Err(AppError::ValidationError(
            "Only SELECT queries are allowed".into(),
        ));
    }

    // Execute the query
    let rows = sqlx::query(&payload.query)
        .fetch_all(&pool)
        .await?;

    // Get column names
    let columns: Vec<String> = if let Some(row) = rows.first() {
        row.columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect()
    } else {
        Vec::new()
    };

    // Convert rows to JSON
    let mut result_rows = Vec::new();
    for row in rows {
        let mut row_map = HashMap::new();
        for (i, column) in columns.iter().enumerate() {
            let value = match row.try_get::<i32, _>(i) {
                Ok(v) => serde_json::Value::Number(v.into()),
                Err(_) => match row.try_get::<String, _>(i) {
                    Ok(v) => serde_json::Value::String(v),
                    Err(_) => match row.try_get::<bool, _>(i) {
                        Ok(v) => serde_json::Value::Bool(v),
                        Err(_) => match row.try_get::<f64, _>(i) {
                            Ok(v) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap_or(0.into())),
                            Err(_) => serde_json::Value::Null,
                        },
                    },
                },
            };
            row_map.insert(column.clone(), value);
        }
        result_rows.push(row_map);
    }

    Ok(Json(QueryResult {
        columns,
        rows: result_rows,
    }))
} 
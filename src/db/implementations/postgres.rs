use sqlx::{postgres::PgPoolOptions, PgPool, Row, Column};
use crate::db::connection::{Connection, ConnectionConfig};
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct PostgresConnection {
    pool: PgPool,
}

impl PostgresConnection {
    pub async fn new(config: ConnectionConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_options.max_connections)
            .min_connections(config.pool_options.min_connections)
            .acquire_timeout(config.get_timeout_duration())
            .idle_timeout(config.get_idle_timeout())
            .max_lifetime(config.get_max_lifetime())
            .connect(&config.connection_string)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl Connection for PostgresConnection {
    async fn execute_query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        let mut results = Vec::new();
        for row in rows {
            let mut row_map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
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
                row_map.insert(column.name().to_string(), value);
            }
            results.push(serde_json::Value::Object(serde_json::Map::from_iter(row_map)));
        }

        Ok(results)
    }

    async fn close(&self) {
        self.pool.close().await;
    }
} 
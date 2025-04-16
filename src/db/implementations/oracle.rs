use oracle::Connection;
use crate::db::connection::{Connection as DbConnection, ConnectionConfig};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OracleConnection {
    conn: Connection,
}

impl OracleConnection {
    pub async fn new(config: ConnectionConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let conn = if let (Some(username), Some(password)) = (config.username, config.password) {
            Connection::connect(&username, &password, &config.connection_string)?
        } else {
            return Err("Oracle connection requires username and password".into());
        };
        
        Ok(Self { conn })
    }
}

#[async_trait::async_trait]
impl DbConnection for OracleConnection {
    async fn execute_query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut stmt = self.conn.statement(query).build()?;
        let rows = stmt.query(&[])?;
        let mut results = Vec::new();

        for row_result in rows {
            let row = row_result?;
            let mut row_map = HashMap::new();
            
            for i in 0..row.column_info().len() {
                let value = if let Ok(val) = row.get::<usize, i32>(i + 1) {
                    serde_json::Value::Number(val.into())
                } else if let Ok(val) = row.get::<usize, String>(i + 1) {
                    serde_json::Value::String(val)
                } else if let Ok(val) = row.get::<usize, bool>(i + 1) {
                    serde_json::Value::Bool(val)
                } else if let Ok(val) = row.get::<usize, f64>(i + 1) {
                    serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap_or(0.into()))
                } else {
                    serde_json::Value::Null
                };
                let column_name = format!("column_{}", i + 1);
                row_map.insert(column_name, value);
            }
            results.push(serde_json::Value::Object(serde_json::Map::from_iter(row_map)));
        }

        Ok(results)
    }

    async fn close(&self) {
        // Oracle connection will be closed when dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::types::PoolOptions;

    #[tokio::test]
    async fn test_oracle_connection_creation() {
        let config = ConnectionConfig::new(
            "localhost:1521/XEPDB1".to_string(),
            PoolOptions::default(),
        ).with_credentials("test_user".to_string(), "test_pass".to_string());

        let conn = OracleConnection::new(config).await;
        assert!(conn.is_err()); // Should fail because test credentials are invalid
    }

    #[tokio::test]
    async fn test_oracle_connection_without_credentials() {
        let config = ConnectionConfig::new(
            "localhost:1521/XEPDB1".to_string(),
            PoolOptions::default(),
        );

        let conn = OracleConnection::new(config).await;
        assert!(conn.is_err());
        assert_eq!(
            conn.unwrap_err().to_string(),
            "Oracle connection requires username and password"
        );
    }
} 
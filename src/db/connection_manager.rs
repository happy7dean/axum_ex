use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::db::types::{DatabaseType, ConnectionInfo};
use crate::db::connection::{Connection, ConnectionConfig, DatabaseConnection};
use crate::db::implementations::{
    postgres::PostgresConnection,
    oracle::OracleConnection,
    mysql::MySQLConnection,
    mssql::MSSQLConnection,
    // redis::RedisConnection,
};

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, DatabaseConnection>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_connection(
        &self,
        info: ConnectionInfo,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let connection = match info.db_type {
            DatabaseType::PostgreSQL => {
                let conn = PostgresConnection::new(ConnectionConfig::new(
                    info.connection_string,
                    info.pool_options,
                )).await?;
                DatabaseConnection::Postgres(conn)
            },
            DatabaseType::Oracle => {
                let mut config = ConnectionConfig::new(
                    info.connection_string,
                    info.pool_options,
                );
                if let (Some(username), Some(password)) = (info.username, info.password) {
                    config = config.with_credentials(username, password);
                }
                let conn = OracleConnection::new(config).await?;
                DatabaseConnection::Oracle(Arc::new(conn))
            },
            DatabaseType::MySQL => {
                let conn = MySQLConnection::new(ConnectionConfig::new(
                    info.connection_string,
                    info.pool_options,
                )).await?;
                DatabaseConnection::MySQL(conn)
            },
            DatabaseType::MSSQL => {
                let conn = MSSQLConnection::new(ConnectionConfig::new(
                    info.connection_string,
                    info.pool_options,
                )).await?;
                DatabaseConnection::MSSQL(conn)
            },
            // DatabaseType::Redis => {
            //     let conn = RedisConnection::new(crate::db::connection::ConnectionConfig::new(
            //         info.connection_string,
            //         info.pool_options,
            //     )).await?;
            //     Box::new(conn)
            // },
            _ => return Err("Unsupported database type".into()),
        };

        let id = Uuid::new_v4().to_string();
        let mut connections = self.connections.write().await;
        connections.insert(id.clone(), connection);

        Ok(id)
    }

    pub async fn get_connection(&self, id: &str) -> Option<Arc<DatabaseConnection>> {
        let connections = self.connections.read().await;
        connections.get(id).map(|conn| Arc::new(conn.clone()))
    }

    pub async fn remove_connection(&self, id: &str) {
        let mut connections = self.connections.write().await;
        if let Some(conn) = connections.remove(id) {
            conn.close().await;
        }
    }

    pub async fn with_connection<F, R>(&self, id: &str, f: F) -> Option<R>
    where
        F: for<'a> FnOnce(&'a DatabaseConnection) -> R + 'static,
    {
        let connections = self.connections.read().await;
        let conn = connections.get(id)?;
        Some(f(conn))
    }
} 
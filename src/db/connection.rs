use std::time::Duration;
use async_trait::async_trait;
use crate::db::types::{DatabaseType, PoolOptions};
use crate::db::implementations::{
    postgres::PostgresConnection,
    mysql::MySQLConnection,
    mssql::MSSQLConnection,
    oracle::OracleConnection,
};
use std::sync::Arc;

#[derive(Debug)]
pub enum DatabaseConnection {
    Postgres(PostgresConnection),
    MySQL(MySQLConnection),
    MSSQL(MSSQLConnection),
    Oracle(Arc<OracleConnection>),
}

impl Clone for DatabaseConnection {
    fn clone(&self) -> Self {
        match self {
            Self::Postgres(conn) => Self::Postgres(conn.clone()),
            Self::MySQL(conn) => Self::MySQL(conn.clone()),
            Self::MSSQL(conn) => Self::MSSQL(conn.clone()),
            Self::Oracle(conn) => Self::Oracle(conn.clone()),
        }
    }
}

#[async_trait]
pub trait Connection: Send + Sync {
    async fn execute_query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>>;
    async fn close(&self);
}

#[async_trait]
impl Connection for DatabaseConnection {
    async fn execute_query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::Postgres(conn) => conn.execute_query(query).await,
            Self::MySQL(conn) => conn.execute_query(query).await,
            Self::MSSQL(conn) => conn.execute_query(query).await,
            Self::Oracle(conn) => conn.execute_query(query).await,
        }
    }

    async fn close(&self) {
        match self {
            Self::Postgres(conn) => conn.close().await,
            Self::MySQL(conn) => conn.close().await,
            Self::MSSQL(conn) => conn.close().await,
            Self::Oracle(conn) => conn.close().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub connection_string: String,
    pub pool_options: PoolOptions,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ConnectionConfig {
    pub fn new(connection_string: String, pool_options: PoolOptions) -> Self {
        Self {
            connection_string,
            pool_options,
            username: None,
            password: None,
        }
    }

    pub fn with_credentials(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    pub fn get_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.pool_options.acquire_timeout_seconds)
    }

    pub fn get_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.pool_options.idle_timeout_seconds)
    }

    pub fn get_max_lifetime(&self) -> Duration {
        Duration::from_secs(self.pool_options.max_lifetime_seconds)
    }
} 
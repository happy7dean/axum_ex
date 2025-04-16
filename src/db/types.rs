use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DatabaseType {
    PostgreSQL,
    Oracle,
    MySQL,
    MSSQL,
    Redis,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionInfo {
    pub db_type: DatabaseType,
    pub connection_string: String,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default)]
    pub pool_options: PoolOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolOptions {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            max_connections: 5,
            min_connections: 0,
            acquire_timeout_seconds: 30,
            idle_timeout_seconds: 300,
            max_lifetime_seconds: 1800,
        }
    }
}

impl ConnectionInfo {
    pub fn new(connection_string: String, pool_options: PoolOptions) -> Self {
        Self {
            db_type: DatabaseType::PostgreSQL,
            connection_string,
            username: None,
            password: None,
            pool_options,
        }
    }

    pub fn with_credentials(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }
} 
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use crate::db::connection::{Connection, ConnectionConfig};
use std::collections::HashMap;
use futures::StreamExt;

#[derive(Debug,Clone)]
pub struct MSSQLConnection {
    config: Config,
}

impl MSSQLConnection {
    pub async fn new(config: ConnectionConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut tiberius_config = Config::from_ado_string(&config.connection_string)?;
        tiberius_config.trust_cert();

        Ok(Self { config: tiberius_config })
    }
}

#[async_trait::async_trait]
impl Connection for MSSQLConnection {
    async fn execute_query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();
        let tcp = TcpStream::connect(self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let mut client = Client::connect(self.config.clone(), tcp.compat_write()).await?;
        let mut stream = client.query(query, &[]).await?;
        let mut row_stream = stream.into_row_stream();

        while let Some(row_result) = row_stream.next().await {
            let row = row_result?;
            let mut row_map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
                let value = match row.get::<i32, _>(i) {
                    Some(v) => serde_json::Value::Number(v.into()),
                    None => match row.get::<&str, _>(i) {
                        Some(v) => serde_json::Value::String(v.to_string()),
                        None => match row.get::<bool, _>(i) {
                            Some(v) => serde_json::Value::Bool(v),
                            None => match row.get::<f64, _>(i) {
                                Some(v) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap_or(0.into())),
                                None => serde_json::Value::Null,
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
        // Tiberius client doesn't have an explicit close method
    }
} 
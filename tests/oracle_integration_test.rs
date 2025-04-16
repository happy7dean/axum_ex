use axum_ex::db::{
    connection::{ConnectionConfig, DatabaseConnection, Connection},
    connection_manager::ConnectionManager,
    types::{ConnectionInfo, DatabaseType, PoolOptions},
};

#[tokio::test]
async fn test_oracle_connection_manager_integration() {
    let manager = ConnectionManager::new();

    let connection_info = ConnectionInfo {
        db_type: DatabaseType::Oracle,
        connection_string: "mock_connection_string".to_string(),
        username: Some("dean".to_string()),
        password: Some("dd".to_string()),
        pool_options: PoolOptions::default(),
    };

    let result = manager.add_connection(connection_info).await;
    assert!(result.is_ok());

    let connection_id = result.unwrap();
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some());

    // Clean up
    manager.remove_connection(&connection_id).await;
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_none());
}

#[tokio::test]
async fn test_oracle_query_execution_integration() {
    let manager = ConnectionManager::new();

    let connection_info = ConnectionInfo {
        db_type: DatabaseType::Oracle,
        connection_string: "mock_connection_string".to_string(),
        username: Some("test_user".to_string()),
        password: Some("test_pass".to_string()),
        pool_options: PoolOptions::default(),
    };

    let connection_id = manager.add_connection(connection_info).await.unwrap();
    let connection = manager.get_connection(&connection_id).await.unwrap();

    let result = connection.execute_query("SELECT * FROM test_table").await;
    assert!(result.is_ok());

    // Clean up
    manager.remove_connection(&connection_id).await;
}

#[tokio::test]
async fn test_oracle_connection_pool_integration() {
    let manager = ConnectionManager::new();

    let pool_options = PoolOptions {
        max_connections: 10,
        min_connections: 2,
        acquire_timeout_seconds: 60,
        idle_timeout_seconds: 600,
        max_lifetime_seconds: 3600,
    };

    let connection_info = ConnectionInfo {
        db_type: DatabaseType::Oracle,
        connection_string: "mock_connection_string".to_string(),
        username: Some("test_user".to_string()),
        password: Some("test_pass".to_string()),
        pool_options,
    };

    let result = manager.add_connection(connection_info).await;
    assert!(result.is_ok());

    let connection_id = result.unwrap();
    manager.remove_connection(&connection_id).await;
} 
use axum_ex::db::{
    connection::Connection,
    connection_manager::ConnectionManager,
    types::{ConnectionInfo, DatabaseType, PoolOptions},
};

#[tokio::test]
async fn test_postgres_connection() {
    let manager = ConnectionManager::new();

    let connection_info = ConnectionInfo {
        db_type: DatabaseType::PostgreSQL,
        connection_string: "postgres://admin:admin_password@localhost:5432/dvdrental".to_string(),
        username: None,
        password: None,
        pool_options: PoolOptions::default(),
    };

    // Test connection creation
    let result = manager.add_connection(connection_info).await;
    assert!(result.is_ok(), "Failed to create PostgreSQL connection: {:?}", result.err());

    let connection_id = result.unwrap();
    
    // Test query execution
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some(), "Failed to get PostgreSQL connection");
    
    let query_result = connection.unwrap().execute_query("SELECT 1").await;
    assert!(query_result.is_ok(), "Failed to execute PostgreSQL query: {:?}", query_result.err());

    // Test connection removal
    manager.remove_connection(&connection_id).await;
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_none(), "Connection should be removed");
} 
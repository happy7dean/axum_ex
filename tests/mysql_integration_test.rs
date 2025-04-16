use axum_ex::db::{
    connection::Connection,
    connection_manager::ConnectionManager,
    types::{ConnectionInfo, DatabaseType, PoolOptions},
};

#[tokio::test]
async fn test_mysql_connection() {
    let manager = ConnectionManager::new();

    let connection_info = ConnectionInfo {
        db_type: DatabaseType::MySQL,
        connection_string: "mysql://root:dd@localhost:3306/test".to_string(),
        username: None,
        password: None,
        pool_options: PoolOptions::default(),
    };

    // Test connection creation
    let result = manager.add_connection(connection_info).await;
    assert!(result.is_ok(), "Failed to create MySQL connection: {:?}", result.err());

    let connection_id = result.unwrap();
    
    // Test getting connection
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some(), "Failed to get MySQL connection");

    // Test query execution
    let connection = connection.unwrap();
    let result = connection.execute_query("SELECT 1").await;
    assert!(result.is_ok(), "Failed to execute MySQL query: {:?}", result.err());

    // Test connection removal
    manager.remove_connection(&connection_id).await;
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_none(), "Connection should be removed");
} 
use axum_ex::db::{
    connection::Connection,
    connection_manager::ConnectionManager,
    types::{ConnectionInfo, DatabaseType, PoolOptions},
};

#[tokio::test]
async fn test_sql_execution() {
    let manager = ConnectionManager::new();

    // Create a PostgreSQL connection
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
    
    // Test SELECT query
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some(), "Failed to get PostgreSQL connection");
    
    let query_result = connection.as_ref().unwrap().execute_query("SELECT 1 as test").await;
    assert!(query_result.is_ok(), "Failed to execute SELECT query: {:?}", query_result.err());
    let results = query_result.unwrap();
    assert!(!results.is_empty(), "SELECT query should return results");

    // Test INSERT query
    let query_result = connection.as_ref().unwrap().execute_query(
        "INSERT INTO test_table (name) VALUES ('test') RETURNING id"
    ).await;
    assert!(query_result.is_ok(), "Failed to execute INSERT query: {:?}", query_result.err());

    // Test UPDATE query
    let query_result = connection.as_ref().unwrap().execute_query(
        "UPDATE test_table SET name = 'updated' WHERE name = 'test' RETURNING id"
    ).await;
    assert!(query_result.is_ok(), "Failed to execute UPDATE query: {:?}", query_result.err());

    // Test DELETE query
    let query_result = connection.as_ref().unwrap().execute_query(
        "DELETE FROM test_table WHERE name = 'updated' RETURNING id"
    ).await;
    assert!(query_result.is_ok(), "Failed to execute DELETE query: {:?}", query_result.err());

    // Clean up
    manager.remove_connection(&connection_id).await;
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_none(), "Connection should be removed");
} 
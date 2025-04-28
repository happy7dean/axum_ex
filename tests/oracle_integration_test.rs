use axum_ex::db::{
    connection::{ConnectionConfig, DatabaseConnection, Connection},
    connection_manager::ConnectionManager,
    types::{ConnectionInfo, DatabaseType, PoolOptions},
};
use std::env;

// Docker 환경 변수에서 연결 정보를 가져오는 함수
fn get_oracle_connection_info() -> ConnectionInfo {
    let host = env::var("ORACLE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("ORACLE_PORT").unwrap_or_else(|_| "1521".to_string());
    let service = env::var("ORACLE_SERVICE").unwrap_or_else(|_| "XE".to_string());
    let username = env::var("ORACLE_USER").unwrap_or_else(|_| "system".to_string());
    let password = env::var("ORACLE_PASSWORD").unwrap_or_else(|_| "oracle".to_string());

    let connection_string = format!("{}:{}/{}", host, port, service);

    ConnectionInfo {
        db_type: DatabaseType::Oracle,
        connection_string,
        username: Some(username),
        password: Some(password),
        pool_options: PoolOptions::default(),
    }
}

#[tokio::test]
async fn test_oracle_connection_manager_integration() {
    let manager = ConnectionManager::new();
    let connection_info = get_oracle_connection_info();

    let result = manager.add_connection(connection_info).await;
    if let Err(e) = &result {
        println!("Connection error: {}", e);
    }
    assert!(result.is_ok(), "Failed to create Oracle connection: {:?}", result.err());

    let connection_id = result.unwrap();
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some(), "Failed to get Oracle connection");

    // Clean up
    manager.remove_connection(&connection_id).await;
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_none(), "Connection should be removed");
}

#[tokio::test]
async fn test_oracle_query_execution_integration() {
    let manager = ConnectionManager::new();
    let connection_info = get_oracle_connection_info();

    let result = manager.add_connection(connection_info).await;
    if let Err(e) = &result {
        println!("Connection error: {}", e);
    }
    assert!(result.is_ok(), "Failed to create Oracle connection: {:?}", result.err());

    let connection_id = result.unwrap();
    let connection = manager.get_connection(&connection_id).await;
    assert!(connection.is_some(), "Failed to get Oracle connection");

    // 테스트용 테이블 생성
    let create_table = connection.as_ref().unwrap().execute_query(
        "CREATE TABLE test_table (id NUMBER PRIMARY KEY, name VARCHAR2(100))"
    ).await;
    if let Err(e) = &create_table {
        println!("Table creation error: {}", e);
    }

    // 데이터 삽입
    let insert = connection.as_ref().unwrap().execute_query(
        "INSERT INTO test_table VALUES (1, 'test')"
    ).await;
    if let Err(e) = &insert {
        println!("Insert error: {}", e);
    }

    // 데이터 조회
    let result = connection.as_ref().unwrap().execute_query("SELECT * FROM test_table").await;
    if let Err(e) = &result {
        println!("Query execution error: {}", e);
    }
    assert!(result.is_ok(), "Failed to execute query: {:?}", result.err());

    // 테이블 삭제
    let drop_table = connection.as_ref().unwrap().execute_query("DROP TABLE test_table").await;
    if let Err(e) = &drop_table {
        println!("Table drop error: {}", e);
    }

    // Clean up
    manager.remove_connection(&connection_id).await;
}

#[tokio::test]
async fn test_oracle_connection_pool_integration() {
    let manager = ConnectionManager::new();
    let connection_info = get_oracle_connection_info();

    let pool_options = PoolOptions {
        max_connections: 10,
        min_connections: 2,
        acquire_timeout_seconds: 60,
        idle_timeout_seconds: 600,
        max_lifetime_seconds: 3600,
    };

    let result = manager.add_connection(connection_info).await;
    if let Err(e) = &result {
        println!("Connection error: {}", e);
    }
    assert!(result.is_ok(), "Failed to create Oracle connection: {:?}", result.err());

    let connection_id = result.unwrap();
    manager.remove_connection(&connection_id).await;
} 
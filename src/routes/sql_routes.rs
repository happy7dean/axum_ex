use crate::handlers::sql_handlers::execute_sql;
use crate::db::connection_manager::ConnectionManager;
use axum::{
    routing::post,
    Router,
};

pub fn create_routes() -> Router<ConnectionManager> {
    Router::new()
        .route("/sql", post(execute_sql))
} 
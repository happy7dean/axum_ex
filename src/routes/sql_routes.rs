use crate::handlers::sql_handlers::execute_sql;
use axum::{
    routing::post,
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/sql", post(execute_sql))
} 
use crate::handlers::connection_handlers::create_connection;
use crate::db::connection_manager::ConnectionManager;
use axum::{
    routing::post,
    Router,
};

pub fn create_routes() -> Router<ConnectionManager> {
    Router::new()
        .route("/connect", post(create_connection))
} 
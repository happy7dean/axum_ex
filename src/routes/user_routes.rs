use crate::handlers::user_handlers::{create_user, delete_user, get_user, get_users, update_user};
use axum::{
    routing::{get, post},
    Router,
}; // 적절한 핸들러 임포트

pub fn create_routes() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
}

use crate::handlers::book_handles::{create_book, delete_book, get_book, get_books, update_book};
use crate::db::connection_manager::ConnectionManager;
use axum::{
    routing::{
        get, 
        // post,
        // put,
        // delete,
    },
    Router,
}; // 적절한 핸들러 임포트

pub fn create_routes() -> Router<ConnectionManager> {
    Router::new()
        .route("/books", get(get_books).post(create_book))
        .route(
            "/books/{id}",
            get(get_book).put(update_book).delete(delete_book),
        )
}

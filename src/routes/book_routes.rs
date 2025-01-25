use crate::handlers::book_handles::{create_book, delete_book, get_book, get_books, update_book};
use axum::{
    routing::{get, post},
    Router,
}; // 적절한 핸들러 임포트

pub fn create_routes() -> Router {
    Router::new()
        .route("/books", get(get_books).post(create_book))
        .route(
            "/books/:id",
            get(get_book).put(update_book).delete(delete_book),
        )
}

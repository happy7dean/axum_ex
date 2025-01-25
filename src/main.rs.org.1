use axum::{
    http::StatusCode,
    response::{Response, Result},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use dotenv::dotenv;
use std::env;

mod db;
mod error;
mod handlers;
mod models;

use db::create_pool;
use handlers::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = create_pool().await?;

    let app = Router::new()
        .route("/books", get(get_books).post(create_book))
        .route(
            "/books/{id}",
            get(get_book).put(update_book).delete(delete_book),
        )
        .layer(Extension(pool));

    println!("서버가 0.0.0.0:3000에서 시작됩니다...");

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

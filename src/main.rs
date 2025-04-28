use axum::{
    // http::StatusCode,
    response::Result,
    // response::{Response, Result},
    // routing::{delete, get, post, put},
    extract::State,
    // Json, Router,
};
use dotenv::dotenv;
// use std::{env, io};

mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod utils; // 모듈 임포트

use db::connection_manager::ConnectionManager;
//use handlers::*;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt,
    // layer::{self, SubscriberExt},
    // util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let info_file = rolling::daily("./logs", "info").with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_target(false)
        .pretty()
        .json()
        // .with_writer(io::stdout)
        .with_max_level(tracing::Level::TRACE)
        .with_writer(info_file)
        .with_ansi(false)
        .init();

    let connection_manager = ConnectionManager::new();

    let app = routes::create_routes()
        .with_state(connection_manager)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    println!("서버가 0.0.0.0:3000에서 시작됩니다...");
    tracing::info!("listening on {}", "locahost:3000");

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

use axum::{
    http::StatusCode,
    response::{Response, Result},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use dotenv::dotenv;
use std::{env, io};

mod db;
mod error;
mod handlers;
mod models;
mod routes; // 모듈 임포트

use db::create_pool;
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
    let pool = create_pool().await?;

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

    let app = routes::create_routes().layer(Extension(pool)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    ); // 라우트를 앱에 추가

    println!("서버가 0.0.0.0:3000에서 시작됩니다...");
    tracing::info!("listening on {}", "locahost:3000");

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

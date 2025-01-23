use dotenv::dotenv;
use std::env;

use sqlx::{postgres::PgPoolOptions, query_as};

use axum::{
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // .env 파일 로드
    dotenv().ok();

    // 환경 변수 가져오기
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("데이터베이스 연결 : {}", database_url);

    // initialize tracing
    tracing_subscriber::fmt::init();

    // 데이터베이스 연결 풀 생성
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/books", get(get_books).post(create_book))
        .route(
            "/books/:id",
            get(get_book).put(update_book).delete(delete_book),
        )
        .layer(Extension(pool));

    println!("서버가 0.0.0.0:3000에서 시작됩니다...");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //         .serve(app.into_make_service())
    //         .await
    //         .unwrap();

    Ok(())
}

// 데이터 모델 정의

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    id: i32,
    title: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct CreateBook {
    title: String,
    author: String,
}

// 핸들러 함수 구현

use axum::response::IntoResponse;
use axum::{Extension, Json};
use sqlx::PgPool;

// 등록
async fn create_book(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateBook>,
) -> Result<impl IntoResponse> {
    let rec = query_as!(
        Book,
        "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id, title, author",
        &payload.title,
        &payload.author
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("데이터베이스 오류: {}", e),
        )
    })?;

    // let rec = sqlx::query_as::<_, Book>(
    //     "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id, title, author",
    // )
    // .bind(&payload.title)
    // .bind(&payload.author)
    // .fetch_one(&pool)
    // .await
    // .map_err(|e| {
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         format!("데이터베이스 오류: {}", e),
    //     )
    // })?;

    Ok((StatusCode::CREATED, Json(rec)))
    // Ok(<(StatusCode::CREATED, Json(rec))>::Result)
}

// 모두 검색
async fn get_books(
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let recs = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("데이터베이스 오류: {}", e),
            )
        })?;

    Ok(Json(recs))
}

use axum::extract::Path;
// 특정북 검색
async fn get_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rec = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                format!("책을 찾을 수 없습니다: {}", e),
            )
        })?;

    Ok(Json(rec))
}

// 수정
async fn update_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateBook>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rec = sqlx::query_as::<_, Book>(
        "UPDATE books SET title = $1, author = $2 WHERE id = $3 RETURNING id, title, author",
    )
    .bind(&payload.title)
    .bind(&payload.author)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("책을 업데이트할 수 없습니다: {}", e),
        )
    })?;

    Ok(Json(rec))
}

// 삭제
async fn delete_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("책을 삭제할 수 없습니다: {}", e),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, format!("책을 찾을 수 없습니다.")));
    }

    Ok(StatusCode::NO_CONTENT)
}

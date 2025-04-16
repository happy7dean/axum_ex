use crate::error::AppError;
use crate::models::book_models::{Book, CreateBook};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Path, Extension, Json};
use sqlx::{query_as, PgPool};

pub async fn create_book(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateBook>,
) -> Result<impl IntoResponse, AppError> {
    if payload.title.is_empty() || payload.author.is_empty() {
        return Err(AppError::validation_error(
            "제목과 저자는 필수 항목입니다.".into(),
        ));
    }
    // 트랜젝션
    // let mut tx = pool.begin().await?;
    let rec = query_as::<_, Book>(
        "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id, title, author",
    )
    .bind(&payload.title)
    .bind(&payload.author)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(rec)))
}

pub async fn get_books(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, AppError> {
    let recs: Vec<_> = query_as::<_, Book>("SELECT id, title, author FROM books")
        .fetch_all(&pool)
        .await?;
    Ok(Json(recs))
}

pub async fn get_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let rec = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(rec))
}

pub async fn update_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateBook>,
) -> Result<impl IntoResponse, AppError> {
    if payload.title.is_empty() || payload.author.is_empty() {
        return Err(AppError::validation_error(
            "제목과 저자는 필수 항목입니다.".into(),
        ));
    }
    let rec = sqlx::query_as::<_, Book>(
        "UPDATE books SET title = $1, author = $2 WHERE id = $3 RETURNING id, title, author",
    )
    .bind(&payload.title)
    .bind(&payload.author)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(rec))
}

pub async fn delete_book(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::validation_error("책을 찾을 수 없습니다.".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

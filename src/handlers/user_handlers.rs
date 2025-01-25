use crate::error::AppError;
use crate::models::user_models::{CreateUser, User};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Path, Extension, Json};
use sqlx::{query_as, PgPool};

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(AppError::ValidationError(
            "이름과 이메일은 필수 항목입니다.".into(),
        ));
    }
    // 트랜젝션
    // let mut tx = pool.begin().await?;
    let rec = query_as::<_, User>(
        "INSERT INTO users (name, email, passwd) VALUES ($1, $2, $3) RETURNING id, name, email, passwd",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.passwd)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(rec)))
}

pub async fn get_users(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, AppError> {
    let recs: Vec<_> = query_as::<_, User>("SELECT id, name, email, passwd FROM users")
        .fetch_all(&pool)
        .await?;
    Ok(Json(recs))
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let rec = sqlx::query_as::<_, User>("SELECT id, name, email, passwd FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(rec))
}

pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(AppError::ValidationError(
            "제목과 저자는 필수 항목입니다.".into(),
        ));
    }
    let rec = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, email = $2, passwd = $3 WHERE id = $4 RETURNING id, name, email, passwd",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.passwd)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(rec))
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::DataNotFoundError("책을 찾을 수 없습니다.".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

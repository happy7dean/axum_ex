use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl AppError {
    pub fn validation_error(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::BAD_REQUEST,
        }
    }

    pub fn database_error(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::database_error(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::database_error(error.to_string())
    }
}

mod book_routes;
mod connection_routes;
mod sql_routes;
mod user_routes; // 예시로 다른 라우트 모듈을 추가할 수 있음.

use axum::Router;
use crate::db::connection_manager::ConnectionManager;

pub fn create_routes() -> Router<ConnectionManager> {
    Router::new()
        .merge(book_routes::create_routes())
        .merge(user_routes::create_routes())
        .merge(sql_routes::create_routes())
        .merge(connection_routes::create_routes())
    // let book_routes = book_routes::create_routes();
    // let user_routes = user_routes::create_routes(); // 추가한 경우

    // // 다른 라우트를 조합할 수 있음
    // book_routes.merge(user_routes)
}

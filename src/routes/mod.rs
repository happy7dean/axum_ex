pub mod book_routes;
pub mod user_routes; // 예시로 다른 라우트 모듈을 추가할 수 있음.

use axum::Router;

pub fn create_routes() -> Router {
    book_routes::create_routes().merge(user_routes::create_routes())
    // let book_routes = book_routes::create_routes();
    // let user_routes = user_routes::create_routes(); // 추가한 경우

    // // 다른 라우트를 조합할 수 있음
    // book_routes.merge(user_routes)
}

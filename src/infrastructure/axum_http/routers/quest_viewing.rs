use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{
    domain::value_objects::board_checking_filter::BoardCheckingFilter,
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .route("/:id", get(view_details))
        .route("/board-checking", get(board_checking))
}

pub async fn view_details(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "View Details").into_response()
}

pub async fn board_checking(filter: Query<BoardCheckingFilter>) -> impl IntoResponse {
    println!("{:?}", filter);
    (StatusCode::OK, "Board Checking").into_response()
}

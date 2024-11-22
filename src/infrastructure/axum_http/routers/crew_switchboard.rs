use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::infrastructure::postgres::postgres_connector::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .route("/join", post(join))
        .route("/leave", post(leave))
}

pub async fn join() -> impl IntoResponse {
    (StatusCode::CREATED, "Join").into_response()
}

pub async fn leave() -> impl IntoResponse {
    (StatusCode::OK, "Leave").into_response()
}

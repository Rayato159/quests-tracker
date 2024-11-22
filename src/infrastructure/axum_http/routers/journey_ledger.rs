use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::patch, Router};

use crate::infrastructure::postgres::postgres_connector::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .route("/in-journey/:id", patch(in_journey))
        .route("/to-completed/:id", patch(to_completed))
        .route("/to-failed/:id", patch(to_failed))
}

pub async fn in_journey(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "In Journey").into_response()
}

pub async fn to_completed(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "To Completed").into_response()
}

pub async fn to_failed(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "To Failed").into_response()
}

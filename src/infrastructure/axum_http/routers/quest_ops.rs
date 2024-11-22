use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Router,
};

use crate::infrastructure::postgres::postgres_connector::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .route("/", post(add))
        .route("/:id", patch(edit))
        .route("/:id", delete(remove))
}

pub async fn add() -> impl IntoResponse {
    (StatusCode::CREATED, "Add Quest").into_response()
}

pub async fn edit(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "Edit Quest").into_response()
}

pub async fn remove(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "Remove Quest").into_response()
}

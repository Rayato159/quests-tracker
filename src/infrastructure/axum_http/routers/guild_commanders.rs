use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::infrastructure::postgres::postgres_connector::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new().route("/", post(register))
}

pub async fn register() -> impl IntoResponse {
    (StatusCode::CREATED, "Register").into_response()
}

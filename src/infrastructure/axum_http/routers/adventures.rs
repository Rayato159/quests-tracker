use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::adventurers::AdventurersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurersPostgres::new(db_pool);
    let adventures_use_case = AdventurersUseCase::new(Arc::new(adventurers_repository));

    Router::new().route("/", post(register))
}

pub async fn register() -> impl IntoResponse {
    (StatusCode::CREATED, "Register").into_response()
}

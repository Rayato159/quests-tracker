use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::{
        hashing::{argon2_hashing::Argon2Hashing, Hashing},
        postgres::{
            postgres_connector::PgPoolSquad, repositories::adventurers::AdventurersPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurersPostgres::new(db_pool);
    let hashing = Argon2Hashing;
    let adventures_use_case =
        AdventurersUseCase::new(Arc::new(adventurers_repository), Arc::new(hashing));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventures_use_case))
}

pub async fn register<T1, T2>(
    State(adventures_use_case): State<Arc<AdventurersUseCase<T1, T2>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: Hashing + Send + Sync,
{
    match adventures_use_case
        .register(register_adventurer_model)
        .await
    {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

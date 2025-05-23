use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
    Extension, Router,
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
        transaction_provider::TransactionProvider,
    },
    infrastructure::{
        axum_http::middleware::adventurers_authorization,
        postgres::{
            postgres_connector::PgPoolSquad,
            repositories::{
                crew_switchboard::CrewSwitchboardPostgres, diesel_transaction::DieselTransaction,
                quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let tx = DieselTransaction::new(Arc::clone(&db_pool));

    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
        Arc::new(tx),
    );

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .route("/transaction-test/:quest_id", post(transaction_test))
        .route_layer(middleware::from_fn(adventurers_authorization))
        .with_state(Arc::new(crew_switchboard_use_case))
}

pub async fn join<T1, T2, T3>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2, T3>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync + 'static,
    T2: QuestViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match crew_switchboard_use_case
        .join(quest_id, adventurer_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {}, has joined quest id: {}",
                adventurer_id, quest_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn leave<T1, T2, T3>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2, T3>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync + 'static,
    T2: QuestViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match crew_switchboard_use_case
        .leave(quest_id, adventurer_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {}, has leaved quest id: {}",
                adventurer_id, quest_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn transaction_test<T1, T2, T3>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2, T3>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync + 'static,
    T2: QuestViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match crew_switchboard_use_case
        .join_and_delete_transaction(quest_id, adventurer_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {}, has join and leave quest id: {}",
                adventurer_id, quest_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

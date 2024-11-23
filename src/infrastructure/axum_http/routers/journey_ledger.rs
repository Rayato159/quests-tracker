use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
    Extension, Router,
};

use crate::{
    application::usecases::journey_ledger::JourneyLedgerUseCase,
    domain::{
        repositories::{
            journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
        },
        value_objects::quest_statuses::QuestStatuses,
    },
    infrastructure::{
        axum_http::middleware::guild_commanders_authorization,
        postgres::{
            postgres_connector::PgPoolSquad,
            repositories::{
                journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_use_case = JourneyLedgerUseCase::new(
        Arc::new(journey_ledger_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_completed))
        .route("/to-failed/:quest_id", patch(to_failed))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(journey_ledger_use_case))
}

pub async fn in_journey<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_use_case
        .in_journey(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => (
            StatusCode::OK,
            format!(
                "Quest id: {} is now {:?}",
                quest_id,
                QuestStatuses::InJourney
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_completed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_use_case
        .to_completed(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => (
            StatusCode::OK,
            format!(
                "Quest id: {} is now {:?}",
                quest_id,
                QuestStatuses::Completed
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match journey_ledger_use_case
        .to_failed(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => (
            StatusCode::OK,
            format!("Quest id: {} is now {:?}", quest_id, QuestStatuses::Failed),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::{
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::quest_viewing::QuestViewingPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let quest_viewing_use_case = QuestViewingUseCase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:id", get(view_details))
        .route("/board-checking", get(board_checking))
        .with_state(Arc::new(quest_viewing_use_case))
}

pub async fn view_details<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_use_case.view_details(id).await {
        Ok(quest_model) => (StatusCode::OK, Json(quest_model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn board_checking<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    filter: Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_use_case.board_checking(&filter).await {
        Ok(quest_models) => (StatusCode::OK, Json(quest_models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

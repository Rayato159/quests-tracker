use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::{
        repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
        value_objects::quest_model::{AddQuestModel, EditQuestModel},
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad,
        repositories::{quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));

    let quest_ops_use_case = QuestOpsUseCase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository),
    );

    let quest_ops_artifact = Arc::new(quest_ops_use_case);

    Router::new()
        .route("/", post(add))
        .route("/:id", patch(edit))
        .route("/:id", delete(remove))
        .with_state(Arc::clone(&quest_ops_artifact))
}

pub async fn add<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Json(add_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case.add(add_quest_model).await {
        Ok(user_id) => {
            let response = format!("Add quest success with id: {}", user_id);
            (StatusCode::CREATED, response).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Add quest error").into_response(),
    }
}

pub async fn edit<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Path(id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case.edit(id, edit_quest_model).await {
        Ok(quest_id) => {
            let response = format!("Edit quest success with id: {}", quest_id);
            (StatusCode::OK, response).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Edit quest error").into_response(),
    }
}

pub async fn remove<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case.remove(id).await {
        Ok(_) => {
            let response = format!("Remove quest success with id: {}", id);
            (StatusCode::OK, response).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Remove quest error").into_response(),
    }
}

use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Router,
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
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

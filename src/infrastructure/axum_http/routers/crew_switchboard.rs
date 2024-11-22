use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad,
        repositories::{
            crew_switchboard::CrewSwitchboardPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));

    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/join", post(join))
        .route("/leave", post(leave))
}

pub async fn join() -> impl IntoResponse {
    (StatusCode::CREATED, "Join").into_response()
}

pub async fn leave() -> impl IntoResponse {
    (StatusCode::OK, "Leave").into_response()
}

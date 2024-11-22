use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::value_objects::board_checking_filter::BoardCheckingFilter,
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
}

pub async fn view_details(Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "View Details").into_response()
}

pub async fn board_checking(filter: Query<BoardCheckingFilter>) -> impl IntoResponse {
    println!("{:?}", filter);
    (StatusCode::OK, "Board Checking").into_response()
}

use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Path, Query, State},
};

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::value_objects::board_checking_filter::BoardCheckingFilter,
    infrastructure::postgres::repositories::quest_viewing::QuestViewingPostgres,
};

pub fn quest_viewing_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/quest-viewing")
            .service(board_checking)
            .service(view_details),
    );
}

#[web::get("/board-checking")]
pub async fn board_checking(
    quest_viewing_use_case: State<Arc<QuestViewingUseCase<QuestViewingPostgres>>>,
    query: Query<BoardCheckingFilter>,
) -> web::HttpResponse {
    match quest_viewing_use_case.board_checking(&query).await {
        Ok(quest_models) => web::HttpResponse::Ok().json(&quest_models),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::get("/{quest_id}")]
pub async fn view_details(
    quest_viewing_use_case: State<Arc<QuestViewingUseCase<QuestViewingPostgres>>>,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();

    match quest_viewing_use_case.view_details(quest_id).await {
        Ok(quest_model) => web::HttpResponse::Ok().json(&quest_model),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Path, State},
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    infrastructure::ntex_http::middleware::AdventurersAuthorization,
    infrastructure::postgres::repositories::{
        crew_switchboard::CrewSwitchboardPostgres, diesel_transaction::DieselTransaction,
        quest_viewing::QuestViewingPostgres,
    },
};

pub fn crew_switchboard_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/crew-switchboard")
            .wrap(AdventurersAuthorization)
            .service(join)
            .service(leave)
            .service(transaction_test),
    );
}

#[web::post("/join/{quest_id}")]
pub async fn join(
    req: web::HttpRequest,
    crew_switchboard_use_case: State<
        Arc<
            CrewSwitchboardUseCase<
                CrewSwitchboardPostgres,
                QuestViewingPostgres,
                DieselTransaction,
            >,
        >,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let adventurer_id = *req.extensions().get::<i32>().unwrap();

    match crew_switchboard_use_case
        .join(quest_id, adventurer_id)
        .await
    {
        Ok(_) => web::HttpResponse::Ok().body(format!(
            "Adventurer id: {}, has joined quest id: {}",
            adventurer_id, quest_id
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::delete("/leave/{quest_id}")]
pub async fn leave(
    req: web::HttpRequest,
    crew_switchboard_use_case: State<
        Arc<
            CrewSwitchboardUseCase<
                CrewSwitchboardPostgres,
                QuestViewingPostgres,
                DieselTransaction,
            >,
        >,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let adventurer_id = *req.extensions().get::<i32>().unwrap();

    match crew_switchboard_use_case
        .leave(quest_id, adventurer_id)
        .await
    {
        Ok(_) => web::HttpResponse::Ok().body(format!(
            "Adventurer id: {}, has leaved quest id: {}",
            adventurer_id, quest_id
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::post("/transaction-test/{quest_id}")]
pub async fn transaction_test(
    req: web::HttpRequest,
    crew_switchboard_use_case: State<
        Arc<
            CrewSwitchboardUseCase<
                CrewSwitchboardPostgres,
                QuestViewingPostgres,
                DieselTransaction,
            >,
        >,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let adventurer_id = *req.extensions().get::<i32>().unwrap();

    match crew_switchboard_use_case
        .join_and_delete_transaction(quest_id, adventurer_id)
        .await
    {
        Ok(_) => web::HttpResponse::Ok().body(format!(
            "Adventurer id: {}, has join and leave quest id: {}",
            adventurer_id, quest_id
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

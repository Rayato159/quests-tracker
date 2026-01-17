use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Path, State},
};

use crate::{
    application::usecases::journey_ledger::JourneyLedgerUseCase,
    domain::value_objects::quest_statuses::QuestStatuses,
    infrastructure::ntex_http::middleware::GuildCommandersAuthorization,
    infrastructure::postgres::repositories::{
        journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres,
    },
};

pub fn journey_ledger_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/journey-ledger")
            .wrap(GuildCommandersAuthorization)
            .service(in_journey)
            .service(to_completed)
            .service(to_failed),
    );
}

#[web::patch("/in-journey/{quest_id}")]
pub async fn in_journey(
    req: web::HttpRequest,
    journey_ledger_use_case: State<
        Arc<JourneyLedgerUseCase<JourneyLedgerPostgres, QuestViewingPostgres>>,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();

    match journey_ledger_use_case
        .in_journey(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => web::HttpResponse::Ok().body(format!(
            "Quest id: {} is now {:?}",
            quest_id,
            QuestStatuses::InJourney
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::patch("/to-completed/{quest_id}")]
pub async fn to_completed(
    req: web::HttpRequest,
    journey_ledger_use_case: State<
        Arc<JourneyLedgerUseCase<JourneyLedgerPostgres, QuestViewingPostgres>>,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();

    match journey_ledger_use_case
        .to_completed(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => web::HttpResponse::Ok().body(format!(
            "Quest id: {} is now {:?}",
            quest_id,
            QuestStatuses::Completed
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::patch("/to-failed/{quest_id}")]
pub async fn to_failed(
    req: web::HttpRequest,
    journey_ledger_use_case: State<
        Arc<JourneyLedgerUseCase<JourneyLedgerPostgres, QuestViewingPostgres>>,
    >,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();

    match journey_ledger_use_case
        .to_failed(quest_id, guild_commander_id)
        .await
    {
        Ok(quest_id) => web::HttpResponse::Ok().body(format!(
            "Quest id: {} is now {:?}",
            quest_id,
            QuestStatuses::Failed
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

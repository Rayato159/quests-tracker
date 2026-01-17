use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Json, Path, State},
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::value_objects::quest_model::{AddQuestModel, EditQuestModel},
    infrastructure::ntex_http::middleware::GuildCommandersAuthorization,
    infrastructure::postgres::repositories::{
        quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres,
    },
};

pub fn quest_ops_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/quest-ops")
            .wrap(GuildCommandersAuthorization)
            .service(add)
            .service(edit)
            .service(remove),
    );
}

#[web::post("")]
pub async fn add(
    req: web::HttpRequest,
    quest_ops_use_case: State<Arc<QuestOpsUseCase<QuestOpsPostgres, QuestViewingPostgres>>>,
    add_quest_model: Json<AddQuestModel>,
) -> web::HttpResponse {
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();
    match quest_ops_use_case
        .add(guild_commander_id, add_quest_model.into_inner())
        .await
    {
        Ok(quest_id_result) => {
            let response = format!("Add quest success with id: {}", quest_id_result);
            web::HttpResponse::Created().body(response)
        }
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::patch("/{quest_id}")]
pub async fn edit(
    req: web::HttpRequest,
    quest_ops_use_case: State<Arc<QuestOpsUseCase<QuestOpsPostgres, QuestViewingPostgres>>>,
    path: Path<i32>,
    edit_quest_model: Json<EditQuestModel>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();
    match quest_ops_use_case
        .edit(quest_id, guild_commander_id, edit_quest_model.into_inner())
        .await
    {
        Ok(quest_id_result) => {
            let response = format!("Edit quest success with id: {}", quest_id_result);
            web::HttpResponse::Ok().body(response)
        }
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[web::delete("/{quest_id}")]
pub async fn remove(
    req: web::HttpRequest,
    quest_ops_use_case: State<Arc<QuestOpsUseCase<QuestOpsPostgres, QuestViewingPostgres>>>,
    path: Path<i32>,
) -> web::HttpResponse {
    let quest_id = path.into_inner();
    let guild_commander_id = *req.extensions().get::<i32>().unwrap();
    match quest_ops_use_case
        .remove(quest_id, guild_commander_id)
        .await
    {
        Ok(_) => {
            let response = format!("Remove quest success with id: {}", quest_id);
            web::HttpResponse::Ok().body(response)
        }
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

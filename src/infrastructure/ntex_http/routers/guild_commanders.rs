use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Json, State},
};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::value_objects::guild_commander_model::RegisterGuildCommanderModel,
    infrastructure::postgres::repositories::guild_commanders::GuildCommandersPostgres,
};

pub fn guild_commanders_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/guild-commanders").service(register));
}

#[web::post("")]
pub async fn register(
    guild_commanders_use_case: State<Arc<GuildCommandersUseCase<GuildCommandersPostgres>>>,
    register_guild_commander_model: Json<RegisterGuildCommanderModel>,
) -> web::HttpResponse {
    match guild_commanders_use_case
        .register(register_guild_commander_model.into_inner())
        .await
    {
        Ok(guild_commander_id) => web::HttpResponse::Created().body(format!(
            "Register guild_commander id: {} successfully",
            guild_commander_id
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

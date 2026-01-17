use std::sync::Arc;

use ntex::web::{
    self, ServiceConfig, scope,
    types::{Json, State},
};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::value_objects::adventurer_model::RegisterAdventurerModel,
    infrastructure::postgres::repositories::adventurers::AdventurersPostgres,
};

pub fn adventurers_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/adventurers").service(register));
}

#[web::post("")]
pub async fn register(
    adventurer_use_case: State<Arc<AdventurersUseCase<AdventurersPostgres>>>,
    register_adventurer_model: Json<RegisterAdventurerModel>,
) -> web::HttpResponse {
    match adventurer_use_case
        .register(register_adventurer_model.into_inner())
        .await
    {
        Ok(adventurer_id) => web::HttpResponse::Created().body(format!(
            "Register adventurer id: {} successfully",
            adventurer_id
        )),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

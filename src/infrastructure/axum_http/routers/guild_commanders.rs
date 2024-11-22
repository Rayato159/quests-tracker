use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::guild_commanders::GuildCommandersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repository = GuildCommandersPostgres::new(db_pool);
    let guild_commanders_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commanders_repository));

    Router::new().route("/", post(register))
}

pub async fn register() -> impl IntoResponse {
    (StatusCode::CREATED, "Register").into_response()
}

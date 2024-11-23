use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::guild_commanders::GuildCommandersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repository = GuildCommandersPostgres::new(db_pool);
    let guild_commanders_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commanders_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_use_case))
}

pub async fn register<T>(
    State(guild_commanders_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    match guild_commanders_use_case
        .register(register_guild_commander_model)
        .await
    {
        Ok(guild_commander_id) => (
            StatusCode::CREATED,
            format!(
                "Register guild_commander id: {} successfully",
                guild_commander_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::config_model::{self},
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connector::PgPoolSquad,
            repositories::{
                adventurers::AdventurersPostgres, guild_commanders::GuildCommandersPostgres,
            },
        },
    },
};

pub fn routes(
    db_pool: Arc<PgPoolSquad>,
    jwt_config: Arc<config_model::JwtAuthentication>,
) -> Router {
    let adventurers_repository = AdventurersPostgres::new(Arc::clone(&db_pool));
    let guild_commanders_repository = GuildCommandersPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commanders_repository),
        Arc::clone(&jwt_config),
    );

    Router::new()
        .route("/adventurer/login", post(adventurer_login))
        .route("/adventurer/refresh-token", post(adventurer_refresh_token))
        .route("/guild-commander/login", post(guild_commander_login))
        .route(
            "/guild-commander/refresh-token",
            post(guild_commander_refresh_token),
        )
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurer_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case.adventurer_login(login_model).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn adventurer_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(refresh_token): Json<String>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case
        .adventurer_refresh_token(refresh_token)
        .await
    {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn guild_commander_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case
        .guild_commander_login(login_model)
        .await
    {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn guild_commander_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(refresh_token): Json<String>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case
        .guild_commander_refresh_token(refresh_token)
        .await
    {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

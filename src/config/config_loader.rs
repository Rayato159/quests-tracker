use anyhow::Result;

use super::{
    config_model::{AdventurersSecret, DotEnvyConfig, GuildCommandersSecret},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = super::config_model::Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = super::config_model::Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventurers_secret_env() -> Result<AdventurersSecret> {
    dotenvy::dotenv().ok();

    Ok(AdventurersSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET")?,
        refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET")?,
    })
}

pub fn get_guild_commanders_secret_env() -> Result<GuildCommandersSecret> {
    dotenvy::dotenv().ok();

    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET")?,
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")?,
    })
}

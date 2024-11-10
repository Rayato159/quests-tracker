use anyhow::Result;

use super::config_model::DotEnvyConfig;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let stage = std::env::var("STAGE").expect("STAGE is invalid");
    let stage = super::stage::Stage::try_from(&stage)?;

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

    Ok(DotEnvyConfig {
        stage,
        server,
        database,
    })
}

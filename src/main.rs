use std::sync::Arc;

use quests_tracker::{config::config_loader, infrastructure::postgres::postgres_connector};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };
    info!("ENV has been loaded");

    let postgres_pool = match postgres_connector::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Postgres: {}", e);
            std::process::exit(1);
        }
    };
    info!("Postgres connection has been established");
}

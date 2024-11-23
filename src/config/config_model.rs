use super::stage::Stage;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub stage: Stage,
    pub server: Server,
    pub database: Database,
    pub jwt_authentication: JwtAuthentication,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct JwtAuthentication {
    pub adventurer_secret: String,
    pub adventurer_refresh_secret: String,
    pub guild_commander_secret: String,
    pub guild_commander_refresh_secret: String,
}

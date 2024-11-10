use super::stage::Stage;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub stage: Stage,
    pub server: Server,
    pub database: Database,
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

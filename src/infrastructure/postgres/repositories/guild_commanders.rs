use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::guild_commanders::InsertGuildCommanderEntity,
        repositories::guild_commanders::GuildCommandersRepository,
    },
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct GuildCommandersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommandersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GuildCommandersRepository for GuildCommandersPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: InsertGuildCommanderEntity,
    ) -> Result<()> {
        panic!("Not implemented");
    }
}

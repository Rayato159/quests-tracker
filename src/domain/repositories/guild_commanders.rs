use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::guild_commanders::InsertGuildCommanderEntity;

pub type GuildCommandersRepositorySquad = Arc<dyn GuildCommandersRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait GuildCommandersRepository {
    async fn register(
        &self,
        insert_guild_commander_entity: InsertGuildCommanderEntity,
    ) -> Result<()>;
}

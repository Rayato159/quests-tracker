use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::guild_commanders::GuildCommandersRepository,
    value_objects::guild_commander_model::InsertGuildCommanderModel,
};

pub struct GuildCommandersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }
}

impl<T> GuildCommandersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    async fn register(
        &self,
        insert_guild_commander_model: InsertGuildCommanderModel,
    ) -> Result<()> {
        panic!("Not implemented");
    }
}

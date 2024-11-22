use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::guild_commanders::GuildCommandersRepositorySquad,
    value_objects::guild_commander_model::InsertGuildCommanderModel,
};

pub struct GuildCommandersUseCase {
    guild_commanders_repository: Arc<GuildCommandersRepositorySquad>,
}

impl GuildCommandersUseCase {
    async fn register(
        &self,
        insert_guild_commander_model: InsertGuildCommanderModel,
    ) -> Result<()> {
        panic!("Not implemented");
    }
}

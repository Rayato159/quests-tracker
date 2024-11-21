use anyhow::Result;

use crate::domain::{
    repositories::guild_commanders::GuildCommandersRepositorySquad,
    value_objects::guild_commander_model::InsertGuildCommanderModel,
};

pub struct GuildCommandersRepository {
    guild_commanders_repository: GuildCommandersRepositorySquad,
}

impl GuildCommandersRepository {
    async fn register(
        &self,
        insert_guild_commander_model: InsertGuildCommanderModel,
    ) -> Result<()> {
        panic!("Not implemented");
    }
}

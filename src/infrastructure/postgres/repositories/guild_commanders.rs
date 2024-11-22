use anyhow::Result;

use crate::domain::entities::guild_commanders::InsertGuildCommanderEntity;

pub struct GuildCommandersPostgres;

impl GuildCommandersPostgres {
    async fn register(
        &self,
        insert_guild_commander_entity: InsertGuildCommanderEntity,
    ) -> Result<()> {
        panic!("Not implemented");
    }
}

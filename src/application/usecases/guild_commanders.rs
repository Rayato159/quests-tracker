use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::hashing::Hashing,
};

pub struct GuildCommandersUseCase<T1, T2>
where
    T1: GuildCommandersRepository + Send + Sync,
    T2: Hashing,
{
    guild_commanders_repository: Arc<T1>,
    hashing: Arc<T2>,
}

impl<T1, T2> GuildCommandersUseCase<T1, T2>
where
    T1: GuildCommandersRepository + Send + Sync,
    T2: Hashing,
{
    pub fn new(guild_commanders_repository: Arc<T1>, hashing: Arc<T2>) -> Self {
        Self {
            guild_commanders_repository,
            hashing,
        }
    }

    pub async fn register(
        &self,
        mut register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        let hashed_password = self
            .hashing
            .hash(register_guild_commander_model.password.clone())?;

        register_guild_commander_model.password = hashed_password;

        let register_entity = register_guild_commander_model.to_entity();

        let guild_commander_id = self
            .guild_commanders_repository
            .register(register_entity)
            .await?;

        Ok(guild_commander_id)
    }
}

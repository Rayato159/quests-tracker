use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    value_objects::passport::Passport,
};

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    guild_commanders_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, guild_commanders_repository: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            guild_commanders_repository,
        }
    }
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    async fn login(&self, username: &str) -> Result<Passport> {
        panic!("Not implemented");
    }

    async fn logout(&self) -> Result<()> {
        panic!("Not implemented");
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
        panic!("Not implemented");
    }
}

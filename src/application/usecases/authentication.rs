use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        adventurers::AdventurersRepositorySquad, guild_commanders::GuildCommandersRepositorySquad,
    },
    value_objects::passport::Passport,
};

pub struct AuthenticationUseCase {
    adventurers_repository: Arc<AdventurersRepositorySquad>,
    guild_commanders_repository: Arc<GuildCommandersRepositorySquad>,
}

impl AuthenticationUseCase {
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

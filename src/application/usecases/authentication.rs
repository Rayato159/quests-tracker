use anyhow::Result;

use crate::domain::{
    repositories::{
        adventurers::AdvanturersRepositorySquad, guild_commanders::GuildCommandersRepositorySquad,
    },
    value_objects::passport::Passport,
};

pub struct AuthenticationRepository {
    advanturers_repository: AdvanturersRepositorySquad,
    guild_commanders_repository: GuildCommandersRepositorySquad,
}

impl AuthenticationRepository {
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

use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{
    config::config_model,
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        argon2_hashing,
        jwt_authentication::{
            self,
            authentication_model::LoginModel,
            jwt_model::{Claims, Passport, Roles},
        },
    },
};

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    guild_commanders_repository: Arc<T2>,
    pub jwt_config: Arc<config_model::JwtAuthentication>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(
        adventurers_repository: Arc<T1>,
        guild_commanders_repository: Arc<T2>,
        jwt_config: Arc<config_model::JwtAuthentication>,
    ) -> Self {
        Self {
            adventurers_repository,
            guild_commanders_repository,
            jwt_config,
        }
    }

    pub async fn adventurer_login(&self, login_model: LoginModel) -> Result<Passport> {
        let adventurer = self
            .adventurers_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = adventurer.password;
        let login_password = login_model.password;

        if argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = jwt_authentication::generate_token(
            self.jwt_config.adventurer_secret.clone(),
            &access_token_claims,
        )?;

        let refresh_token = jwt_authentication::generate_token(
            self.jwt_config.adventurer_refresh_secret.clone(),
            &refresh_token_claims,
        )?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn adventurer_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let claims = jwt_authentication::verify_token(
            self.jwt_config.adventurer_refresh_secret.clone(),
            refresh_token.clone(),
        )?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::Adventurer,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = jwt_authentication::generate_token(
            self.jwt_config.adventurer_secret.clone(),
            &access_token_claims,
        )?;

        let refresh_token = jwt_authentication::generate_token(
            self.jwt_config.adventurer_refresh_secret.clone(),
            &refresh_token_claims,
        )?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn guild_commander_login(&self, login_model: LoginModel) -> Result<Passport> {
        let guild_commander = self
            .guild_commanders_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = guild_commander.password;
        let login_password = login_model.password;

        if argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = jwt_authentication::generate_token(
            self.jwt_config.guild_commander_secret.clone(),
            &access_token_claims,
        )?;

        let refresh_token = jwt_authentication::generate_token(
            self.jwt_config.guild_commander_refresh_secret.clone(),
            &refresh_token_claims,
        )?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn guild_commander_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let claims = jwt_authentication::verify_token(
            self.jwt_config.guild_commander_refresh_secret.clone(),
            refresh_token.clone(),
        )?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::GuildCommander,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = jwt_authentication::generate_token(
            self.jwt_config.guild_commander_secret.clone(),
            &access_token_claims,
        )?;

        let refresh_token = jwt_authentication::generate_token(
            self.jwt_config.guild_commander_refresh_secret.clone(),
            &refresh_token_claims,
        )?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }
}

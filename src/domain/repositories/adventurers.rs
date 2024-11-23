use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity};

#[async_trait]
#[automock]
pub trait AdventurersRepository {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity>;
}

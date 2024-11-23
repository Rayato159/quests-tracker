use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::adventurers::RegisterAdventurerEntity;

#[async_trait]
#[automock]
pub trait AdventurersRepository {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32>;
}

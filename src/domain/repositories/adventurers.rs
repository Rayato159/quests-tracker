use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::adventurers::InsertAdventurerEntity;

pub type AdventurersRepositorySquad = dyn AdventurersRepository + Send + Sync;

#[async_trait]
#[automock]
pub trait AdventurersRepository {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()>;
}

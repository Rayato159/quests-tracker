use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::adventurers::InsertAdventurerEntity;

#[async_trait]
#[automock]
pub trait AdventurersRepository {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()>;
}

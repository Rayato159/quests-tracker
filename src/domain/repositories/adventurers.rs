use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::adventurers::InsertAdventurerEntity;

pub type AdvanturersRepositorySquad = Arc<dyn AdvanturersRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait AdvanturersRepository {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()>;
}

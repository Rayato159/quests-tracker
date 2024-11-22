use anyhow::Result;
use axum::async_trait;
use mockall::automock;

pub type CrewSwitchboardRepositorySquad = dyn CrewSwitchboardRepository + Send + Sync;

#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository {
    async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()>;
    async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()>;
}

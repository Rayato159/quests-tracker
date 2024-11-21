use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

pub type JourneyLedgerRepositorySquad = Arc<dyn JourneyLedgerRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait JourneyLedgerRepository {
    async fn in_journey(&self, quest_id: i32) -> Result<()>;
    async fn to_completed(&self, quest_id: i32) -> Result<()>;
    async fn to_failed(&self, quest_id: i32) -> Result<()>;
}

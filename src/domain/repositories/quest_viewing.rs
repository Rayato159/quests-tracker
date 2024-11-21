use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::{
    entities::quests::QuestEntity, value_objects::board_checking_filter::BoardCheckingFilter,
};

pub type QuestViewingRepositorySquad = Arc<dyn QuestViewingRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait QuestViewingRepository {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity>;
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>>;
}

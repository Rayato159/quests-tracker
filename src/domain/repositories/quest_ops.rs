use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::quests::{InsertQuestEntity, UpdateQuestEntity};

#[async_trait]
#[automock]
pub trait QuestOpsRepository {
    async fn add(&self, insert_quest_entity: InsertQuestEntity) -> Result<()>;
    async fn edit(&self, update_quest_entity: UpdateQuestEntity) -> Result<()>;
    async fn remove(&self, quest_id: i32) -> Result<()>;
}

use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::quests::{AddQuestEntity, EditQuestEntity},
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::{
        quest_model::{AddQuestModel, EditQuestModel},
        quest_statuses::QuestStatuses,
    },
};

pub struct QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    quest_ops_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository,
        }
    }

    pub async fn add(&self, add_quest_model: AddQuestModel) -> Result<i32> {
        // Check if adventurer exists

        // Add quest
        let insert_quset_entity = AddQuestEntity {
            name: add_quest_model.name,
            description: add_quest_model.description,
            guild_commander_id: add_quest_model.guild_commander_id,
            status: QuestStatuses::Open.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let result = self.quest_ops_repository.add(insert_quset_entity).await?;

        Ok(result)
    }

    pub async fn edit(&self, quest_id: i32, edit_quest_model: EditQuestModel) -> Result<i32> {
        // Check if adventurer exists

        // Update quest
        let edit_quest_entity = EditQuestEntity {
            name: edit_quest_model.name,
            description: edit_quest_model.description,
            guild_commander_id: edit_quest_model.guild_commander_id,
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let result = self
            .quest_ops_repository
            .edit(quest_id, edit_quest_entity)
            .await?;

        Ok(result)
    }

    pub async fn remove(&self, quest_id: i32) -> Result<()> {
        self.quest_ops_repository.remove(quest_id).await?;

        Ok(())
    }
}

use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest.id)
            .await?;

        let quest_model = QuestModel {
            id: quest.id,
            name: quest.name,
            description: quest.description,
            status: quest.status,
            guild_commander_id: quest.guild_commander_id,
            adventurer_count: adventurers_count,
            created_at: quest.created_at,
            updated_at: quest.updated_at,
        };

        Ok(quest_model)
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let results = self.quest_viewing_repository.board_checking(filter).await?;

        let quests_model = results
            .into_iter()
            .map(|q| QuestModel {
                id: q.id,
                name: q.name,
                description: q.description,
                status: q.status,
                guild_commander_id: q.guild_commander_id,
                adventurer_count: 0,
                created_at: q.created_at,
                updated_at: q.updated_at,
            })
            .collect::<Vec<QuestModel>>();

        Ok(quests_model)
    }
}

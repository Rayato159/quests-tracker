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
        let result = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(result.id)
            .await?;

        let quest_model = result.to_model(adventurers_count);

        Ok(quest_model)
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let results = self.quest_viewing_repository.board_checking(filter).await?;

        let mut quests_model = Vec::new();

        for result in results.into_iter() {
            let adventurers_count = self
                .quest_viewing_repository
                .adventurers_counting_by_quest_id(result.id)
                .await
                .unwrap_or(0);

            quests_model.push(result.to_model(adventurers_count));
        }

        Ok(quests_model)
    }
}

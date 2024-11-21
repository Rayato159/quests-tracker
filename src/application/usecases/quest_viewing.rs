use anyhow::Result;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepositorySquad,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};

pub struct QuestViewingRepository {
    quest_viewing_repository: QuestViewingRepositorySquad,
}

impl QuestViewingRepository {
    async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        panic!("Not implemented");
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        panic!("Not implemented");
    }
}

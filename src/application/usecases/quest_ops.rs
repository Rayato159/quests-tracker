use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        quest_ops::QuestOpsRepositorySquad, quest_viewing::QuestViewingRepositorySquad,
    },
    value_objects::quest_model::{InsertQuestModel, UpdateQuestModel},
};

pub struct QuestOpsUseCase {
    quest_ops_repository: Arc<QuestOpsRepositorySquad>,
    quest_viewing_repository: Arc<QuestViewingRepositorySquad>,
}

impl QuestOpsUseCase {
    async fn add(&self, insert_quest_model: InsertQuestModel) -> Result<()> {
        panic!("Not implemented");
    }

    async fn edit(&self, update_quest_model: UpdateQuestModel) -> Result<()> {
        panic!("Not implemented");
    }

    async fn remove(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented");
    }
}

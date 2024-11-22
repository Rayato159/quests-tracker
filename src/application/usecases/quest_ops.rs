use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{InsertQuestModel, UpdateQuestModel},
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

    pub async fn add(&self, insert_quest_model: InsertQuestModel) -> Result<()> {
        panic!("Not implemented");
    }

    pub async fn edit(&self, update_quest_model: UpdateQuestModel) -> Result<()> {
        panic!("Not implemented");
    }

    pub async fn remove(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented");
    }
}

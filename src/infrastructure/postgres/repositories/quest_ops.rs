use anyhow::Result;

use crate::domain::entities::quests::{InsertQuestEntity, UpdateQuestEntity};

pub struct QuestOpsPostgres;

impl QuestOpsPostgres {
    async fn add(&self, insert_quest_entity: InsertQuestEntity) -> Result<()> {
        panic!("Not implemented")
    }

    async fn edit(&self, update_quest_entity: UpdateQuestEntity) -> Result<()> {
        panic!("Not implemented")
    }

    async fn remove(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented")
    }
}

use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::quests::{InsertQuestEntity, UpdateQuestEntity},
        repositories::quest_ops::QuestOpsRepository,
    },
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct QuestOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
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

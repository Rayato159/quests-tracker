use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        entities::quests::QuestEntity,
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::{
            board_checking_filter::BoardCheckingFilter,
            quest_adventurer_junction::QuestAdventurerJunction,
        },
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad,
        schema::{quest_adventurer_junction, quests},
    },
};

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quests::table
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .select(QuestEntity::as_select())
            .first::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        panic!("Not implemented");
    }

    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quest_adventurer_junction::table
            .filter(quest_adventurer_junction::quest_id.eq(quest_id))
            .count()
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
}

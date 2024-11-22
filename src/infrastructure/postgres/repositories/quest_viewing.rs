use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        panic!("Not implemented");
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        panic!("Not implemented");
    }
}

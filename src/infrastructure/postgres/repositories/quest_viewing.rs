use anyhow::Result;

use crate::domain::{
    entities::quests::QuestEntity, value_objects::board_checking_filter::BoardCheckingFilter,
};

pub struct QuestViewingPostgres;

impl QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        panic!("Not implemented");
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        panic!("Not implemented");
    }
}

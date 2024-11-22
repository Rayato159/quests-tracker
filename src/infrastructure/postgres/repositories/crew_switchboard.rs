use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::repositories::crew_switchboard::CrewSwitchboardRepository,
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchboardPostgres {
    async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented")
    }

    async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented")
    }
}

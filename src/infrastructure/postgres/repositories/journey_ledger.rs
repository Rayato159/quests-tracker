use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::repositories::journey_ledger::JourneyLedgerRepository,
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct JourneyLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JourneyLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepository for JourneyLedgerPostgres {
    async fn in_journey(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented");
    }

    async fn to_completed(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented");
    }

    async fn to_failed(&self, quest_id: i32) -> Result<()> {
        panic!("Not implemented");
    }
}

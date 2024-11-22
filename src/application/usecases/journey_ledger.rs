use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::{
    journey_ledger::JourneyLedgerRepositorySquad, quest_viewing::QuestViewingRepositorySquad,
};

pub struct JourneyLedgerUseCase {
    journey_ledger_repository: Arc<JourneyLedgerRepositorySquad>,
    quest_viewing_repository: Arc<QuestViewingRepositorySquad>,
}

impl JourneyLedgerUseCase {
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

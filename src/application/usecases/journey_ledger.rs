use anyhow::Result;

use crate::domain::repositories::{
    journey_ledger::JourneyLedgerRepositorySquad, quest_viewing::QuestViewingRepositorySquad,
};

pub struct JourneyLedgerRepository {
    journey_ledger_repository: JourneyLedgerRepositorySquad,
    quest_viewing_repository: QuestViewingRepositorySquad,
}

impl JourneyLedgerRepository {
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

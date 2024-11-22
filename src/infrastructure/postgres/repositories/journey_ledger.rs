use anyhow::Result;

pub struct JourneyLedgerPostgres;

impl JourneyLedgerPostgres {
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

use anyhow::Result;

pub struct CrewSwitchboardPostgres;

impl CrewSwitchboardPostgres {
    async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented")
    }

    async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented")
    }
}

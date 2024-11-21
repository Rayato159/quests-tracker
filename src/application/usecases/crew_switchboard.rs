use anyhow::Result;

use crate::domain::repositories::{
    crew_switchboard::CrewSwitchboardRepositorySquad, quest_viewing::QuestViewingRepositorySquad,
};

pub struct CrewSwitchboardRepository {
    crew_switchboard_repository: CrewSwitchboardRepositorySquad,
    quest_viewing_repository: QuestViewingRepositorySquad,
}

impl CrewSwitchboardRepository {
    async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented");
    }

    async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented");
    }
}

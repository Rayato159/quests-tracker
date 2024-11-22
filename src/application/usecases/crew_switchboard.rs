use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::{
    crew_switchboard::CrewSwitchboardRepositorySquad, quest_viewing::QuestViewingRepositorySquad,
};

pub struct CrewSwitchboardUseCase {
    crew_switchboard_repository: Arc<CrewSwitchboardRepositorySquad>,
    quest_viewing_repository: Arc<QuestViewingRepositorySquad>,
}

impl CrewSwitchboardUseCase {
    async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented");
    }

    async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        panic!("Not implemented");
    }
}

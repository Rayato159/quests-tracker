use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::value_objects::quest_adventurer_junction::QuestAdventurerJunction;

#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()>;
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()>;
}

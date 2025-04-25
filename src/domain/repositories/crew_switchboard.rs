use anyhow::Result;
use axum::async_trait;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use mockall::automock;

use crate::domain::value_objects::quest_adventurer_junction::QuestAdventurerJunction;

#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()>;
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()>;
    // An example of a method that could be used for testing purposes
    fn for_transaction_test_1(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: QuestAdventurerJunction,
    ) -> Result<()>;
    fn for_transaction_test_2(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: QuestAdventurerJunction,
    ) -> Result<()>;
}

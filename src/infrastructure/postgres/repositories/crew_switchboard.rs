use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchboardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, schema::quest_adventurer_junction,
    },
};

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchboardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        insert_into(quest_adventurer_junction::table)
            .values(junction_body)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        delete(quest_adventurer_junction::table)
            .filter(quest_adventurer_junction::adventurer_id.eq(junction_body.adventurer_id))
            .filter(quest_adventurer_junction::quest_id.eq(junction_body.quest_id))
            .execute(&mut conn)?;

        Ok(())
    }

    // An example of a method that could be used for testing purposes
    fn for_transaction_test_1(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: QuestAdventurerJunction,
    ) -> Result<()> {
        insert_into(quest_adventurer_junction::table)
            .values(junction_body)
            .execute(conn)?;

        Ok(())
    }

    fn for_transaction_test_2(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: QuestAdventurerJunction,
    ) -> Result<()> {
        delete(quest_adventurer_junction::table)
            .filter(quest_adventurer_junction::adventurer_id.eq(junction_body.adventurer_id))
            .filter(quest_adventurer_junction::quest_id.eq(junction_body.quest_id))
            .execute(conn)?;

        Ok(())
    }
}

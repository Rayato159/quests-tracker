use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::entities::adventurers::InsertAdventurerEntity,
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct AdventurersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

impl AdventurersPostgres {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()> {
        panic!("Not implemented")
    }
}

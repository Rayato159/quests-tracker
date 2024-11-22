use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::adventurers::InsertAdventurerEntity,
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgres::postgres_connector::PgPoolSquad,
};

pub struct AdventurersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

#[async_trait]
impl AdventurersRepository for AdventurersPostgres {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()> {
        panic!("Not implemented")
    }
}

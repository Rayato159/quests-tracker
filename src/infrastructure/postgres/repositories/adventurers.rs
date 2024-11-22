use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::{
    domain::entities::adventurers::RegisterAdventurerEntity,
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

#[async_trait]
impl AdventurersRepository for AdventurersPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<()> {
        panic!("Not implemented")
    }
}

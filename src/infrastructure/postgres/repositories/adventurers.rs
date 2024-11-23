use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};

use crate::domain::entities::adventurers::AdventurerEntity;
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::infrastructure::postgres::schema::adventurers;
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
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
            .values(&register_adventurer_entity)
            .returning(adventurers::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .first::<AdventurerEntity>(&mut conn)?;

        Ok(result)
    }
}

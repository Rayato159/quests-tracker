use anyhow::Result;

use crate::domain::entities::adventurers::InsertAdventurerEntity;

pub struct AdventurersPostgres;

impl AdventurersPostgres {
    async fn register(&self, insert_adventurer_entity: InsertAdventurerEntity) -> Result<()> {
        panic!("Not implemented")
    }
}

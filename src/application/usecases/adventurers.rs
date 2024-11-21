use anyhow::Result;

use crate::domain::{
    repositories::adventurers::AdvanturersRepositorySquad,
    value_objects::adventurer_model::InsertAdventurerModel,
};

pub struct AdvanturersRepository {
    advanturers_repository: AdvanturersRepositorySquad,
}

impl AdvanturersRepository {
    async fn register(&self, insert_adventurer_model: InsertAdventurerModel) -> Result<()> {
        panic!("Not implemented");
    }
}

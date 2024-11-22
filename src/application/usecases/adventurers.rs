use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::adventurers::AdventurersRepositorySquad,
    value_objects::adventurer_model::InsertAdventurerModel,
};

pub struct AdventurersUseCase {
    adventurers_repository: Arc<AdventurersRepositorySquad>,
}

impl AdventurersUseCase {
    async fn register(&self, insert_adventurer_model: InsertAdventurerModel) -> Result<()> {
        panic!("Not implemented");
    }
}

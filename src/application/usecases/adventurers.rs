use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::adventurers::AdventurersRepository,
    value_objects::adventurer_model::InsertAdventurerModel,
};

pub struct AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: T) -> Self {
        Self {
            adventurers_repository: Arc::new(adventurers_repository),
        }
    }
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    async fn register(&self, insert_adventurer_model: InsertAdventurerModel) -> Result<()> {
        panic!("Not implemented");
    }
}

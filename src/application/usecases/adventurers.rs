use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::hashing::Hashing,
};

pub struct AdventurersUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: Hashing,
{
    adventurers_repository: Arc<T1>,
    hashing: Arc<T2>,
}

impl<T1, T2> AdventurersUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: Hashing + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, hashing: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            hashing,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = self
            .hashing
            .hash(register_adventurer_model.password.clone())?;

        register_adventurer_model.password = hashed_password;

        let register_entity = register_adventurer_model.to_entity();

        let adventurer_id = self
            .adventurers_repository
            .register(register_entity)
            .await?;

        Ok(adventurer_id)
    }
}

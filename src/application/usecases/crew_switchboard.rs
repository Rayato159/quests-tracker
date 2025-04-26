use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
        transaction_provider::TransactionProvider,
    },
    value_objects::{
        quest_adventurer_junction::{QuestAdventurerJunction, MAX_ADVENTURERS_PER_QUEST},
        quest_statuses::QuestStatuses,
    },
};

// T3 is just a placeholder for the transaction provider, not necessarily.
pub struct CrewSwitchboardUseCase<T1, T2, T3>
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    crew_switchboard_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
    tx: Arc<T3>,
}

impl<T1, T2, T3> CrewSwitchboardUseCase<T1, T2, T3>
where
    T1: CrewSwitchboardRepository + Send + Sync + 'static,
    T2: QuestViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    pub fn new(
        crew_switchboard_repository: Arc<T1>,
        quest_viewing_repository: Arc<T2>,
        tx: Arc<T3>,
    ) -> Self {
        Self {
            crew_switchboard_repository,
            quest_viewing_repository,
            tx,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let quest_status_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();
        let adventurers_count_condition = adventurers_count < MAX_ADVENTURERS_PER_QUEST;

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not joinable"));
        }

        if !adventurers_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        self.crew_switchboard_repository
            .join(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let leaving_coditions = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        if !leaving_coditions {
            return Err(anyhow::anyhow!("Quest is not leavable"));
        }

        self.crew_switchboard_repository
            .leave(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }

    pub async fn join_and_delete_transaction(
        &self,
        quest_id: i32,
        adventurer_id: i32,
    ) -> Result<()> {
        let tx = Arc::clone(&self.tx);
        let repo = Arc::clone(&self.crew_switchboard_repository);

        tx.transaction::<_, anyhow::Error, _>(move |conn| {
            repo.for_transaction_test_1(
                conn,
                QuestAdventurerJunction {
                    quest_id,
                    adventurer_id,
                },
            )?;

            repo.for_transaction_test_2(
                conn,
                QuestAdventurerJunction {
                    quest_id,
                    adventurer_id,
                },
            )?;

            Ok(())
        })
    }
}

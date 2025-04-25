#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{TimeZone, Utc};
    use diesel::{
        r2d2::{ConnectionManager, PooledConnection},
        PgConnection,
    };

    use crate::{
        application::usecases::crew_switchboard::CrewSwitchboardUseCase,
        domain::{
            entities::quests::QuestEntity,
            repositories::{
                crew_switchboard::MockCrewSwitchboardRepository,
                quest_viewing::MockQuestViewingRepository,
                transaction_provider::MockTransactionProvider,
            },
            value_objects::{
                quest_adventurer_junction::MAX_ADVENTURERS_PER_QUEST, quest_statuses::QuestStatuses,
            },
        },
        infrastructure::postgres::postgres_connector::establish_connection,
    };

    #[tokio::test]
    async fn test_join_success() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_repo.expect_view_details().returning(move |_| {
            Box::pin(async move {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: now,
                    updated_at: now,
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_is_not_open() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_repo.expect_view_details().returning(move |_| {
            Box::pin(async move {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::InJourney.to_string(),
                    guild_commander_id: 1,
                    created_at: now,
                    updated_at: now,
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is not joinable");
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_is_full() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(MAX_ADVENTURERS_PER_QUEST) }));

        mock_quest_repo.expect_view_details().returning(move |_| {
            Box::pin(async move {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: now,
                    updated_at: now,
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is full");
    }

    #[tokio::test]
    async fn test_leave_success() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(1) }));

        mock_quest_repo.expect_view_details().returning(move |_| {
            Box::pin(async move {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: now,
                    updated_at: now,
                })
            })
        });

        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.leave(1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_fails_when_quest_is_not_open() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_repo.expect_view_details().returning(move |_| {
            Box::pin(async move {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::InJourney.to_string(),
                    guild_commander_id: 1,
                    created_at: now,
                    updated_at: now,
                })
            })
        });

        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.leave(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is not leavable");
    }

    // This is need a real database connection to test the transaction.
    // Maybe, I should PR to the maintainer about this.
    #[tokio::test]
    async fn test_join_and_leave_transaction() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mock_quest_repo = MockQuestViewingRepository::new();
        let mut mock_tx_repo = MockTransactionProvider::new();

        mock_tx_repo.expect_transaction().returning(
            |f: Box<
                dyn FnOnce(
                        &mut PooledConnection<ConnectionManager<PgConnection>>,
                    ) -> Result<(), anyhow::Error>
                    + 'static,
            >| {
                dotenvy::dotenv().ok();
                let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid");
                let db_pool = establish_connection(&db_url).unwrap();
                let mut conn = db_pool.get().unwrap();
                f(&mut conn)
            },
        );

        mock_crew_repo
            .expect_for_transaction_test_1()
            .returning(|_, _| Ok(()));

        mock_crew_repo
            .expect_for_transaction_test_2()
            .returning(|_, _| Ok(()));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_quest_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join_and_delete_transaction(1, 1).await;

        assert!(result.is_ok());
    }
}

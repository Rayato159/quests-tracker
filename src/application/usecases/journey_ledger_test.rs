#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::usecases::journey_ledger::JourneyLedgerUseCase,
        domain::{
            entities::quests::QuestEntity,
            repositories::{
                journey_ledger::MockJourneyLedgerRepository,
                quest_viewing::MockQuestViewingRepository,
            },
            value_objects::quest_statuses::QuestStatuses,
        },
    };

    #[tokio::test]
    async fn test_in_journey() {
        let mut mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::Open.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_journey_ledger_repository
            .expect_in_journey()
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.in_journey(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_in_journey_adventurers_max() {
        let mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::Open.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(5) }));

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.in_journey(1, 1).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_in_journey_adventurers_wrong_status() {
        let mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::InJourney.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(5) }));

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.in_journey(1, 1).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_to_completed() {
        let mut mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::InJourney.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        mock_journey_ledger_repository
            .expect_to_completed()
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.to_completed(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_to_completed_wrong_status() {
        let mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::Open.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.to_completed(1, 1).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_to_failed() {
        let mut mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::InJourney.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        mock_journey_ledger_repository
            .expect_to_failed()
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.to_failed(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_to_failed_wrong_status() {
        let mock_journey_ledger_repository = MockJourneyLedgerRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = chrono::Utc::now().naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        status: QuestStatuses::Open.to_string(),
                        created_at: now,
                        updated_at: now,
                        description: Some("This is a test quest".to_string()),
                        guild_commander_id: 1,
                    })
                })
            });

        let journey_ledger_use_case = JourneyLedgerUseCase::new(
            Arc::new(mock_journey_ledger_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = journey_ledger_use_case.to_failed(1, 1).await;

        assert!(result.is_err());
    }
}

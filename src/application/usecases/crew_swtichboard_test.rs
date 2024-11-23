#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{TimeZone, Utc};

    use crate::{
        application::usecases::crew_switchboard::CrewSwitchboardUseCase,
        domain::{
            entities::quests::QuestEntity,
            repositories::{
                crew_switchboard::MockCrewSwitchboardRepository,
                quest_viewing::MockQuestViewingRepository,
            },
            value_objects::{
                quest_adventurer_junction::MAX_ADVENTURERS_PER_QUEST, quest_statuses::QuestStatuses,
            },
        },
    };

    #[tokio::test]
    async fn test_join_success() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_repo.expect_view_details().returning(|_| {
            Box::pin(async {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),

                    updated_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case =
            CrewSwitchboardUseCase::new(Arc::new(mock_crew_repo), Arc::new(mock_quest_repo));

        let result = use_case.join(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_is_not_open() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_repo.expect_view_details().returning(|_| {
            Box::pin(async {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::InJourney.to_string(),
                    guild_commander_id: 1,
                    created_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),

                    updated_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case =
            CrewSwitchboardUseCase::new(Arc::new(mock_crew_repo), Arc::new(mock_quest_repo));

        let result = use_case.join(1, 1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is not joinable");
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_is_full() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(MAX_ADVENTURERS_PER_QUEST) }));

        mock_quest_repo.expect_view_details().returning(|_| {
            Box::pin(async {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),

                    updated_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),
                })
            })
        });

        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case =
            CrewSwitchboardUseCase::new(Arc::new(mock_crew_repo), Arc::new(mock_quest_repo));

        let result = use_case.join(1, 1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is full");
    }

    #[tokio::test]
    async fn test_leave_success() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();

        mock_quest_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(1) }));

        mock_quest_repo.expect_view_details().returning(|_| {
            Box::pin(async {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::Open.to_string(),
                    guild_commander_id: 1,
                    created_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),

                    updated_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),
                })
            })
        });

        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case =
            CrewSwitchboardUseCase::new(Arc::new(mock_crew_repo), Arc::new(mock_quest_repo));

        let result = use_case.leave(1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_fails_when_quest_is_not_open() {
        let mut mock_crew_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_repo = MockQuestViewingRepository::new();

        mock_quest_repo.expect_view_details().returning(|_| {
            Box::pin(async {
                Ok(QuestEntity {
                    id: 1,
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    status: QuestStatuses::InJourney.to_string(),
                    guild_commander_id: 1,
                    created_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),

                    updated_at: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .unwrap()
                        .naive_utc(),
                })
            })
        });

        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case =
            CrewSwitchboardUseCase::new(Arc::new(mock_crew_repo), Arc::new(mock_quest_repo));

        let result = use_case.leave(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Quest is not leavable");
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{TimeZone, Utc};

    use crate::{
        application::usecases::quest_viewing::QuestViewingUseCase,
        domain::{
            entities::quests::QuestEntity,
            repositories::quest_viewing::MockQuestViewingRepository,
            value_objects::{
                board_checking_filter::BoardCheckingFilter, quest_model::QuestModel,
                quest_statuses::QuestStatuses,
            },
        },
    };

    #[tokio::test]
    async fn test_view_details() {
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_viewing_repository
            .expect_view_details()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(QuestEntity {
                        id: 1,
                        name: "Test Quest".to_string(),
                        description: Some("Test Description".to_string()),
                        created_at: now,
                        updated_at: now,
                        guild_commander_id: 1,
                        status: QuestStatuses::Open.to_string(),
                    })
                })
            });

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        let quest_viewing_use_case =
            QuestViewingUseCase::new(Arc::new(mock_quest_viewing_repository));

        let result = quest_viewing_use_case.view_details(1).await.unwrap();

        assert_eq!(
            result,
            QuestModel {
                id: 1,
                name: "Test Quest".to_string(),
                description: Some("Test Description".to_string()),
                created_at: now,
                updated_at: now,
                guild_commander_id: 1,
                status: QuestStatuses::Open.to_string(),
                adventurers_count: 2,
            }
        )
    }

    #[tokio::test]
    async fn test_board_checking() {
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_quest_viewing_repository
            .expect_board_checking()
            .returning(move |_| {
                Box::pin(async move {
                    Ok(vec![
                        QuestEntity {
                            id: 1,
                            name: "Test Quest".to_string(),
                            description: Some("Test Description".to_string()),
                            created_at: now,
                            updated_at: now,
                            guild_commander_id: 1,
                            status: QuestStatuses::Open.to_string(),
                        },
                        QuestEntity {
                            id: 2,
                            name: "Test Quest".to_string(),
                            description: Some("Test Description".to_string()),
                            created_at: now,
                            updated_at: now,
                            guild_commander_id: 1,
                            status: QuestStatuses::InJourney.to_string(),
                        },
                    ])
                })
            });

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        let quest_viewing_use_case =
            QuestViewingUseCase::new(Arc::new(mock_quest_viewing_repository));

        let result = quest_viewing_use_case
            .board_checking(&BoardCheckingFilter::default())
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
    }
}

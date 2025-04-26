#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::usecases::quest_ops::QuestOpsUseCase,
        domain::{
            repositories::{
                quest_ops::MockQuestOpsRepository, quest_viewing::MockQuestViewingRepository,
            },
            value_objects::quest_model::{AddQuestModel, EditQuestModel},
        },
    };

    #[tokio::test]
    async fn test_add() {
        let mut mock_quest_ops_repository = MockQuestOpsRepository::new();
        let mock_quest_viewing_repository = MockQuestViewingRepository::new();

        mock_quest_ops_repository
            .expect_add()
            .returning(|_| Box::pin(async { Ok(1) }));

        let quest_ops_use_case = QuestOpsUseCase::new(
            Arc::new(mock_quest_ops_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let add_quest_model = AddQuestModel {
            name: "Test Quest".to_string(),
            description: Some("Test Description".to_string()),
        };

        let result = quest_ops_use_case.add(1, add_quest_model).await.unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_edit_success() {
        let mut mock_quest_ops_repository = MockQuestOpsRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(0) }));

        mock_quest_ops_repository
            .expect_edit()
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let quest_ops_use_case = QuestOpsUseCase::new(
            Arc::new(mock_quest_ops_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let edit_quest_model = EditQuestModel {
            name: None,
            description: Some("Test Description".to_string()),
        };

        let result = quest_ops_use_case
            .edit(1, 1, edit_quest_model)
            .await
            .unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_edit_failed() {
        let mock_quest_ops_repository = MockQuestOpsRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(1) }));

        let quest_ops_use_case = QuestOpsUseCase::new(
            Arc::new(mock_quest_ops_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let edit_quest_model = EditQuestModel {
            name: None,
            description: Some("Test Description".to_string()),
        };

        let result = quest_ops_use_case.edit(1, 1, edit_quest_model).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_success() {
        let mut mock_quest_ops_repository = MockQuestOpsRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(0) }));

        mock_quest_ops_repository
            .expect_remove()
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let quest_ops_use_case = QuestOpsUseCase::new(
            Arc::new(mock_quest_ops_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = quest_ops_use_case.remove(1, 1).await.unwrap();

        assert_eq!(result, ());
    }

    #[tokio::test]
    async fn test_remove_failed() {
        let mock_quest_ops_repository = MockQuestOpsRepository::new();
        let mut mock_quest_viewing_repository = MockQuestViewingRepository::new();

        mock_quest_viewing_repository
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(1) }));

        let quest_ops_use_case = QuestOpsUseCase::new(
            Arc::new(mock_quest_ops_repository),
            Arc::new(mock_quest_viewing_repository),
        );

        let result = quest_ops_use_case.remove(1, 1).await;

        assert!(result.is_err());
    }
}

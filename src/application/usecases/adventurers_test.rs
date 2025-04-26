#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::usecases::adventurers::AdventurersUseCase,
        domain::{
            repositories::adventurers::MockAdventurersRepository,
            value_objects::adventurer_model::RegisterAdventurerModel,
        },
    };

    #[tokio::test]
    async fn test_register_adventurer() {
        let mut mock_adventurers_repository = MockAdventurersRepository::new();

        mock_adventurers_repository
            .expect_register()
            .returning(|_| Box::pin(async { Ok(1) }));

        let adventurers_use_case = AdventurersUseCase::new(Arc::new(mock_adventurers_repository));

        let register_adventurer_model = RegisterAdventurerModel {
            username: "TestAdventurer".to_string(),
            password: "password!1234".to_string(),
        };

        let result = adventurers_use_case
            .register(register_adventurer_model)
            .await
            .unwrap();

        assert_eq!(result, 1);
    }
}

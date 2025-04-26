#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::usecases::guild_commanders::GuildCommandersUseCase,
        domain::{
            repositories::guild_commanders::MockGuildCommandersRepository,
            value_objects::guild_commander_model::RegisterGuildCommanderModel,
        },
    };

    #[tokio::test]
    async fn test_register_guild_commander() {
        let mut mock_guild_commanders_repository = MockGuildCommandersRepository::new();

        mock_guild_commanders_repository
            .expect_register()
            .returning(|_| Box::pin(async { Ok(1) }));

        let guild_commanders_use_case =
            GuildCommandersUseCase::new(Arc::new(mock_guild_commanders_repository));

        let register_guild_commander_model = RegisterGuildCommanderModel {
            username: "TestGuildCommander".to_string(),
            password: "password!1234".to_string(),
        };

        let result = guild_commanders_use_case
            .register(register_guild_commander_model)
            .await
            .unwrap();

        assert_eq!(result, 1);
    }
}

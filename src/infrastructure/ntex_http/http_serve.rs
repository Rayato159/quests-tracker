use std::sync::Arc;

use anyhow::Result;
use ntex::web::types::JsonConfig;
use ntex::{http, web};
use ntex_cors::Cors;

use crate::application::usecases::adventurers::AdventurersUseCase;
use crate::application::usecases::authentication::AuthenticationUseCase;
use crate::application::usecases::crew_switchboard::CrewSwitchboardUseCase;
use crate::application::usecases::guild_commanders::GuildCommandersUseCase;
use crate::application::usecases::journey_ledger::JourneyLedgerUseCase;
use crate::application::usecases::quest_ops::QuestOpsUseCase;
use crate::application::usecases::quest_viewing::QuestViewingUseCase;
use crate::infrastructure::ntex_http::default_routers::{health_check, not_found};
use crate::infrastructure::ntex_http::routers;
use crate::infrastructure::postgres::repositories::adventurers::AdventurersPostgres;
use crate::infrastructure::postgres::repositories::crew_switchboard::CrewSwitchboardPostgres;
use crate::infrastructure::postgres::repositories::diesel_transaction::DieselTransaction;
use crate::infrastructure::postgres::repositories::guild_commanders::GuildCommandersPostgres;
use crate::infrastructure::postgres::repositories::journey_ledger::JourneyLedgerPostgres;
use crate::infrastructure::postgres::repositories::quest_ops::QuestOpsPostgres;
use crate::infrastructure::postgres::repositories::quest_viewing::QuestViewingPostgres;
use crate::{
    config::config_model::DotEnvyConfig, infrastructure::postgres::postgres_connector::PgPoolSquad,
};

const SERVER_HOST: &'static str = "0.0.0.0";

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let port = config.server.port;
    let body_limit = config.server.body_limit;

    web::HttpServer::new(move || {
        let app_states = AppState::new(Arc::clone(&db_pool));
        web::App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec![
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PUT,
                        http::Method::PATCH,
                        http::Method::DELETE,
                    ])
                    .finish(),
            )
            .default_service(web::route().to(not_found))
            .route("/health-check", web::get().to(health_check))
            .state(JsonConfig::default().limit(body_limit as usize * 1024 * 1024))
            .state(Arc::clone(&app_states.adventurer_use_case))
            .state(Arc::clone(&app_states.authentication_use_case))
            .state(Arc::clone(&app_states.crew_switchboard_use_case))
            .state(Arc::clone(&app_states.guild_commander_use_case))
            .state(Arc::clone(&app_states.journey_ledger_use_case))
            .state(Arc::clone(&app_states.quest_ops_use_case))
            .state(Arc::clone(&app_states.quest_viewing_use_case))
            .configure(routers::adventurers::adventurers_routes)
            .configure(routers::authentication::authentication_routes)
            .configure(routers::crew_switchboard::crew_switchboard_routes)
            .configure(routers::guild_commanders::guild_commanders_routes)
            .configure(routers::journey_ledger::journey_ledger_routes)
            .configure(routers::quest_ops::quest_ops_routes)
            .configure(routers::quest_viewing::quest_viewing_routes)
    })
    .bind((SERVER_HOST, port))?
    .run()
    .await?;

    Ok(())
}

struct AppState {
    pub adventurer_use_case: Arc<AdventurersUseCase<AdventurersPostgres>>,
    pub authentication_use_case:
        Arc<AuthenticationUseCase<AdventurersPostgres, GuildCommandersPostgres>>,
    pub crew_switchboard_use_case: Arc<
        CrewSwitchboardUseCase<CrewSwitchboardPostgres, QuestViewingPostgres, DieselTransaction>,
    >,
    pub guild_commander_use_case: Arc<GuildCommandersUseCase<GuildCommandersPostgres>>,
    pub journey_ledger_use_case:
        Arc<JourneyLedgerUseCase<JourneyLedgerPostgres, QuestViewingPostgres>>,
    pub quest_ops_use_case: Arc<QuestOpsUseCase<QuestOpsPostgres, QuestViewingPostgres>>,
    pub quest_viewing_use_case: Arc<QuestViewingUseCase<QuestViewingPostgres>>,
}

impl AppState {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        let adventurers_repo = Arc::new(AdventurersPostgres::new(Arc::clone(&db_pool)));
        let crew_switchboard_repo = Arc::new(CrewSwitchboardPostgres::new(Arc::clone(&db_pool)));
        let tx = Arc::new(DieselTransaction::new(Arc::clone(&db_pool)));
        let guild_commanders_repo = Arc::new(GuildCommandersPostgres::new(Arc::clone(&db_pool)));
        let journey_ledger_repo = Arc::new(JourneyLedgerPostgres::new(Arc::clone(&db_pool)));
        let quest_ops_repo = Arc::new(QuestOpsPostgres::new(Arc::clone(&db_pool)));
        let quest_viewing_repo = Arc::new(QuestViewingPostgres::new(Arc::clone(&db_pool)));

        let adventurer_use_case = Arc::new(AdventurersUseCase::new(Arc::clone(&adventurers_repo)));
        let authentication_use_case = Arc::new(AuthenticationUseCase::new(
            Arc::clone(&adventurers_repo),
            Arc::clone(&guild_commanders_repo),
        ));
        let crew_switchboard_use_case = Arc::new(CrewSwitchboardUseCase::new(
            Arc::clone(&crew_switchboard_repo),
            Arc::clone(&quest_viewing_repo),
            Arc::clone(&tx),
        ));
        let guild_commander_use_case = Arc::new(GuildCommandersUseCase::new(Arc::clone(
            &guild_commanders_repo,
        )));
        let journey_ledger_use_case = Arc::new(JourneyLedgerUseCase::new(
            Arc::clone(&journey_ledger_repo),
            Arc::clone(&quest_viewing_repo),
        ));
        let quest_ops_use_case = Arc::new(QuestOpsUseCase::new(
            Arc::clone(&quest_ops_repo),
            Arc::clone(&quest_viewing_repo),
        ));
        let quest_viewing_use_case =
            Arc::new(QuestViewingUseCase::new(Arc::clone(&quest_viewing_repo)));

        AppState {
            adventurer_use_case,
            authentication_use_case,
            crew_switchboard_use_case,
            guild_commander_use_case,
            journey_ledger_use_case,
            quest_ops_use_case,
            quest_viewing_use_case,
        }
    }
}

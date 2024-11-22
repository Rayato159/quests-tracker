use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::{axum_http::routers, postgres::postgres_connector::PgPoolSquad},
};

use super::default_routers;

pub async fn start(config: &DotEnvyConfig, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .route("/health-check", get(default_routers::health_check))
        .nest(
            "/quest-ops",
            routers::quest_ops::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/quest-viewing",
            routers::quest_viewing::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/crew-switchboard",
            routers::crew_switchboard::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/journey-ledger",
            routers::journey_ledger::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/adventures",
            routers::adventures::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/guild-commanders",
            routers::guild_commanders::routes(Arc::clone(&db_pool)),
        )
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .fallback(default_routers::not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal"),
        _ = terminate => info!("Received terminate signal"),
    }
}

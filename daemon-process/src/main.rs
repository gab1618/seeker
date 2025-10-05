use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_core::server::DaemonServer;

use crate::log_config::setup_logging;

mod log_config;
use seeker_env::EnvArgs;

use elasticsearch_indexer::ElasticSearchIndexer;

#[tokio::main]
async fn main() {
    if let Err(err) = start_daemon().await {
        log::error!("{err}");
    }
}

async fn start_daemon() -> anyhow::Result<()> {
    setup_logging()?;
    let env_args = EnvArgs::load()?;
    let shared_indexer = Arc::new(ElasticSearchIndexer::new("localhost:5151", "a".into()));
    let listener = TcpListener::bind(&env_args.bind_url).await?;
    let server = DaemonServer::new(listener, shared_indexer.clone())?;
    log::info!("Server started at {}", env_args.bind_url);
    server.start().await;

    tokio::signal::ctrl_c().await?;
    log::info!("Server stopped");

    Ok(())
}

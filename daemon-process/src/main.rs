use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_core::server::DaemonServer;

use crate::log_config::setup_logging;

mod log_config;
use elasticsearch_indexer::ElasticSearchIndexer;
use seeker_env::EnvArgs;

#[tokio::main]
async fn main() {
    if let Err(err) = start_daemon().await {
        log::error!("{err}");
    }
}

const BIND_URL: &str = "127.0.0.1:5151";

async fn start_daemon() -> anyhow::Result<()> {
    setup_logging("/var/log/seekerd.log".into())?;
    let env_args = EnvArgs::load()?;
    let shared_indexer = Arc::new(ElasticSearchIndexer::new(
        &env_args.elasticsearch_cluster_url,
        env_args.elasticsearch_index_name,
    )?);
    let listener = TcpListener::bind(BIND_URL).await?;
    let server = DaemonServer::new(listener, shared_indexer.clone())?;
    log::info!("Daemon started at {}", BIND_URL);
    server.start().await;

    tokio::signal::ctrl_c().await?;
    log::info!("Server stopped");

    Ok(())
}

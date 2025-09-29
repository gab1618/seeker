use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_core::{indexer::Indexer, server::DaemonServer};

use crate::log_config::setup_logging;

mod log_config;
use seeker_env::EnvArgs;

// TODO: implement actual indexer
struct MockIndexer {}

#[async_trait::async_trait]
impl Indexer for MockIndexer {
    async fn index_file<'a>(
        &'a self,
        file_path: &'a std::path::Path,
    ) -> anyhow::Result<()> {
        println!("Indexing file: {}", file_path.display());

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    if let Err(err) = start_daemon().await {
        log::error!("{err}");
    }
}

async fn start_daemon() -> anyhow::Result<()> {
    setup_logging("/var/log/seekerd.log".into())?;
    let env_args = EnvArgs::load()?;
    let shared_indexer = Arc::new(MockIndexer {});
    let listener = TcpListener::bind(&env_args.bind_url).await?;
    let server = DaemonServer::new(listener, shared_indexer.clone())?;
    log::info!("Server started at {}", env_args.bind_url);
    server.start().await;

    tokio::signal::ctrl_c().await?;
    log::info!("Server stopped");

    Ok(())
}

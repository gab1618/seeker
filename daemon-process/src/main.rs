use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_core::{indexer::Indexer, server::DaemonServer};

use crate::{
    error::{DaemonProcessErr, DaemonProcessResult}, log_config::setup_logging
};

mod error;
mod log_config;
use seeker_env::EnvArgs;

// TODO: implement actual indexer
struct MockIndexer {}
impl Indexer for MockIndexer {
    fn index_file(
        &self,
        file_path: std::path::PathBuf,
    ) -> seeker_daemon_core::error::DaemonServerResult<()> {
        println!("Indexing file: {}", file_path.display());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> DaemonProcessResult<()> {
    let env_args = EnvArgs::load().unwrap();
    setup_logging()?;
    let shared_indexer = Arc::new(MockIndexer {});
    let listener = TcpListener::bind(&env_args.bind_url)
        .await
        .map_err(|_| DaemonProcessErr::SetupServer)?;
    let server = DaemonServer::new(listener, shared_indexer.clone())
        .map_err(|_| DaemonProcessErr::StartServer)?;
    log::info!("Server started at {}", env_args.bind_url);
    server.start().await;

    tokio::signal::ctrl_c()
        .await
        .map_err(|_| DaemonProcessErr::InterruptServer)?;
    log::info!("Server stopped");

    Ok(())
}

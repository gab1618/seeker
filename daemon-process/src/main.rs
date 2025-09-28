use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_core::{indexer::Indexer, server::DaemonServer};

use crate::log_config::{setup_logging, unwrap_log};

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
async fn main() {
    unwrap_log(setup_logging());
    let env_args = unwrap_log(EnvArgs::load());
    let shared_indexer = Arc::new(MockIndexer {});
    let listener = unwrap_log(TcpListener::bind("").await);
    let server = unwrap_log(DaemonServer::new(listener, shared_indexer.clone()));
    log::info!("Server started at {}", env_args.bind_url);
    server.start().await;

    unwrap_log(tokio::signal::ctrl_c().await);
    log::info!("Server stopped");
}

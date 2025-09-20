use std::sync::Arc;
use tokio::net::TcpListener;

use seeker_daemon_server_core::{
    error::DaemonServerResult, indexer::Indexer, server::DaemonServer,
};

// TODO: implement actual indexer
struct MockIndexer {}
impl Indexer for MockIndexer {
    fn index_file(
        &self,
        file_path: std::path::PathBuf,
    ) -> seeker_daemon_server_core::error::DaemonServerResult<()> {
        println!("Indexing file: {}", file_path.display());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> DaemonServerResult<()> {
    let shared_indexer = Arc::new(MockIndexer {});
    let listener = TcpListener::bind("127.0.0.1:5151")
        .await
        .expect("Could not bind to port 5151");
    let server = DaemonServer::new(listener, shared_indexer.clone())?;
    println!("Server started at port 5151");
    let _ = server.start().await;

    tokio::signal::ctrl_c()
        .await
        .expect("Could not interrupt server");
    println!("Server stopped");

    Ok(())
}

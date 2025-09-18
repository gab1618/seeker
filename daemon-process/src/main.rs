use std::{net::TcpListener, sync::Arc};

use daemon_server_core::{indexer::Indexer, server::SeekerDaemonServer};

// TODO: implement actual indexer
struct MockIndexer {}
impl Indexer for MockIndexer {
    fn index_file(&self, file_path: std::path::PathBuf) -> daemon_server_core::error::DaemonServerResult<()> {
        println!("Indexing file: {}", file_path.display());

        Ok(())
    }
}

async fn main() {
    let shared_indexer = Arc::new(MockIndexer{});
    let listener = TcpListener::bind("127.0.0.1:5151").unwrap();
    let server = SeekerDaemonServer::new(listener, shared_indexer.clone()).unwrap();
    let server_handle = server.start();
    println!("Server started at port 5151");
    server_handle.join().unwrap().unwrap();
}

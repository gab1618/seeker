use daemon_server_core::{indexer::Indexer, server::SeekerDaemonServer};
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};
use tokio::net::{TcpListener, TcpStream};

use crate::SeekerDaemonClient;

#[derive(Default)]
struct MockIndexer {
    index_calls_count: AtomicU8,
}

impl MockIndexer {
    pub fn get_curr_index_count(&self) -> u8 {
        self.index_calls_count.load(Ordering::Relaxed)
    }
}

impl Indexer for MockIndexer {
    fn index_file(
        &self,
        _file_path: std::path::PathBuf,
    ) -> daemon_server_core::error::DaemonServerResult<()> {
        self.index_calls_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

const TEST_URL: &'static str = "127.0.0.1:5151";

async fn setup_server() -> Arc<MockIndexer> {
    let shared_indexer = Arc::new(MockIndexer::default());
    let server_listener = TcpListener::bind(TEST_URL)
        .await
        .expect("Could not start server listener");
    let server = SeekerDaemonServer::new(server_listener, shared_indexer.clone())
        .expect("Could not create server");

    server.start().await;

    shared_indexer
}

#[tokio::test]
async fn test_index_req() {
    let shared_indexer = setup_server().await;
    let client_conn = TcpStream::connect(TEST_URL).await.unwrap();
    let mut client = SeekerDaemonClient::new(client_conn).unwrap();
    assert_eq!(shared_indexer.get_curr_index_count(), 0);

    client.index_file("./text.txt".into()).await.unwrap();
    assert_eq!(shared_indexer.get_curr_index_count(), 1);
}

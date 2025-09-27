use seeker_daemon_core::{error::DaemonServerResult, indexer::Indexer, server::DaemonServer};
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};
use tokio::net::{TcpListener, TcpStream};

use crate::DaemonClient;

#[derive(Default)]
struct MockIndexer {
    index_calls_count: AtomicU8,
}

impl MockIndexer {
    pub fn get_curr_index_count(&self) -> u8 {
        self.index_calls_count.load(Ordering::Relaxed)
    }
}

#[async_trait::async_trait]
impl Indexer for MockIndexer {
    async fn index_file<'a>(&'a self, _file_path: std::path::PathBuf) -> DaemonServerResult<()> {
        self.index_calls_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

const TEST_URL: &str = "127.0.0.1:5151";

async fn setup_server() -> Arc<MockIndexer> {
    let shared_indexer = Arc::new(MockIndexer::default());
    let server_listener = TcpListener::bind(TEST_URL)
        .await
        .expect("Could not start server listener");
    let server = DaemonServer::new(server_listener, shared_indexer.clone())
        .expect("Could not create server");

    server.start().await;

    shared_indexer
}

#[tokio::test]
async fn test_index_req() {
    let shared_indexer = setup_server().await;
    let client_conn = TcpStream::connect(TEST_URL).await.unwrap();
    let mut client = DaemonClient::new(client_conn);
    assert_eq!(shared_indexer.get_curr_index_count(), 0);

    client.index_file("./text.txt".into()).await.unwrap();
    assert_eq!(shared_indexer.get_curr_index_count(), 1);
}

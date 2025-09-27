use serde_json::json;
use std::{fs::OpenOptions, io::Read};

use elasticsearch::{Elasticsearch, IndexParts, http::transport::Transport};
use seeker_daemon_core::{error::DaemonServerError, indexer::Indexer};

pub struct ElasticSearchIndexer {
    client: Elasticsearch,
    index_name: String,
}

impl ElasticSearchIndexer {
    pub fn new(cluster_url: &str, index_name: String) -> Self {
        let t = Transport::single_node(cluster_url).unwrap();
        let client = Elasticsearch::new(t);
        Self { client, index_name }
    }
}

#[async_trait::async_trait]
impl Indexer for ElasticSearchIndexer {
    async fn index_file<'a>(
        &'a self,
        file_path: std::path::PathBuf,
    ) -> seeker_daemon_core::error::DaemonServerResult<()> {
        let file_id = file_path.as_os_str().to_str().unwrap();
        let mut r = OpenOptions::new().read(true).open(&file_path).unwrap();
        let mut file_content = String::new();
        r.read_to_string(&mut file_content).unwrap();
        let filename = file_path.file_name().unwrap().to_str().unwrap();
        self.client
            .index(IndexParts::IndexId(&self.index_name, file_id))
            .body(json! ({
                "id": file_id,
                "name": filename,
                "content": file_content,
            }));
        Err(DaemonServerError::Unreachable)
    }
}

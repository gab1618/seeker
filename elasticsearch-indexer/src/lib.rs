use serde_json::json;

use elasticsearch::{Elasticsearch, IndexParts, http::transport::Transport};
use seeker_daemon_core::indexer::Indexer;

use crate::error::ElasticIndexerErr;

mod error;

pub struct ElasticSearchIndexer {
    client: Option<Elasticsearch>,
    index_name: String,
}

impl ElasticSearchIndexer {
    pub fn new(cluster_url: &str, index_name: String) -> anyhow::Result<Self> {
        let t = Transport::single_node(cluster_url).ok();

        let client = t.map(Elasticsearch::new);

        Ok(Self { client, index_name })
    }
}

#[async_trait::async_trait]
impl Indexer for ElasticSearchIndexer {
    async fn index_file(&self, file_path: String, content: String) -> anyhow::Result<()> {
        let filename = file_path.split("/").collect::<Vec<&str>>().pop().unwrap();

        //TODO: when client is unavailable, save the request locally.
        if let Some(c) = &self.client {
            c.index(IndexParts::IndexId(&self.index_name, &file_path))
                .body(json! ({
                    "id": file_path,
                    "name": filename,
                    "content": content,
                }));
        }
        Ok(())
    }
}

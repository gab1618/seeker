use serde_json::json;
use std::path::PathBuf;

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
    async fn index_file(&self, file_path: PathBuf, content: String) -> anyhow::Result<()> {
        let file_id = file_path
            .as_os_str()
            .to_str()
            .ok_or(ElasticIndexerErr::GenerateFileId)?;
        let filename = file_path
            .file_name()
            .ok_or(ElasticIndexerErr::GetFileName)?
            .to_str()
            .ok_or(ElasticIndexerErr::GetFileName)?;

        //TODO: when client is unavailable, save the request locally.
        if let Some(c) = &self.client {
            c.index(IndexParts::IndexId(&self.index_name, file_id))
                .body(json! ({
                    "id": file_id,
                    "name": filename,
                    "content": content,
                }));
        }
        Ok(())
    }
}

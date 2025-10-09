use std::path::PathBuf;

#[async_trait::async_trait]
pub trait Indexer {
    async fn index_file(&self, file_path: PathBuf, content: String) -> anyhow::Result<()>;
}

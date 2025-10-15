#[async_trait::async_trait]
pub trait Indexer {
    async fn index_file(&self, file_path: String, content: String) -> anyhow::Result<()>;
}

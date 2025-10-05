use std::path::Path;

#[async_trait::async_trait]
pub trait Indexer {
    async fn index_file<'a>(&'a self, file_path: &'a Path) -> anyhow::Result<()>;
}

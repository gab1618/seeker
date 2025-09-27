use std::path::PathBuf;

use crate::error::DaemonServerResult;

#[async_trait::async_trait]
pub trait Indexer {
    async fn index_file<'a>(&'a self, file_path: PathBuf) -> DaemonServerResult<()>;
}

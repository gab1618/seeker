use std::path::PathBuf;

use crate::error::DaemonServerResult;

pub trait Indexer {
    fn index_file(&self, file_path: PathBuf) -> DaemonServerResult<()>;
}

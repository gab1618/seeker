use std::path::PathBuf;

pub trait Indexer {
    fn index_file(&self, file_path: PathBuf) -> anyhow::Result<()>;
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElasticIndexerErr {
    #[error("Could not generate file id")]
    GenerateFileId,
    #[error("Could not open file: {0}")]
    OpenFile(#[source] std::io::Error),
    #[error("Could not read file: {0}")]
    ReadFile(#[source] std::io::Error),
    #[error("Could not get file name")]
    GetFileName,
}

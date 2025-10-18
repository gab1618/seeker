use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElasticIndexerErr {
    #[error("Could not get file name")]
    GetFileName,
}

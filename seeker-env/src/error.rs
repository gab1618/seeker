use std::env::VarError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeekerEnvErr {
    #[error("Could not load daemon bind url: {0}")]
    LoadDaemonBindUrl(#[source] VarError),
    #[error("Could not load elasticsearch cluster url: {0}")]
    LoadESClusterUrl(#[source] VarError),
    #[error("Could not load elasticsearch index name: {0}")]
    LoadESIndexName(#[source] VarError),
}

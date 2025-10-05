use std::env::VarError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeekerEnvErr {
    #[error("Could not load daemon bind url: {0}")]
    LoadDaemonBindUrl(#[source] VarError),
    #[error("Could not load elastic search cluster url: {0}")]
    LoadElasticSearchClusterUrl(#[source] VarError),
}

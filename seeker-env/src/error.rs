use std::env::VarError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeekerEnvErr {
    #[error("Could not load daemon bind url: {source}")]
    LoadDaemonBindUrl {
        #[from]
        source: VarError,
    },
}

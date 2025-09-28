use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeekerEnvErr {
    #[error("Could not load daemon bind url")]
    LoadDaemonBindUrl,
}

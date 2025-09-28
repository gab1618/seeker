use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonProcessErr {
    #[error("Could not setup logger")]
    SetupLogger,
    #[error("Could not start logger")]
    StartLogger,
}

pub type DaemonProcessResult<T> = Result<T, DaemonProcessErr>;

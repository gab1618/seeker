use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonClientErr {
    #[error("Could not send index file request")]
    SendIndexReq,
    #[error("Could not receive daemon response")]
    RecvServerResponse,
    #[error("Could not parse server response")]
    ParseServerResponse,
    #[error("Unreachable")]
    Unreacheable,
}

pub type DaemonClientResult<T> = Result<T, DaemonClientErr>;

use seeker_daemon_core::error::DaemonServerError;
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DaemonClientErr {
    #[error("Could not send index file request: {0}")]
    SendIndexReq(#[source] io::Error),
    #[error("Could not receive daemon response: {0}")]
    RecvServerResponse(#[source] io::Error),
    #[error("Could not parse server response: {0}")]
    ParseServerResponse(#[source] DaemonServerError),
}

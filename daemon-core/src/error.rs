use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DaemonServerError {
    #[error("Could not parse command")]
    ParseCommand,
    #[error("Could not read request: {0}")]
    ReadRequest(#[source] io::Error),
    #[error("Could not send response: {0}")]
    SendResponse(#[source] io::Error),
    #[error("Could not parse response")]
    ParseResponse,
}

use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DaemonServerError {
    #[error("Could not parse command")]
    ParseCommand,
    #[error("Could not read request")]
    ReadRequest(#[source] io::Error),
    #[error("Could not send response")]
    SendResponse(#[source] io::Error),
    #[error("Could not start server")]
    StartServer,
    #[error("Could not parse response")]
    ParseResponse,
}

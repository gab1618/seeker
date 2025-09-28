use std::fmt;

#[derive(Debug)]
pub enum DaemonServerError {
    ParseCommand,
    Unreachable,
    ReadRequest,
    SendResponse,
    StartServer,
    ParseResponse,
}

pub type DaemonServerResult<T> = Result<T, DaemonServerError>;

impl fmt::Display for DaemonServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonServerError::ParseCommand => write!(f, "Could not parse command"),
            DaemonServerError::Unreachable => write!(f, "UNREACHABLE"),
            DaemonServerError::ReadRequest => write!(f, "Could not read request"),
            DaemonServerError::SendResponse => write!(f, "Could not send response"),
            DaemonServerError::StartServer => write!(f, "Could not start server"),
            DaemonServerError::ParseResponse => write!(f, "Could not parse response"),
        }
    }
}

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

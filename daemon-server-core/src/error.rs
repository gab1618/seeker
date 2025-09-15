#[derive(Debug)]
pub enum DaemonServerError {
    ParseCommand,
    Unreachable,
    ReadRequest,
    SendResponse,
    StartServer,
}

pub type DaemonServerResult<T> = Result<T, DaemonServerError>;

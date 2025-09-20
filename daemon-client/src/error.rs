#[derive(Debug)]
pub enum DaemonClientErr {
    SendIndexReq,
    RecvServerResponse,
    ParseServerResponse,
    Unreacheable,
}

pub type DaemonClientResult<T> = Result<T, DaemonClientErr>;

#[derive(Debug)]
pub enum DaemonProcessErr {
    SetupLogger,
    StartLogger,
    SetupServer,
    StartServer,
    InterruptServer,
}

pub type DaemonProcessResult<T> = Result<T, DaemonProcessErr>;

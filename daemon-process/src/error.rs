use std::fmt;

#[derive(Debug)]
pub enum DaemonProcessErr {
    SetupLogger,
    StartLogger,
    SetupServer,
    StartServer,
    InterruptServer,
}

pub type DaemonProcessResult<T> = Result<T, DaemonProcessErr>;

impl fmt::Display for DaemonProcessErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonProcessErr::SetupLogger => write!(f, "Could not setup logger"),
            DaemonProcessErr::StartLogger => write!(f, "Could not start logger"),
            DaemonProcessErr::SetupServer => write!(f, "Could not setup server"),
            DaemonProcessErr::StartServer => write!(f, "Could not start server"),
            DaemonProcessErr::InterruptServer => write!(f, "Could not interrupt server"),
        }
    }
}

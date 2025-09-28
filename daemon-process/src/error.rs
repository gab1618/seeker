use std::fmt;

#[derive(Debug)]
pub enum DaemonProcessErr {
    SetupLogger,
    StartLogger,
}

pub type DaemonProcessResult<T> = Result<T, DaemonProcessErr>;

impl fmt::Display for DaemonProcessErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonProcessErr::SetupLogger => write!(f, "Could not setup logger"),
            DaemonProcessErr::StartLogger => write!(f, "Could not start logger"),
        }
    }
}

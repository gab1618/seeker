#[derive(Debug)]
pub enum SeekerHookErr {
    InvalidGitArgs,
    SetupLogFile,
    SaveState,
    IndexFile,
    StartDaemonClient,
}

pub type SeekerHookResult<T> = Result<T, SeekerHookErr>;

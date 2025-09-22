#[derive(Debug)]
pub enum SeekerHookErr {
    InvalidGitArgs,
    SetupLogFile,
    SaveState,
}

pub type SeekerHookResult<T> = Result<T, SeekerHookErr>;

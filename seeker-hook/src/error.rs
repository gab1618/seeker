#[derive(Debug)]
pub enum SeekerHookErr {
    InvalidGitArgs,
    SetupLogFile,
    ApplyLogConfig
}

pub type SeekerHookResult<T> = Result<T, SeekerHookErr>;

#[derive(Debug)]
pub enum SeekerHookErr {
    InvalidGitArgs,
    SetupLogFile,
    SaveState,
    IndexFile,
    StartDaemonClient,
    OpenRepo,
    GetRepoLastCommit,
    GetRepoHead,
    ParseRepoOid,
    GetRepoTree,
    FindRepoCommit,
    GetRepoDiff,
    UpdateRepoHead,
}

pub type SeekerHookResult<T> = Result<T, SeekerHookErr>;

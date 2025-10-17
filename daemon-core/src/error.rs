use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DaemonServerError {
    #[error("Could not parse command")]
    ParseCommand,
    #[error("Could not read request: {0}")]
    ReadRequest(#[source] io::Error),
    #[error("Could not send response: {0}")]
    SendResponse(#[source] io::Error),
    #[error("Could not parse response")]
    ParseResponse,
    #[error("Could not open repository: {0}")]
    OpenRepo(#[source] git2::Error),
    #[error("Could not get last commit: {0}")]
    GetLastCommit(#[source] git2::Error),
    #[error("Could not get changes: {0}")]
    GetChanges(#[source] git2::Error),
    #[error("Could not parse changes")]
    ParseChanges,
    #[error("Could not get file path")]
    GetFilePath,
    #[error("Could not save state: {0}")]
    SaveStateValue(#[source] io::Error),
    #[error("Could not update head: {0}")]
    UpdateRepoHead(#[source] git2::Error),
    #[error("Could not get repo latest ref")]
    GetRepoLatestRef,
    #[error("Could not get repo refs: {0}")]
    GetRepoRefs(#[source] git2::Error),
    #[error("Could not setup state dir: {0}")]
    SetupStateDir(#[source] io::Error),
}

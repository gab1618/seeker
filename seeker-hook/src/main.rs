use std::path::PathBuf;

use changes_tracker::ChangesTracker;
use setup_repo::setup_repo;
use state_manager::StateManager;

use crate::{
    args::GitArgs,
    daemon_client::get_daemon_client,
    error::{SeekerHookErr, SeekerHookResult},
    state_manager::StateValue,
};

#[cfg(test)]
mod test;

mod args;
mod changes_tracker;
mod daemon_client;
mod setup_repo;
mod state_manager;

pub mod error;

#[tokio::main]
async fn main() -> SeekerHookResult<()> {
    let stdin = std::io::stdin();
    let stdin_line = stdin
        .lines()
        .next()
        .expect("Could not read hook input")
        .expect("Could not read hook input");

    let git_args = GitArgs::try_from(stdin_line.as_str())?;

    let repo_path: PathBuf = std::env::args().nth(1).unwrap_or(".".to_string()).into();

    setup_repo(&repo_path, &git_args).unwrap();

    let manager = StateManager::new(repo_path.clone());
    let tracker = ChangesTracker::new(repo_path, &manager).expect("Could not get file changes");

    let mut daemon_client = get_daemon_client().await?;

    for entry in tracker.get_changed_files().unwrap() {
        let (filepath, _) = entry.unwrap();
        daemon_client
            .index_file(filepath)
            .await
            .map_err(|_| SeekerHookErr::IndexFile)?;
    }

    manager.save_state_value(StateValue::LastIndexedCommit, &git_args.new_rev)?;

    Ok(())
}

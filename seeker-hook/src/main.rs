use std::path::PathBuf;

use changes_tracker::ChangesTracker;
use logger::Logger;
use setup_repo::setup_repo;
use state_manager::StateManager;

use crate::{args::GitArgs, error::SeekerHookResult};

#[cfg(test)]
mod test;

mod args;
mod changes_tracker;
mod logger;
mod setup_repo;
mod state_manager;

pub mod error;

fn main() -> SeekerHookResult<()> {
    let stdin = std::io::stdin();
    let stdin_line = stdin
        .lines()
        .next()
        .expect("Could not read hook input")
        .expect("Could not read hook input");

    let git_args = GitArgs::try_from(stdin_line.as_str())?;

    log::info!("Starting setup");
    let repo_path: PathBuf = std::env::args().nth(1).unwrap_or(".".to_string()).into();

    setup_repo(&repo_path, git_args).unwrap();

    Logger::setup_logging(repo_path.join("info").join("log"))?;

    let manager = StateManager::new(repo_path.clone());
    let tracker = ChangesTracker::new(repo_path, &manager).expect("Could not get file changes");

    log::info!("Setup done. Starting the index routine");

    for entry in tracker.get_changed_files().unwrap() {
        let (filepath, _) = entry.unwrap();
        log::info!("Indexing file {}", filepath.to_str().unwrap());
    }

    Ok(())
}

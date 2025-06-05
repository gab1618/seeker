use std::{io::BufRead, path::PathBuf};

use args::parse_git_args;
use changes_tracker::ChangesTracker;
use logger::Logger;
use state_manager::StateManager;

#[cfg(test)]
mod test;

mod args;
mod changes_tracker;
mod logger;
mod state_manager;

fn main() {
    let stdin = std::io::stdin();
    let git_args = parse_git_args(&stdin).expect("Could not parse args");
    drop(stdin);

    log::info!("Starting setup");
    let repo_path: PathBuf = std::env::args().nth(1).unwrap_or(".".to_string()).into();
    Logger::setup_logging(repo_path.join("info").join("log")).unwrap();

    let manager = StateManager::new(repo_path.clone());
    let tracker = ChangesTracker::new(repo_path, &manager);

    log::info!("Setup done. Starting the index routine");
}

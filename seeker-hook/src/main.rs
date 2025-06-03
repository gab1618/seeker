use std::path::PathBuf;

use logger::Logger;

#[cfg(test)]
mod test;

mod changes_tracker;
mod logger;
mod state_manager;

fn main() {
    log::info!("Starting setup");
    let repo_path: PathBuf = std::env::args().nth(1).unwrap_or(".".to_string()).into();
    Logger::setup_logging(repo_path.join("info").join("log")).unwrap();

    log::info!("Setup done. Starting the index routine");
    log::info!("Indexing routine not implemented yet. Terminating the execution");
}

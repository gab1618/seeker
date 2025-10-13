use tempfile::tempdir;

use crate::{changes_tracker::{test::utils::TestRepo, ChangesTracker}, state::State};

mod utils;

#[test]
fn test_get_latest_changes() {
    let test_repo = TestRepo::new().unwrap();
    let state_dir = tempdir().unwrap();
    let state_manager = State::new(state_dir.path().to_path_buf());
    let _tracker = ChangesTracker::new(test_repo.bare_repo().unwrap(), &state_manager);
}

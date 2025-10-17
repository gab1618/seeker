use tempfile::tempdir;

use crate::{changes_tracker::{test::utils::TestRepo, ChangesTracker}, state::State};

mod utils;

#[test]
fn test_get_latest_changes() {
    let test_repo = TestRepo::new().unwrap();
    let state_dir = tempdir().unwrap();
    let state_manager = State::new(state_dir.path().to_path_buf()).unwrap();
    test_repo.commit_clone("README.md", "## Hello!", "First commit").unwrap();
    test_repo.push_remote().unwrap();
    let tracker = ChangesTracker::new(test_repo.bare_repo().unwrap(), &state_manager);
    let mut changes = tracker.get_changed_files().unwrap();
    let (file_path, file_content) = changes.next().unwrap();

    assert_eq!(file_path, "README.md".to_string());
    assert_eq!(file_content, "## Hello!".to_string());
}

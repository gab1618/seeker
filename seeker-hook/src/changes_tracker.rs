use git2::{self, DiffOptions, Repository};
use std::path::{Path, PathBuf};

use crate::state_manager::StateManager;

pub struct ChangesTracker<'a, P: AsRef<Path>> {
    mount_path: P,
    state_manager: &'a StateManager,
}

impl<'a, P: AsRef<Path>> ChangesTracker<'a, P> {
    pub fn new(mount_path: P, state_manager: &'a StateManager) -> Self {
        Self {
            mount_path,
            state_manager,
        }
    }

    pub fn get_changed_files(&self) -> Vec<(PathBuf, Vec<u8>)> {
        let last_indexed_commit = self
            .state_manager
            .get_state_file_value(crate::state_manager::StateValue::LastIndexedCommit);
        let repo = Repository::open_bare(&self.mount_path).unwrap();

        let last_commit_oid = repo.head().unwrap().peel_to_commit().unwrap().id();

        let last_indexed_commit_oid = match last_indexed_commit {
            None => None,
            Some(last_indexed_commit) => {
                Some(repo.revparse_single(&last_indexed_commit).unwrap().id())
            }
        };

        let start_commit = repo.find_commit(last_commit_oid).unwrap();

        let end_commit = match last_indexed_commit_oid {
            None => None,
            Some(last_indexed_commit_oid) => {
                Some(repo.find_commit(last_indexed_commit_oid).unwrap())
            }
        };

        let start_tree = start_commit.tree().unwrap();
        let end_tree = end_commit.map(|c| c.tree().unwrap());

        let mut diff_options = DiffOptions::new();
        diff_options.ignore_whitespace(true);
        diff_options.ignore_submodules(true);
        diff_options.minimal(true);

        let diff = repo
            .diff_tree_to_tree(
                Some(&start_tree),
                end_tree.as_ref(),
                Some(&mut diff_options),
            )
            .unwrap();

        let changed_files = diff
            .deltas()
            .map(|delta| {
                let path = delta.new_file().path().unwrap();
                let diff_commit = repo.find_commit(start_commit.id()).unwrap();
                let commit_tree = diff_commit.tree().unwrap();
                let entry = commit_tree.get_path(path).unwrap();
                let blob = repo.find_blob(entry.id()).unwrap();
                (path.to_owned(), blob.content().to_vec())
            })
            .collect();

        changed_files
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::test::utils::setup_repo::setup_repo;

    use super::*;
    #[test]
    fn test_get_changed_files() {
        let state_dir = tempdir().unwrap();
        let (_, bare_repo_dir) = setup_repo().unwrap();
        let state_manager = StateManager::new(state_dir.path().to_owned());
        let tracker = ChangesTracker::new(bare_repo_dir.path().to_owned(), &state_manager);

        let changed_files = tracker.get_changed_files();
        assert!(changed_files.len() > 0);

        let expected_file_content = "testing";

        let (_, first_change) = &changed_files[0];

        assert_eq!(
            first_change.clone(),
            expected_file_content.bytes().collect::<Vec<u8>>()
        );
    }
}

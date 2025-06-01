use git2::{self, DiffOptions, Repository};
use std::path::PathBuf;

use crate::state_manager::StateManager;

pub struct ChangesTracker<'a> {
    mount_path: PathBuf,
    state_manager: &'a StateManager,
}

impl<'a> ChangesTracker<'a> {
    pub fn new(mount_path: PathBuf, state_manager: &'a StateManager) -> Self {
        Self {
            mount_path,
            state_manager,
        }
    }

    pub fn get_changed_files(&self) -> Vec<PathBuf> {
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

        let mut changed_files = Vec::new();

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    changed_files.push(path.to_owned());
                }
                true
            },
            None,
            None,
            None,
        )
        .unwrap();

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
    }
}

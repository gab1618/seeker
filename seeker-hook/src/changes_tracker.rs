use git2::{self, DiffOptions, Oid, Repository};
use std::path::{Path, PathBuf};

use crate::state_manager::StateManager;

#[derive(Debug)]
pub enum ChangesTrackerErr {
    OpenRepo,
    GetLastCommit,
    ParseOid,
    GetTree,
    FindCommit,
    GetDiff,
}

pub struct ChangesTracker<'a> {
    state_manager: &'a StateManager,
    repo: Repository,
}

type ChangeEntry = Result<(PathBuf, Vec<u8>), ChangesTrackerErr>;

impl<'a> ChangesTracker<'a> {
    pub fn new<P: AsRef<Path>>(
        mount_path: P,
        state_manager: &'a StateManager,
    ) -> Result<Self, ChangesTrackerErr> {
        Ok(Self {
            repo: Repository::open_bare(mount_path).map_err(|_| ChangesTrackerErr::OpenRepo)?,
            state_manager,
        })
    }

    pub fn get_changed_files(
        &'a self,
    ) -> Result<impl Iterator<Item = ChangeEntry> + 'a, ChangesTrackerErr> {
        let last_indexed_commit = self
            .state_manager
            .get_state_file_value(crate::state_manager::StateValue::LastIndexedCommit);

        let last_commit_oid = self
            .repo
            .head()
            .map_err(|_| ChangesTrackerErr::GetLastCommit)?
            .peel_to_commit()
            .map_err(|_| ChangesTrackerErr::GetLastCommit)?
            .id();

        let last_indexed_commit_oid = last_indexed_commit
            .map(|id| Oid::from_str(&id).map_err(|_| ChangesTrackerErr::ParseOid))
            .map_or(Ok(None), |v| v.map(Some))?;

        let start_commit = self
            .repo
            .find_commit(last_commit_oid)
            .map_err(|_| ChangesTrackerErr::FindCommit)?;
        let end_commit = last_indexed_commit_oid
            .map(|obj| {
                self.repo
                    .find_commit(obj)
                    .map_err(|_| ChangesTrackerErr::FindCommit)
            })
            .map_or(Ok(None), |v| v.map(Some))?;

        let start_tree = start_commit
            .tree()
            .map_err(|_| ChangesTrackerErr::GetTree)?;
        let end_tree = end_commit
            .map(|c| c.tree().map_err(|_| ChangesTrackerErr::GetTree))
            .map_or(Ok(None), |v| v.map(Some))?;

        let mut diff_options = DiffOptions::new();
        diff_options.ignore_whitespace(true);
        diff_options.ignore_submodules(true);
        diff_options.minimal(true);

        let diff = self
            .repo
            .diff_tree_to_tree(
                Some(&start_tree),
                end_tree.as_ref(),
                Some(&mut diff_options),
            )
            .map_err(|_| ChangesTrackerErr::GetDiff)?;

        // Dereference changed paths, since their lifetimes are still attached to the diff object
        let changed_paths: Vec<PathBuf> = diff
            .deltas()
            .filter_map(|delta| delta.new_file().path().map(|p| p.to_owned()))
            .collect();

        Ok(changed_paths.into_iter().map(move |path| {
            let diff_commit = self
                .repo
                .find_commit(start_commit.id())
                .map_err(|_| ChangesTrackerErr::GetDiff)?;
            let commit_tree = diff_commit.tree().map_err(|_| ChangesTrackerErr::GetDiff)?;
            let entry = commit_tree
                .get_path(&path)
                .map_err(|_| ChangesTrackerErr::GetDiff)?;
            let blob = self
                .repo
                .find_blob(entry.id())
                .map_err(|_| ChangesTrackerErr::GetDiff)?;
            Ok((path.to_owned(), blob.content().to_vec()))
        }))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::{state_manager::StateValue, test::utils::test_repo::TestRepo};

    use super::*;
    #[test]
    fn test_get_changed_files() {
        let state_dir = tempdir().unwrap();
        let test_repo = TestRepo::new().unwrap();
        test_repo
            .commit_and_push_change("doc.md", "testing", "first commit")
            .unwrap();
        let bare_repo_dir = test_repo.bare_dir();
        let state_manager = StateManager::new(state_dir.path().to_owned());
        let tracker = ChangesTracker::new(bare_repo_dir.path(), &state_manager).unwrap();

        let mut changed_files = tracker.get_changed_files().unwrap();
        let first_entry = changed_files.next().unwrap().unwrap();

        let expected_file_content = "testing";

        let (_, first_change) = first_entry;

        assert_eq!(
            first_change.clone(),
            expected_file_content.bytes().collect::<Vec<u8>>()
        );
    }

    #[test]
    fn test_get_new_changes() {
        // Mark first commit as indexed, so the tracker should only bring the second commit as a
        // change
        let state_dir = tempdir().unwrap();
        let state_manager = StateManager::new(state_dir.path().to_owned());
        let test_repo = TestRepo::new().unwrap();
        let first_commit = test_repo
            .commit_and_push_change("doc.md", "testing", "first commit")
            .unwrap();
        state_manager
            .save_state_value(StateValue::LastIndexedCommit, &first_commit)
            .unwrap();

        let expected_file_content = "second";
        test_repo
            .commit_and_push_change("doc-1.md", expected_file_content, "second commit")
            .unwrap();
        let bare_repo_dir = test_repo.bare_dir();
        let tracker = ChangesTracker::new(bare_repo_dir.path(), &state_manager).unwrap();

        let mut changed_files = tracker.get_changed_files().unwrap();
        let first_entry = changed_files.next().unwrap().unwrap();

        let (_, first_change) = first_entry;

        assert_eq!(
            first_change.clone(),
            expected_file_content.bytes().collect::<Vec<u8>>()
        );
    }
}

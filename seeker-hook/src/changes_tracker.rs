use git2::{self, DiffOptions, Oid, Repository};
use std::path::{Path, PathBuf};

use crate::{
    error::SeekerHookErr,
    state_manager::{StateManager, StateValue},
};

pub struct ChangesTracker<'a> {
    state_manager: &'a StateManager,
    repo: Repository,
}

type ChangeEntry = Result<(PathBuf, Vec<u8>), SeekerHookErr>;

impl<'a> ChangesTracker<'a> {
    pub fn new<P: AsRef<Path>>(
        mount_path: P,
        state_manager: &'a StateManager,
    ) -> Result<Self, SeekerHookErr> {
        Ok(Self {
            repo: Repository::open_bare(mount_path).map_err(|_| SeekerHookErr::OpenRepo)?,
            state_manager,
        })
    }

    pub fn get_changed_files(&'a self) -> Result<Vec<String>, SeekerHookErr> {
        let last_indexed_commit = self
            .state_manager
            .get_state_file_value(StateValue::LastIndexedCommit);

        let last_commit_oid = self
            .repo
            .head()
            .map_err(|_| SeekerHookErr::GetRepoHead)?
            .peel_to_commit()
            .map_err(|_| SeekerHookErr::GetRepoLastCommit)?
            .id();

        let last_indexed_commit_oid = last_indexed_commit
            .map(|id| Oid::from_str(&id).map_err(|_| SeekerHookErr::ParseRepoOid))
            .map_or(Ok(None), |v| v.map(Some))?;

        let start_commit = self
            .repo
            .find_commit(last_commit_oid)
            .map_err(|_| SeekerHookErr::FindRepoCommit)?;
        let end_commit = last_indexed_commit_oid
            .map(|obj| {
                self.repo
                    .find_commit(obj)
                    .map_err(|_| SeekerHookErr::FindRepoCommit)
            })
            .map_or(Ok(None), |v| v.map(Some))?;

        let start_tree = start_commit
            .tree()
            .map_err(|_| SeekerHookErr::GetRepoTree)?;
        let end_tree = end_commit
            .map(|c| c.tree().map_err(|_| SeekerHookErr::GetRepoTree))
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
            .map_err(|_| SeekerHookErr::GetRepoDiff)?;

        let changed_oids: Vec<String> = diff
            .deltas()
            .map(|delta| delta.new_file().id().to_string())
            .collect();

        Ok(changed_oids)
    }
}


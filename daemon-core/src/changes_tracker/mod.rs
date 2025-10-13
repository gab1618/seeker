use git2::{self, Repository};
use std::path::{Path, PathBuf};

use crate::state::State;

#[cfg(test)]
mod test;

pub struct ChangesTracker<'a> {
    state: &'a State,
    repo: Repository,
}

type ChangeEntry = (PathBuf, String);

impl<'a> ChangesTracker<'a> {
    pub fn new(repo: Repository, state_manager: &'a State) -> Self {
        Self {
            repo,
            state: state_manager,
        }
    }
    pub fn get_changed_files(&'a self) -> anyhow::Result<Vec<ChangeEntry>> {
        let changed_files: Vec<ChangeEntry> = vec![];

        Ok(changed_files)
    }
}

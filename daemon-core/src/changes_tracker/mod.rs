use git2::{self, Repository};
use std::path::{Path, PathBuf};

use crate::{error::DaemonServerError, state::State};

#[cfg(test)]
mod test;

pub struct ChangesTracker<'a> {
    state: &'a State,
    repo: Repository,
}

type ChangeEntry = (PathBuf, String);

impl<'a> ChangesTracker<'a> {
    pub fn new<P: AsRef<Path>>(mount_path: P, state_manager: &'a State) -> anyhow::Result<Self> {
        Ok(Self {
            repo: Repository::open_bare(mount_path).map_err(DaemonServerError::OpenRepo)?,
            state: state_manager,
        })
    }

    pub fn get_changed_files(&'a self) -> anyhow::Result<Vec<ChangeEntry>> {
        let changed_files: Vec<ChangeEntry> = vec![];

        Ok(changed_files)
    }
}

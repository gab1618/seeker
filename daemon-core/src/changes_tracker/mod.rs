use crate::{
    error::DaemonServerError,
    state::{State, StateValue},
};
use git2::{self, Commit, Oid, Repository};

#[cfg(test)]
mod test;

pub struct ChangesTracker<'a> {
    state: &'a State,
    repo: Repository,
}

type ChangeEntry = (String, String);

impl<'a> ChangesTracker<'a> {
    pub fn new(repo: Repository, state_manager: &'a State) -> Self {
        Self {
            repo,
            state: state_manager,
        }
    }
    pub fn get_changed_files(&'a self) -> anyhow::Result<impl Iterator<Item = ChangeEntry>> {
        let last_indexed_commit_id = self
            .state
            .get_state_file_value(StateValue::LastIndexedCommit);

        let last_indexed_commit = last_indexed_commit_id
            .map(|id| {
                let last_commit = self
                    .repo
                    .find_commit(Oid::from_str(&id).map_err(DaemonServerError::GetChanges)?)
                    .map_err(DaemonServerError::GetChanges)?;

                Ok(last_commit)
            })
            .map_or::<Result<Option<Commit>, DaemonServerError>, _>(Ok(None), |res| {
                res.map(Some)
            })?;
        let repo_head = self.repo.head().map_err(DaemonServerError::GetChanges)?;
        let last_commit = repo_head
            .peel_to_commit()
            .map_err(DaemonServerError::GetChanges)?;

        let old_tree = last_indexed_commit
            .map(|commit| commit.tree().map_err(DaemonServerError::GetChanges))
            .map_or(Ok(None), |res| res.map(Some))?;
        let new_tree = last_commit.tree().map_err(DaemonServerError::GetChanges)?;

        let diff = self
            .repo
            .diff_tree_to_tree(old_tree.as_ref(), Some(&new_tree), None)
            .map_err(DaemonServerError::GetChanges)?;

        let changed_files = diff
            .deltas()
            .map(|delta| {
                let new_file = delta.new_file();
                let old_file = delta.old_file();

                let path = new_file
                    .path()
                    .or_else(|| old_file.path())
                    .ok_or(DaemonServerError::ParseCommand)? // TODO: use a proper error
                    .to_str()
                    .ok_or(DaemonServerError::ParseCommand)? // TODO: use a proper error
                    .to_string();
                let file_id = new_file.id();
                let content = self
                    .repo
                    .find_blob(file_id)
                    .map_err(DaemonServerError::GetChanges)?;
                let str_content = String::from_utf8(content.content().to_vec()).unwrap();

                Ok((path, str_content))
            })
            .collect::<Result<Vec<_>, DaemonServerError>>()?;
        Ok(changed_files.into_iter())
    }
}

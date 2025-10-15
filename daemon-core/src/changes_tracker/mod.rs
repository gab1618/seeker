use crate::state::{State, StateValue};
use git2::{self, Oid, Repository};

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
        let mut changed_files: Vec<ChangeEntry> = vec![];

        let last_indexed_commit_id = self
            .state
            .get_state_file_value(StateValue::LastIndexedCommit);

        let last_indexed_commit = last_indexed_commit_id
            .map(|id| self.repo.find_commit(Oid::from_str(&id).unwrap()).unwrap());
        let repo_head = self.repo.head().unwrap();
        let last_commit = repo_head.peel_to_commit().unwrap();

        let old_tree = last_indexed_commit.map(|commit| commit.tree().unwrap());
        let new_tree = last_commit.tree().unwrap();

        let diff = self
            .repo
            .diff_tree_to_tree(old_tree.as_ref(), Some(&new_tree), None)
            .unwrap();

        diff.foreach(
            &mut |delta, _| {
                let new_file = delta.new_file();
                let old_file = delta.old_file();

                let path = new_file
                    .path()
                    .or_else(|| old_file.path())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let file_id = new_file.id();
                let content = self.repo.find_blob(file_id).unwrap();
                let str_content = String::from_utf8(content.content().to_vec()).unwrap();
                changed_files.push((path, str_content));

                true
            },
            None,
            None,
            None,
        )
        .unwrap();
        Ok(changed_files.into_iter())
    }
}

use git2::{PushOptions, RemoteCallbacks, Repository, Signature};
use std::{fs::OpenOptions, io::Write, path::Path};
use tempfile::{tempdir, TempDir};

#[derive(Debug)]
pub enum TestRepoErr {
    Push,
    RemoteAdd,
    Add,
    InitBare,
    InitClone,
    WriteChanges,
    CommitChanges,
    GetCurrBranch,
}
type TestRepoResult<T> = Result<T, TestRepoErr>;

pub struct TestRepo {
    clone_dir: TempDir,
    bare_dir: TempDir,
}
impl TestRepo {
    pub fn new() -> TestRepoResult<Self> {
        let clone_dir = tempdir().map_err(|_| TestRepoErr::InitClone)?;
        let bare_dir = tempdir().map_err(|_| TestRepoErr::InitBare)?;

        let clone_repo = Repository::init(clone_dir.path()).unwrap();
        Repository::init_bare(bare_dir.path()).unwrap();

        clone_repo
            .remote("origin", bare_dir.path().to_str().unwrap())
            .unwrap();

        Ok(Self {
            clone_dir,
            bare_dir,
        })
    }
    pub fn commit_and_push_change(
        &self,
        filename: &str,
        content: &str,
        msg: &str,
    ) -> TestRepoResult<()> {
        let clone_repo = Repository::open(self.clone_dir.path()).unwrap();

        let filepath = self.clone_dir().path().to_path_buf().join(filename);
        let mut sample_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filepath)
            .unwrap();
        write!(sample_file, "{content}").map_err(|_| TestRepoErr::WriteChanges)?;
        drop(sample_file);

        let mut index = clone_repo.index().unwrap();
        index.add_path(Path::new(filename)).unwrap();
        index.write().unwrap();

        let signature = Signature::now("seeker-test", "test.seeker@seeker.com").unwrap();

        let tree_oid = index.write_tree().unwrap();
        let tree = clone_repo.find_tree(tree_oid).unwrap();

        let previous_commit = clone_repo.head().and_then(|r| r.peel_to_commit()).ok();

        let mut commit_parents = vec![];
        if let Some(prev) = previous_commit {
            commit_parents.push(prev)
        }

        clone_repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                msg,
                &tree,
                &commit_parents.iter().collect::<Vec<_>>()[..],
            )
            .unwrap();

        let clone_head = clone_repo.head().unwrap();
        let current_branch_name = clone_head.shorthand().unwrap();

        let mut remote = clone_repo.find_remote("origin").unwrap();
        let remote_callbacks = RemoteCallbacks::new();
        let mut push_options = PushOptions::new();

        push_options.remote_callbacks(remote_callbacks);

        let refspec = format!(
            "refs/heads/{branch}:refs/heads/{branch}",
            branch = current_branch_name
        );

        remote.push(&[&refspec], Some(&mut push_options)).unwrap();

        Ok(())
    }
    pub fn bare_dir(&self) -> &TempDir {
        &self.bare_dir
    }
    pub fn clone_dir(&self) -> &TempDir {
        &self.clone_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    #[test]
    fn test_setup_repo() {
        let test_repo = TestRepo::new().unwrap();
        test_repo
            .commit_and_push_change("doc.md", "testing", "first commit")
            .unwrap();
        let clone_repo_dir = test_repo.clone_dir();
        let bare_repo_dir = test_repo.bare_dir();

        let clone_repo = Repository::open(clone_repo_dir.path()).unwrap();
        let bare_repo = Repository::open_bare(bare_repo_dir.path()).unwrap();

        // Test if both directories are valid repositories and have a existing commit
        assert!(!clone_repo.is_empty().unwrap());
        clone_repo
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .message()
            .unwrap();

        assert!(!bare_repo.is_empty().unwrap());
        bare_repo
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .message()
            .unwrap();
    }
}

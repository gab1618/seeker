use std::{fs::OpenOptions, io::Write, path::Path};

use git2::{Commit, Oid, Repository, Signature};
use tempfile::{TempDir, tempdir};

pub struct TestRepo {
    bare_repo: Repository,
    bare_dir: TempDir,
    clone_repo: Repository,
    clone_dir: TempDir,
}

impl TestRepo {
    pub fn new() -> anyhow::Result<Self> {
        let bare_dir = tempdir()?;
        let bare_repo = Repository::init_bare(bare_dir.path())?;

        let clone_dir = tempdir()?;
        let clone_repo = Repository::clone(
            bare_dir.path().as_os_str().to_str().unwrap(),
            clone_dir.path(),
        )?;

        Ok(Self {
            bare_repo,
            bare_dir,
            clone_repo,
            clone_dir,
        })
    }
    pub fn bare_repo(&self) -> &Repository {
        &self.bare_repo
    }
    pub fn clone_repo(&self) -> &Repository {
        &self.clone_repo
    }
    pub fn commit_clone(&self, filename: &str, content: &str, msg: &str) -> anyhow::Result<Oid> {
        let file_path = self.clone_dir.path().to_path_buf().join(filename);
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)?;

        write!(f, "{content}")?;

        let mut index = self.clone_repo.index()?;
        index.add_path(Path::new(filename))?;
        let signature = Signature::now("seeker-test", "test@seeker.com")?;
        let new_tree_oid = index.write_tree()?;
        let new_tree = self.clone_repo.find_tree(new_tree_oid)?;

        let last_commit = self.clone_repo.head().and_then(|r| r.peel_to_commit()).ok();
        let mut commit_parents: Vec<Commit> = vec![];

        if let Some(last) = last_commit {
            commit_parents.push(last);
        }

        let new_commit = self.clone_repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            msg,
            &new_tree,
            &commit_parents.iter().collect::<Vec<_>>()[..],
        )?;

        Ok(new_commit)
    }
}

#[test]
fn test_new_test_repo() {
    let _ = TestRepo::new().unwrap();
}

#[test]
fn test_commit_clone() {
    let repo = TestRepo::new().unwrap();
    repo.commit_clone("testing.txt", "Hello, world", "First commit").unwrap();
    repo.clone_repo().head().unwrap();

    repo.commit_clone("testing.txt", "Hello, world!!!", "Second commit").unwrap();
    repo.clone_repo().head().unwrap();

}

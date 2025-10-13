use std::{fs::OpenOptions, io::Write, path::Path};

use git2::{Commit, Oid, PushOptions, Repository, Signature};
use tempfile::{TempDir, tempdir};

pub struct TestRepo {
    bare_dir: TempDir,
    clone_dir: TempDir,
}

impl TestRepo {
    pub fn new() -> anyhow::Result<Self> {
        let bare_dir = tempdir()?;
        let clone_dir = tempdir()?;

        Repository::init_bare(bare_dir.path())?;
        Repository::clone(bare_dir.path().to_str().unwrap(), clone_dir.path())?;

        Ok(Self {
            bare_dir,
            clone_dir,
        })
    }
    pub fn bare_repo(&self) -> anyhow::Result<Repository> {
        let bare = Repository::open_bare(self.bare_dir.path())?;
        Ok(bare)
    }
    pub fn clone_repo(&self) -> anyhow::Result<Repository> {
        let clone = Repository::open(self.clone_dir.path())?;
        Ok(clone)
    }
    pub fn commit_clone(&self, filename: &str, content: &str, msg: &str) -> anyhow::Result<Oid> {
        let clone_repo = Repository::open(self.clone_dir.path())?;
        let file_path = self.clone_dir.path().to_path_buf().join(filename);
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)?;

        write!(f, "{content}")?;

        let mut index = clone_repo.index()?;
        index.add_path(Path::new(filename))?;
        let signature = Signature::now("seeker-test", "test@seeker.com")?;
        let new_tree_oid = index.write_tree()?;
        let new_tree = clone_repo.find_tree(new_tree_oid)?;

        let last_commit = clone_repo.head().and_then(|r| r.peel_to_commit()).ok();
        let mut commit_parents: Vec<Commit> = vec![];

        if let Some(last) = last_commit {
            commit_parents.push(last);
        }

        let new_commit = clone_repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            msg,
            &new_tree,
            &commit_parents.iter().collect::<Vec<_>>()[..],
        )?;

        Ok(new_commit)
    }
    pub fn push_remote(&self) -> anyhow::Result<()> {
        let clone_repo = self.clone_repo().unwrap();
        let mut remote = clone_repo.find_remote("origin")?;
        let mut push_options = PushOptions::new();

        let current_head = clone_repo.head()?;
        let current_branch_name = current_head.shorthand().unwrap();
        let refspec = format!(
            "refs/heads/{branch}:refs/heads/{branch}",
            branch = current_branch_name
        );
        remote.push(&[&refspec], Some(&mut push_options))?;

        Ok(())
    }
}

#[test]
fn test_new_test_repo() {
    let _ = TestRepo::new().unwrap();
}

#[test]
fn test_commit_clone() {
    let repo = TestRepo::new().unwrap();
    repo.commit_clone("testing.txt", "Hello, world", "First commit")
        .unwrap();
    repo.clone_repo().unwrap().head().unwrap();

    repo.commit_clone("testing.txt", "Hello, world!!!", "Second commit")
        .unwrap();
    repo.clone_repo().unwrap().head().unwrap();
}

#[test]
fn test_push() {
    let repo = TestRepo::new().unwrap();
    repo.commit_clone("testing.txt", "Hello, world", "First commit")
        .unwrap();

    repo.clone_repo().unwrap().head().unwrap();

    assert!(repo.bare_repo().unwrap().head().is_err());
    repo.push_remote().unwrap();
    assert!(repo.bare_repo().unwrap().head().is_ok());
}

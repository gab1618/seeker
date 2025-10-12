use git2::Repository;
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
}

#[test]
fn test_new_test_repo() {
    let _ = TestRepo::new().unwrap();
}

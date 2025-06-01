use git2::Repository;
use std::{fs::OpenOptions, io::Write, process::Command};
use tempfile::{tempdir, TempDir};

#[derive(Debug)]
pub enum SetupRepoErr {
    Push,
    RemoteAdd,
    Add,
    InitBare,
    InitClone,
    WriteChanges,
    CommitChanges,
    GetCurrBranch,
}
type SetupRepoResult<T> = Result<T, SetupRepoErr>;

pub fn setup_repo() -> SetupRepoResult<(TempDir, TempDir)> {
    let bare_repo_dir = tempdir().map_err(|_| SetupRepoErr::InitBare)?;
    let clone_repo_dir = tempdir().map_err(|_| SetupRepoErr::InitClone)?;

    if !Command::new("git")
        .current_dir(bare_repo_dir.path())
        .args(["init", "--bare"])
        .output()
        .map_err(|_| SetupRepoErr::InitBare)?
        .status
        .success()
    {
        return Err(SetupRepoErr::InitBare);
    }

    if !Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["init"])
        .output()
        .map_err(|_| SetupRepoErr::InitClone)?
        .status
        .success()
    {
        return Err(SetupRepoErr::InitClone);
    }
    if !Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args([
            "remote",
            "add",
            "origin",
            bare_repo_dir.path().to_str().unwrap_or(""),
        ])
        .output()
        .map_err(|_| SetupRepoErr::RemoteAdd)?
        .status
        .success()
    {
        return Err(SetupRepoErr::RemoteAdd);
    }

    let mut sample_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(clone_repo_dir.path().to_path_buf().join("doc.md"))
        .unwrap();
    write!(sample_file, "testing").map_err(|_| SetupRepoErr::WriteChanges)?;
    drop(sample_file);

    if !Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["add", "doc.md"])
        .output()
        .map_err(|_| SetupRepoErr::Add)?
        .status
        .success()
    {
        return Err(SetupRepoErr::Add);
    }

    if !Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["commit", "-m", "testing"])
        .output()
        .map_err(|_| SetupRepoErr::CommitChanges)?
        .status
        .success()
    {
        return Err(SetupRepoErr::CommitChanges);
    }

    let current_clone_branch_name = {
        let clone_repo =
            Repository::open(clone_repo_dir.path()).map_err(|_| SetupRepoErr::GetCurrBranch)?;
        let head = clone_repo.head().map_err(|_| SetupRepoErr::GetCurrBranch)?;
        let branch_name = head.shorthand().ok_or(SetupRepoErr::GetCurrBranch)?;
        branch_name.to_owned()
    };

    if !Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["push", "-u", "origin", &current_clone_branch_name])
        .output()
        .map_err(|_| SetupRepoErr::Push)?
        .status
        .success()
    {
        return Err(SetupRepoErr::Push);
    }

    Ok((clone_repo_dir, bare_repo_dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    #[test]
    fn test_setup_repo() {
        let (clone_repo_dir, bare_repo_dir) = setup_repo().unwrap();
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

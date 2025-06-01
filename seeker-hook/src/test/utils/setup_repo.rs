use git2::Repository;
use std::{fs::OpenOptions, io::Write, process::Command};
use tempfile::{tempdir, TempDir};

pub fn setup_repo() -> std::io::Result<(TempDir, TempDir)> {
    let bare_repo_dir = tempdir()?;
    let clone_repo_dir = tempdir()?;

    let _ = Command::new("git")
        .current_dir(bare_repo_dir.path())
        .args(["init", "--bare"])
        .output()?;

    let _ = Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["init"])
        .output()?;
    let _ = Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args([
            "remote",
            "add",
            "origin",
            bare_repo_dir.path().to_str().unwrap_or(""),
        ])
        .output()?;

    let mut sample_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(clone_repo_dir.path().to_path_buf().join("doc.md"))
        .unwrap();
    let _ = write!(sample_file, "testing")?;
    drop(sample_file);

    let _ = Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["add", "doc.md"])
        .output()?;

    let _ = Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["commit", "-m", "testing"])
        .output()?;

    let current_clone_branch_name = {
        let clone_repo = Repository::open(clone_repo_dir.path()).unwrap();
        let head = clone_repo.head().unwrap();
        let branch_name = head.shorthand().unwrap();
        branch_name.to_owned()
    };

    let _ = Command::new("git")
        .current_dir(clone_repo_dir.path())
        .args(["push", "-u", "origin", &current_clone_branch_name])
        .output()?;

    // TODO: add some proper error handling in case any of these commands fails

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

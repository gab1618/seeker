use std::path::Path;

use git2::Repository;

use crate::args::GitArgs;

#[derive(Debug)]
pub enum SetupRepoErr {
    OpenRepo,
    UpdateHead,
}

/// If repo's head does point to an inexistent ref, so it points to the newly pushed one
pub fn setup_repo<P: AsRef<Path>>(mount_path: P, args: GitArgs) -> Result<(), SetupRepoErr> {
    let repo = Repository::open_bare(mount_path).map_err(|_| SetupRepoErr::OpenRepo)?;

    if repo.head().is_ok() {
        return Ok(());
    }

    repo.set_head(&args.ref_name)
        .map_err(|_| SetupRepoErr::UpdateHead)?;

    Ok(())
}

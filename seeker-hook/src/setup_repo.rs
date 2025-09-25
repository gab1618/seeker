use std::path::Path;

use git2::Repository;

use crate::{
    args::GitArgs,
    error::{SeekerHookErr, SeekerHookResult},
};

/// If repo's head does point to an inexistent ref, so point it to the newly pushed one
pub fn setup_repo<P: AsRef<Path>>(mount_path: P, args: &GitArgs) -> SeekerHookResult<()> {
    let repo = Repository::open_bare(mount_path).map_err(|_| SeekerHookErr::OpenRepo)?;

    if repo.head().is_ok() {
        return Ok(());
    }

    repo.set_head(&args.ref_name)
        .map_err(|_| SeekerHookErr::UpdateRepoHead)?;

    Ok(())
}

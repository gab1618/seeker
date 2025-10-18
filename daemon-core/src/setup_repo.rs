use std::path::Path;

use git2::Repository;

use crate::error::DaemonServerError;

/// If repo's head does point to an inexistent ref, so point it to the newly pushed one
pub fn setup_repo<P: AsRef<Path>>(mount_path: P) -> anyhow::Result<()> {
    let repo = Repository::open_bare(mount_path).map_err(DaemonServerError::OpenRepo)?;

    if repo.head().is_ok() {
        return Ok(());
    }

    let mut refs = repo.references().map_err(DaemonServerError::GetRepoRefs)?;
    let first_ref = refs
        .next()
        .ok_or(DaemonServerError::GetRepoLatestRef)?
        .map_err(DaemonServerError::GetRepoRefs)?;

    repo.set_head(
        first_ref
            .name()
            .ok_or(DaemonServerError::GetRepoLatestRef)?,
    )
    .map_err(DaemonServerError::UpdateRepoHead)?;

    Ok(())
}

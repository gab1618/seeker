use std::path::Path;

use git2::Repository;

/// If repo's head does point to an inexistent ref, so point it to the newly pushed one
pub fn setup_repo<P: AsRef<Path>>(mount_path: P) -> anyhow::Result<()>{
    let repo = Repository::open_bare(mount_path).unwrap();

    if repo.head().is_ok() {
        return Ok(());
    }

    let mut refs = repo.references().unwrap();
    let first_ref = refs.next().unwrap().unwrap();

    repo.set_head(first_ref.name().unwrap()).unwrap();

    Ok(())
}

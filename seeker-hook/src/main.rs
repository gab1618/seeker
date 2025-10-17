use std::path::PathBuf;

use crate::{daemon_client::get_daemon_client, error::SeekerHookResult};

mod daemon_client;

pub mod error;

#[tokio::main]
async fn main() -> SeekerHookResult<()> {
    let exe_path = std::env::current_exe().unwrap();
    let repo_path_parts = exe_path.iter().collect::<Vec<_>>();
    let repo_path = PathBuf::from_iter(repo_path_parts[..repo_path_parts.len() - 2].iter());

    let mut daemon_client = get_daemon_client("127.0.0.1:5151".into()).await?;

    let repo_path_str = repo_path
        .into_os_string()
        .to_str()
        .map(|inner| inner.to_owned())
        .unwrap();
    daemon_client.request_indexing(repo_path_str).await.unwrap();

    Ok(())
}

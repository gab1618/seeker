use std::path::PathBuf;

use crate::{daemon_client::get_daemon_client, error::SeekerHookResult};

mod daemon_client;

pub mod error;

#[tokio::main]
async fn main() -> SeekerHookResult<()> {
    let repo_path: PathBuf = std::env::args().nth(1).unwrap_or(".".to_string()).into();

    let mut daemon_client = get_daemon_client("127.0.0.1:5151".into()).await?;

    let repo_path_str = repo_path
        .into_os_string()
        .to_str()
        .map(|inner| inner.to_owned())
        .unwrap();
    daemon_client.request_indexing(repo_path_str).await.unwrap();

    Ok(())
}

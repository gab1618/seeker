use crate::{daemon_client::get_daemon_client, error::SeekerHookResult};

mod daemon_client;

pub mod error;

#[tokio::main]
async fn main() -> SeekerHookResult<()> {
    let mut repo_path = std::env::current_exe().unwrap();
    // TODO: do I have to explain?
    repo_path.pop();
    repo_path.pop();

    let mut daemon_client = get_daemon_client("127.0.0.1:5151".into()).await?;

    let repo_path_str = repo_path
        .into_os_string()
        .to_str()
        .map(|inner| inner.to_owned())
        .unwrap();
    daemon_client.request_indexing(repo_path_str).await.unwrap();

    Ok(())
}

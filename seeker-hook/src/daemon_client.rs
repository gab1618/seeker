use seeker_daemon_client::DaemonClient;
use tokio::net::TcpStream;

use crate::error::{SeekerHookErr, SeekerHookResult};

pub async fn get_daemon_client(bind_url: String) -> SeekerHookResult<DaemonClient> {
    let conn = TcpStream::connect(bind_url)
        .await
        .map_err(|_| SeekerHookErr::StartDaemonClient)?;
    let client = DaemonClient::new(conn);

    Ok(client)
}

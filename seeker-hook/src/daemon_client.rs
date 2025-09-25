use seeker_daemon_client::DaemonClient;
use tokio::net::TcpStream;

use crate::error::{SeekerHookErr, SeekerHookResult};

pub async fn get_daemon_client() -> SeekerHookResult<DaemonClient> {
    let conn = TcpStream::connect("127.0.0.1:5151")
        .await
        .map_err(|_| SeekerHookErr::StartDaemonClient)?;
    let client = DaemonClient::new(conn);

    Ok(client)
}

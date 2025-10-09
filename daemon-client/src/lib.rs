use seeker_daemon_core::{
    command::{DaemonAction, DaemonCommand},
    response::DaemonResponse,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

use crate::error::DaemonClientErr;

pub mod error;
#[cfg(test)]
mod test;

pub struct DaemonClient {
    conn: TcpStream,
}

impl DaemonClient {
    pub fn new(conn: TcpStream) -> Self {
        Self { conn }
    }
    pub async fn request_indexing(&mut self, repo_path: String) -> anyhow::Result<DaemonResponse> {
        let cmd = DaemonCommand::new(DaemonAction::Index, repo_path);

        let (r, w) = self.conn.split();

        let mut r = BufReader::new(r);
        let mut w = BufWriter::new(w);

        let str_cmd: String = cmd.into();
        let server_req = format!("{str_cmd}\n");
        w.write_all(server_req.as_bytes())
            .await
            .map_err(DaemonClientErr::SendIndexReq)?;
        w.flush().await.map_err(DaemonClientErr::SendIndexReq)?;

        let mut response_input = String::new();
        r.read_line(&mut response_input)
            .await
            .map_err(DaemonClientErr::RecvServerResponse)?;

        let parsed_response: DaemonResponse = response_input
            .as_str()
            .try_into()
            .map_err(DaemonClientErr::ParseServerResponse)?;

        Ok(parsed_response)
    }
}

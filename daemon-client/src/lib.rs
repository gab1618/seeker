use std::{
    io::{self},
    path::PathBuf,
};

use seeker_daemon_server_core::{command::{SeekerDaemonAction, SeekerDaemonCommand}, response::SeekerDaemonResponse};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

use crate::error::{DaemonClientErr, DaemonClientResult};

pub mod error;
#[cfg(test)]
mod test;

pub struct SeekerDaemonClient {
    conn: TcpStream,
}

impl SeekerDaemonClient {
    pub fn new(conn: TcpStream) -> Self {
        Self { conn }
    }
    pub async fn index_file(
        &mut self,
        file_path: PathBuf,
    ) -> DaemonClientResult<SeekerDaemonResponse> {
        let cmd = SeekerDaemonCommand::new(
            SeekerDaemonAction::Index,
            file_path,
        );

        let (r, w) = self.conn.split();

        let mut r = BufReader::new(r);
        let mut w = BufWriter::new(w);

        let str_cmd: String = cmd.into();
        let server_req = format!("{str_cmd}\n");
        w.write_all(server_req.as_bytes())
            .await
            .map_err(|_| DaemonClientErr::SendIndexReq)?;
        w.flush().await.map_err(|_| DaemonClientErr::SendIndexReq)?;

        let mut response_input = String::new();
        r.read_line(&mut response_input)
            .await
            .map_err(|_| DaemonClientErr::RecvServerResponse)?;

        let parsed_response: SeekerDaemonResponse = response_input
            .as_str()
            .try_into()
            .map_err(|_| DaemonClientErr::ParseServerResponse)?;

        Ok(parsed_response)
    }
}

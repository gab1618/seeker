use std::{
    io::{self},
    path::PathBuf,
};

use daemon_server_core::{command::SeekerDaemonCommand, response::SeekerDaemonResponse};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

#[cfg(test)]
mod test;

pub struct SeekerDaemonClient {
    conn: TcpStream,
}

impl SeekerDaemonClient {
    pub fn new(conn: TcpStream) -> io::Result<Self> {
        Ok(Self { conn })
    }
    pub async fn index_file(&mut self, file_path: PathBuf) -> io::Result<SeekerDaemonResponse> {
        let cmd = SeekerDaemonCommand::new(
            daemon_server_core::command::SeekerDaemonAction::Index,
            file_path,
        );

        let (r, w) = self.conn.split();

        let mut r = BufReader::new(r);
        let mut w = BufWriter::new(w);

        let str_cmd: String = cmd.into();
        let server_req = format!("{str_cmd}\n");
        w.write_all(server_req.as_bytes()).await?;
        w.flush().await?;

        let mut response_input = String::new();
        r.read_line(&mut response_input).await?;

        let parsed_response: SeekerDaemonResponse = response_input.as_str().try_into().unwrap();

        Ok(parsed_response)
    }
}

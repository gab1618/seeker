use std::{
    io::{self, BufWriter, Write},
    net::TcpStream,
    path::PathBuf,
};

use daemon_server_core::command::SeekerDaemonCommand;

pub struct SeekerDaemonClient {
    conn: TcpStream,
}

impl SeekerDaemonClient {
    pub fn new(conn: TcpStream) -> io::Result<Self> {
        Ok(Self { conn })
    }
    pub fn index_file(&self, file_path: PathBuf) -> io::Result<()> {
        let mut w = BufWriter::new(&self.conn);
        let cmd = SeekerDaemonCommand::new(
            daemon_server_core::command::SeekerDaemonAction::Index,
            file_path,
        );
        let str_cmd: String = cmd.into();
        writeln!(w, "{str_cmd}")?;
        Ok(())
    }
}

use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    command::{SeekerDaemonAction, SeekerDaemonCommand},
    error::{DaemonServerError, DaemonServerResult},
    indexer::Indexer,
};
pub struct SeekerDaemonServer<T: Indexer + Send + Sync + 'static> {
    listener: TcpListener,
    indexer: Arc<T>,
}

impl<T: Indexer + Send + Sync + 'static> SeekerDaemonServer<T> {
    pub fn bind(url: &str, indexer: Arc<T>) -> DaemonServerResult<Self> {
        let listener = TcpListener::bind(url).map_err(|_| DaemonServerError::StartServer)?;

        Ok(Self { listener, indexer })
    }
    pub fn start(self) -> JoinHandle<DaemonServerResult<Self>> {
        thread::spawn(move || {
            while let Ok((soc, _addrs)) = self.listener.accept() {
                let indexer = self.indexer.clone();
                thread::spawn(move || Self::handle_connection(soc, indexer));
            }
            Ok(self)
        })
    }
    fn handle_connection(soc: TcpStream, indexer: Arc<T>) -> DaemonServerResult<()> {
        let mut input = String::new();
        let mut r = BufReader::new(&soc);
        let mut w = BufWriter::new(&soc);
        r.read_line(&mut input)
            .map_err(|_| DaemonServerError::ReadRequest)?;

        let parsed_command: SeekerDaemonCommand = input.as_str().try_into()?;

        match parsed_command.action {
            SeekerDaemonAction::Index => {
                (*indexer).index_file(parsed_command.filepath)?;
            }
        }

        writeln!(w, "Command received: {input}").map_err(|_| DaemonServerError::SendResponse)?;
        Ok(())
    }
}

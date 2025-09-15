use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
    thread::{self, JoinHandle},
};

pub mod command;

pub struct SeekerDaemonServer {
    listener: TcpListener,
}

impl SeekerDaemonServer {
    pub fn bind(url: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(url)?;

        Ok(Self { listener })
    }
    pub fn start(self) -> JoinHandle<io::Result<Self>> {
        thread::spawn(move || {
            while let Ok((soc, _addrs)) = self.listener.accept() {
                let mut input = String::new();
                let mut r = BufReader::new(&soc);
                let mut w = BufWriter::new(&soc);
                r.read_line(&mut input)?;

                writeln!(w, "Command received: {input}")?;
            }
            Ok(self)
        })
    }
}

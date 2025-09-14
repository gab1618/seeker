use std::io;

use crate::server::SeekerDaemonServer;

mod server;

const DAEMON_BIND_URL: &'static str = "localhost:6513";

fn main() -> io::Result<()> {
    let server = SeekerDaemonServer::bind(DAEMON_BIND_URL)?;

    println!("Starting server...");
    let server_thread_handle = server.start();
    println!("Server running");

    server_thread_handle.join().expect("Unreachable")?;

    Ok(())
}

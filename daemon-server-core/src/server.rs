use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

use crate::{
    command::{DaemonAction, DaemonCommand},
    error::{DaemonServerError, DaemonServerResult},
    indexer::Indexer,
    response::DaemonResponse,
};
pub struct DaemonServer<T: Indexer + Send + Sync + 'static> {
    listener: TcpListener,
    indexer: Arc<T>,
}

impl<T: Indexer + Send + Sync + 'static> DaemonServer<T> {
    pub fn new(listener: TcpListener, indexer: Arc<T>) -> DaemonServerResult<Self> {
        Ok(Self { listener, indexer })
    }
    pub async fn start(self) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(());
            while let Ok((soc, _addr)) = self.listener.accept().await {
                let indexer = self.indexer.clone();
                if let Err(err) = Self::handle_connection(soc, indexer).await {
                    eprintln!("{:#?}", err);
                }
            }

            self
        });
        let _ = rx.await;
    }
    async fn handle_connection(mut soc: TcpStream, indexer: Arc<T>) -> DaemonServerResult<()> {
        let mut input = String::new();

        let (mut r, mut w) = soc.split();
        let mut r = BufReader::new(&mut r);
        let mut w = BufWriter::new(&mut w);

        r.read_line(&mut input)
            .await
            .map_err(|_| DaemonServerError::ReadRequest)?;

        let parsed_command: DaemonCommand = input.as_str().try_into()?;

        match parsed_command.action {
            DaemonAction::Index => {
                indexer.index_file(parsed_command.filepath)?;
            }
        }

        let resp = DaemonResponse {
            message: "Command received".to_owned(),
            status: crate::response::DaemonResponseStatus::Ok,
        };

        let str_response: String = (&resp).into();
        w.write_all(str_response.as_bytes())
            .await
            .map_err(|_| DaemonServerError::SendResponse)?;
        w.flush()
            .await
            .map_err(|_| DaemonServerError::SendResponse)?;

        Ok(())
    }
}

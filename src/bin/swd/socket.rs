use std::{
    path::{Path, PathBuf},
    io,
    fs::remove_file
};

use stopwatchd::models::stopwatch::_simulate_stopwatch;
use tokio::net::{UnixListener, UnixStream};

use crate::signal::SignalReceiver;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub csock_path: PathBuf,
    pub message: String
}

#[allow(dead_code)]
impl ClientInfo {
    pub fn from_client_raw<S: AsRef<str>>(raw: S) -> io::Result<Self> {
        let raw = raw.as_ref();
        let dslash_index = raw.find("//")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, raw))?;
        let csock_path = PathBuf::from(&raw[..dslash_index]);
        let message = &raw[(dslash_index+2)..];
        Ok(ClientInfo {csock_path, message: message.to_string()})
    }
}

pub fn clear_socket<P: AsRef<Path>>(path: &P) -> io::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        remove_file(path)?;
    }
    Ok(())
}

pub fn create_socket<P: AsRef<Path>>(path: &P) -> io::Result<UnixListener> {
    let path = path.as_ref();
    UnixListener::bind(path)
}

async fn handle_client(client: UnixStream) {
    trace!("handle_client");
    client.readable().await.unwrap();
    let mut braw = Vec::with_capacity(4096);
    match client.try_read_buf(&mut braw) {
        Ok(bytes_read) => {
            let raw = String::from_utf8(braw).unwrap();
            let message = format!("client sent {} bytes: {}", bytes_read, raw);
            println!("{}", message);
            info!("{}", message);
            _simulate_stopwatch(std::time::Duration::new(1, 0));
            client.writable().await.unwrap();
            match client.try_write(b"thank you") {
                Ok(_) => trace!("message sent back to client"),
                Err(e) => error!("could not write to client: {}", e)
            };
        },
        Err(e) => {
            let message = format!("could not store client UnixStream in message: {}", e);
            error!("{}", message);
        }
    };
}

pub async fn listen_to_socket(listener: &UnixListener, mut signal_rx: SignalReceiver) {
    debug!("listening to socket");
    loop {
        let incoming = tokio::select!{
            _ = signal_rx.recv() => {
                debug!("exiting listen_to_socket");
                return;
            },
            incoming = listener.accept() => incoming
        };
        match incoming {
            Ok((client, _addr)) => {
                debug!("received incoming");
                tokio::spawn(handle_client(client));
            },
            Err(e) => error!("could not receive message from client: {}", e)
        }
    }
}
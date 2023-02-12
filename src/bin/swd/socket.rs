use std::{
    path::Path,
    io,
    fs::remove_file
};

use tokio::net::UnixListener;

use crate::{signal::SignalReceiver, handlers::handle_client};

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
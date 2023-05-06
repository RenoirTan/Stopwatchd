//! Create and listen to sockets used as communication between `swd` and
//! `swctl`.

use std::{
    path::Path,
    io,
    fs::{remove_file, self}
};

use tokio::net::UnixListener;

use crate::{signal::SignalReceiver, handlers::handle_client, manager::RequestSender};

/// Unix file permissions for sockets.
pub const SOCK_MODE: i32 = 0o550;

/// Remove previous sockets.
pub fn clear_socket<P: AsRef<Path>>(path: &P) -> io::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        remove_file(path)?;
    }
    Ok(())
}

/// Create a socket to listen for client messages.
pub fn create_socket<P: AsRef<Path>>(path: &P) -> io::Result<UnixListener> {
    let path = path.as_ref();
    UnixListener::bind(path)
}

/// Set socket permissions.
pub fn set_socket_perms<P: AsRef<Path>>(path: &P) -> io::Result<i32> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(path, perms)?;
    Ok(SOCK_MODE)
}

/// Listen to socket from [`create_socket`] for client messages and pass them
/// on to the manager.
/// 
/// # Arguments
/// listener - Socket listener from [`create_socket`].
/// 
/// signal_rx - Receives messages when `swd` is instructed to terminate/restart.
/// 
/// req_tx - Transmit messages to manager.
pub async fn listen_to_socket(
    listener: &UnixListener,
    mut signal_rx: SignalReceiver,
    req_tx: RequestSender
) {
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
                tokio::spawn(handle_client(client, req_tx.clone()));
            },
            Err(e) => error!("could not receive message from client: {}", e)
        }
    }
}
use std::{
    path::Path,
    io,
    fs::{remove_file, self}
};

use tokio::net::UnixListener;

use crate::{signal::SignalReceiver, handlers::handle_client, manager::RequestSender};

pub const SOCK_MODE: i32 = 0o550;

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

pub fn set_socket_perms<P: AsRef<Path>>(path: &P) -> io::Result<i32> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(path, perms)?;
    Ok(SOCK_MODE)
}

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
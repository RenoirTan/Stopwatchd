use std::{
    path::Path,
    os::unix::net::{UnixListener, UnixStream},
    io::{self, Read},
    fs::remove_file
};

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

fn handle_client(client: &mut UnixStream) {
    let mut message = String::new();
    match client.read_to_string(&mut message) {
        Ok(bytes_read) => {
            message = format!("client sent {} bytes: {}", bytes_read, message);
        },
        Err(e) => {
            message = format!("could not store client UnixStream in message: {}", e);
            error!("{}", message);
        }
    };
    println!("{}", message);
    info!("{}", message);
}

pub fn listen_to_socket(listener: &UnixListener) {
    debug!("listening to socket");
    for incoming in listener.incoming() {
        debug!("received incoming");
        match incoming {
            Ok(mut client) => handle_client(&mut client),
            Err(e) => error!("could not receive message from client: {}", e)
        }
    }
}
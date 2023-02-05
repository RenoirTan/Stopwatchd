use std::{
    path::{Path, PathBuf},
    os::unix::net::{UnixListener, UnixStream},
    io::{self, Read, Write},
    fs::remove_file, thread, time::Duration
};

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

fn handle_client(client: &mut UnixStream) {
    debug!("handle_client");
    let mut braw = vec![0; 256];
    client.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    match client.read(&mut braw) {
        Ok(bytes_read) => {
            let raw = String::from_utf8(braw).unwrap();
            let message = format!("client sent {} bytes: {}", bytes_read, raw);
            println!("{}", message);
            info!("{}", message);
            match client.write_all(b"thank you") {
                Ok(_) => info!("message sent back to client"),
                Err(e) => error!("could not write to client: {}", e)
            };
            match client.flush() {
                Ok(_) => info!("message flushed to client"),
                Err(e) => error!("could not flush to client: {}", e)
            }
        },
        Err(e) => {
            let message = format!("could not store client UnixStream in message: {}", e);
            error!("{}", message);
        }
    };
}

pub fn listen_to_socket(listener: &UnixListener) {
    debug!("listening to socket");
    for incoming in listener.incoming() {
        debug!("received incoming");
        match incoming {
            Ok(mut client) => {thread::spawn(move || handle_client(&mut client));},
            Err(e) => error!("could not receive message from client: {}", e)
        }
    }
}
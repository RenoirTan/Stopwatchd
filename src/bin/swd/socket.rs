use std::{
    path::{Path, PathBuf},
    os::unix::net::{UnixListener, UnixStream},
    io::{self, Read, Write},
    fs::remove_file
};

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub csock_path: PathBuf,
    pub message: String
}

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
    let mut raw = String::new();
    match client.read_to_string(&mut raw) {
        Ok(bytes_read) => {
            let message = format!("client sent {} bytes: {}", bytes_read, raw);
            println!("{}", message);
            info!("{}", message);
            let client_info = match ClientInfo::from_client_raw(&raw) {
                Ok(ci) => ci,
                Err(e) => {error!("{}", e); return;}
            };
            debug!("client_info: {:?}", client_info);
            let mut reply_socket = match UnixStream::connect(&client_info.csock_path) {
                Ok(rs) => rs,
                Err(e) => {panic!("{}", e);}
            };
            debug!("reply_socket for {:?} created", client_info.csock_path);
            match reply_socket.write_all(b"thank you") {
                Ok(_) => info!("message sent back to client"),
                Err(e) => error!("{}", e)
            };
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
            Ok(mut client) => handle_client(&mut client),
            Err(e) => error!("could not receive message from client: {}", e)
        }
    }
}
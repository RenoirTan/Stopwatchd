use std::io;

use ciborium::de::from_reader;
use stopwatchd::communication::client_message::ClientMessage;
use tokio::net::UnixStream;

const BUF_MIN_CAPACITY: usize = 4096;


pub async fn handle_client(client: UnixStream) -> io::Result<()> {
    client.readable().await?;
    let mut braw = Vec::with_capacity(BUF_MIN_CAPACITY);
    let bytes_read = client.try_read_buf(&mut braw)?;
    debug!("received {} bytes from client", bytes_read);

    let cmsg: ClientMessage = from_reader(&braw[..]).unwrap();
    println!("{:?}", cmsg);

    trace!("waiting to send message to client");
    client.writable().await?;
    client.try_write(b"thank you")?;
    trace!("message sent to client");
    Ok(())
}
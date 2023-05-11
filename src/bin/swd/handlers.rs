//! Handle messages from clients.

use std::io;

use stopwatchd::{
    communication::{
        client::ClientMessage,
        server::{ServerMessage, Reply}
    },
    traits::Codecable
};
use tokio::net::UnixStream;

use crate::manager::{RequestSender, Request, make_response_channels};

// for docs
#[allow(unused)]
use crate::manager::Manager;

/// Minimum size of input buffer.
const BUF_MIN_CAPACITY: usize = 4096;

/// Handle messages from client.
/// 
/// # Arguments
/// client - Stream of bytes from client.
/// 
/// req_tx - Transmitter to [`Manager`].
pub async fn handle_client(client: UnixStream, req_tx: RequestSender) -> io::Result<()> {
    client.readable().await?;
    let mut braw = Vec::with_capacity(BUF_MIN_CAPACITY);
    let bytes_read = client.try_read_buf(&mut braw)?;
    debug!("received {} bytes from client", bytes_read);

    let message = ClientMessage::from_bytes(&braw)?;
    // println!("{:?}", message);

    // Communication from manager (res_tx) to handle_client (res_rx).
    let (res_tx, mut res_rx) = make_response_channels();
    let request = Request { action: message.request, res_tx };
    trace!("sending request to manager");
    req_tx.send(request).map_err(|e|
        io::Error::new(io::ErrorKind::ConnectionRefused, e)
    )?;

    trace!("waiting for response from manager");
    let reply = match res_rx.recv().await {
        Some(response) => {
            debug!("response received");
            ServerMessage::create(response.output)
        },
        None => {
            error!("no error from manager");
            ServerMessage::create(Reply::default())
        }
    };

    client.writable().await?;
    client.try_write(&reply.to_bytes()?)?;
    debug!("sent reply back to client");

    Ok(())
}
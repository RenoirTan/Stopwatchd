use std::io;

use stopwatchd::{
    communication::{
        start::StartSuccess,
        client_message::ClientMessage,
        server_message::{ServerMessage, ServerReply}
    },
    traits::Codecable,
    models::stopwatch::Stopwatch
};
use tokio::net::UnixStream;

use crate::manager::{RequestSender, Request, make_response_channels};

const BUF_MIN_CAPACITY: usize = 4096;


pub async fn handle_client(client: UnixStream, req_tx: RequestSender) -> io::Result<()> {
    client.readable().await?;
    let mut braw = Vec::with_capacity(BUF_MIN_CAPACITY);
    let bytes_read = client.try_read_buf(&mut braw)?;
    debug!("received {} bytes from client", bytes_read);

    let message = ClientMessage::from_bytes(&braw)?;
    println!("{:?}", message);

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
            ServerMessage::create(ServerReply::Default)
        }
    };

    client.writable().await?;
    client.try_write(&reply.to_bytes()?)?;
    debug!("sent reply back to client");

    Ok(())
}

#[allow(dead_code)]
async fn start_stopwatch(client: &UnixStream) -> io::Result<()> {
    debug!("creating stopwatch");
    let mut stopwatch = Stopwatch::start(None);

    let reply: ServerMessage = StartSuccess::from(&stopwatch).into();
    let message = reply.to_bytes()?;

    trace!("waiting to send message to client");
    client.writable().await?;
    client.try_write(&message)?;
    trace!("message sent to client");

    stopwatch.end();
    trace!("stopwatch stopped");
    println!("{:?}", stopwatch);

    Ok(())
}
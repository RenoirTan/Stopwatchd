use std::io;

use stopwatchd::{
    communication::start::{ClientStartStopwatch, ServerStartStopwatch},
    traits::Codecable, models::stopwatch::Stopwatch
};
use tokio::net::UnixStream;

const BUF_MIN_CAPACITY: usize = 4096;


pub async fn handle_client(client: UnixStream) -> io::Result<()> {
    client.readable().await?;
    let mut braw = Vec::with_capacity(BUF_MIN_CAPACITY);
    let bytes_read = client.try_read_buf(&mut braw)?;
    debug!("received {} bytes from client", bytes_read);

    let request = ClientStartStopwatch::from_bytes(&braw)?;
    println!("{:?}", request);

    debug!("creating stopwatch");
    let mut stopwatch = Stopwatch::start(None);

    let reply = ServerStartStopwatch::from(&stopwatch);
    let message = reply.to_bytes()?;

    trace!("waiting to send message to client");
    client.writable().await?;
    client.try_write(&message)?;
    trace!("message sent to client");

    stopwatch.end();
    println!("{:?}", stopwatch);

    Ok(())
}
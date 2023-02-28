use std::process;

use clap::Parser;
#[macro_use]
extern crate log;
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    communication::{
        client_message::ClientMessage,
        stop::StopRequest,
        server_message::{ServerMessage, ServerReply}
    },
    traits::Codecable
};
use tokio::net::UnixStream;

mod cli;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let identifiers = cli.identifiers;
    let verbose = cli.verbose;

    let pid = process::id();
    logging::setup(&format!("sw-stop.{}", pid), None).unwrap();
    info!("logging started");

    let swd_pid = {
        let mut pidfile = open_pidfile(false).unwrap();
        get_swd_pid(&mut pidfile).unwrap()
    };
    debug!("swd_pid is {}", swd_pid);

    let ssock_path = server_socket_path(Some(swd_pid));
    if ssock_path.exists() {
        debug!("{:?} exists", ssock_path);
    } else {
        debug!("{:?} does not exist", ssock_path);
    }
    trace!("connecting to {:?}", ssock_path);
    let stream = UnixStream::connect(&ssock_path).await.unwrap();
    trace!("connected to {:?}", ssock_path);

    trace!("checking if can write to server");
    stream.writable().await.unwrap();

    let request: ClientMessage = StopRequest {
        identifiers,
        verbose
    }.into();

    debug!("encoding message using ciborium");
    let message = request.to_bytes().unwrap();

    info!("writing message to server");
    stream.try_write(&message).unwrap();

    trace!("checking if can read from server");
    stream.readable().await.unwrap();
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw).unwrap();

    let reply = ServerMessage::from_bytes(&braw).unwrap();
    match reply.reply {
        ServerReply::Stop(i) => println!("{:?}", i),
        _ => panic!("swd should have replied with ServerReply::Stop")
    }

    info!("exiting");
}
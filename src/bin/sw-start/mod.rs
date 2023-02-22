use std::process;

#[macro_use]
extern crate log;
use clap::Parser;
use stopwatchd::{
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    logging,
    communication::{
        start::ClientStartStopwatch,
        client_message::ClientMessage, server_message::{ServerMessage, ServerReply}
    },
    traits::Codecable, models::stopwatch::Name
};
use tokio::net::UnixStream;

mod cli;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let sw_name = cli.name;
    let verbose = cli.verbose;
    if let Some(ref n) = sw_name {
        println!("requesting stopwatch to be named: {}", n);
    }

    let pid = process::id();
    logging::setup(&format!("sw-start.{}", pid), None).unwrap();
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
    // stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    // stream.set_write_timeout(Some(Duration::new(1, 0))).unwrap();
    trace!("connected to {:?}", ssock_path);

    trace!("checking if can write to server");
    stream.writable().await.unwrap();

    // generate message
    let request: ClientMessage = ClientStartStopwatch {
        name: Name::from(sw_name),
        verbose
    }.into();

    debug!("encoding message using ciborium");
    let message = request.to_bytes().unwrap();

    info!("writing message to server");
    stream.try_write(&message).unwrap();

    info!("waiting for response from server");

    trace!("checking if can read from server");
    stream.readable().await.unwrap();
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw).unwrap();

    let reply = ServerMessage::from_bytes(&braw).unwrap();
    match reply.reply {
        ServerReply::Start(s) => println!("{:?}", s),
        _ => panic!("swd should have replied with ServerReply::Start")
    }

    info!("exiting");
}
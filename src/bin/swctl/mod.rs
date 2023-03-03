use std::process;

#[macro_use]
extern crate log;
use clap::Parser;
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    communication::{
        client_message::ClientMessage,
        server_message::{ServerMessage, ServerReply},
        start::StartReply,
        info::InfoReply,
        stop::StopReply,
        lap::LapReply,
        pause::PauseReply, play::PlayReply
    },
    traits::Codecable
};
use tokio::net::UnixStream;

mod cli;
mod request;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);

    let pid = process::id();
    logging::setup(&format!("swctl.{}", pid), None).unwrap();
    debug!("swctl has started outputting logs");

    let swd_pid = {
        let mut pidfile = open_pidfile(false).unwrap();
        get_swd_pid(&mut pidfile).unwrap()
    };
    debug!("swd_pid is {}", swd_pid);

    let ssock_path = server_socket_path(Some(swd_pid));
    if ssock_path.exists() {
        debug!("{:?} exists", ssock_path);
    } else {
        panic!("{:?} does not exist", ssock_path);
    }
    trace!("connecting to {:?}", ssock_path);
    let stream = UnixStream::connect(&ssock_path).await.unwrap();
    trace!("connected to {:?}", ssock_path);

    trace!("checking if can write to server");
    stream.writable().await.unwrap();

    let message: ClientMessage = request::args_to_request(cli).into();
    let message_bytes = message.to_bytes().unwrap();

    info!("writing message to server");
    stream.try_write(&message_bytes).unwrap();

    trace!("checking if can read from server");
    stream.readable().await.unwrap();
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw).unwrap();

    let reply = ServerMessage::from_bytes(&braw).unwrap();
    println!("{:?}", reply);

    match reply.reply {
        ServerReply::Start(s) => handle_start(s).await,
        ServerReply::Info(i) => handle_info(i).await,
        ServerReply::Stop(s) => handle_stop(s).await,
        ServerReply::Lap(l) => handle_lap(l).await,
        ServerReply::Pause(p) => handle_pause(p).await,
        ServerReply::Play(p) => handle_play(p).await,
        ServerReply::Default => panic!("should not be ServerReply::Default")
    }

    info!("exiting");
}

async fn handle_start(start_reply: StartReply) {
    if start_reply.start.is_ok() {
        println!("successfully started");
    } else {
        println!("error occurred");
    }
}

async fn handle_info(info_reply: InfoReply) {
    if info_reply.errored.len() > 0 {
        println!("errors occurred");
    } else {
        println!("successful query");
    }
}

async fn handle_stop(stop_reply: StopReply) {
    if stop_reply.errored.len() > 0 {
        println!("errors occurred");
    } else {
        println!("successful query");
    }
}

async fn handle_lap(lap_reply: LapReply) {
    if lap_reply.errored.len() > 0 {
        println!("errors occurred");
    } else {
        println!("successful query");
    }
}

async fn handle_pause(pause_reply: PauseReply) {
    if pause_reply.errored.len() > 0 {
        println!("errors occurred");
    } else {
        println!("successful query");
    }
}

async fn handle_play(play_reply: PlayReply) {
    if play_reply.errored.len() > 0 {
        println!("errors occurred");
    } else {
        println!("successful query");
    }
}
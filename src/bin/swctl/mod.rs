use std::process;

#[macro_use]
extern crate log;
use clap::Parser;
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    communication::{
        client_message::{ClientMessage, ClientRequest},
        server_message::{ServerMessage, ServerReplyKind, ServerReply},
        info::InfoReply
    },
    traits::Codecable
};
use tabled::Table;
use tokio::net::UnixStream;

use crate::formatted::{BasicStopwatchDetails, BasicStopwatchDetailsBuilder};

mod cli;
mod formatted;
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

    let message: ClientMessage = request::args_to_request(&cli).into();
    let message_bytes = message.to_bytes().unwrap();

    info!("writing message to server");
    stream.try_write(&message_bytes).unwrap();

    trace!("checking if can read from server");
    stream.readable().await.unwrap();
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw).unwrap();

    let reply = ServerMessage::from_bytes(&braw).unwrap();

    match reply.reply.specific_answer {
        ServerReplyKind::Default => panic!("should not be ServerReply::Default"),
        ServerReplyKind::Info(InfoReply::All(_)) =>
            info_all_print(&cli, message.request, reply.reply).await,
        _ => generic_print(&cli, message.request, reply.reply).await
    }

    info!("exiting");
}

async fn generic_print(args: &cli::Cli, request: ClientRequest, mut reply: ServerReply) {
    let details: Vec<BasicStopwatchDetails> = {
        let mut d = vec![];
        let builder = BasicStopwatchDetailsBuilder::new(&args.datetime_fmt, &args.duration_fmt);
        for identifier in request.identifiers {
            let success = match reply.successful.remove(&identifier) {
                Some(s) => s,
                None => continue
            };
            d.push(builder.get_details(success, args.show_datetime_info));
        }
        d
    };
    let details_table = Table::new(details).to_string();
    println!("{}", details_table);
}

async fn info_all_print(args: &cli::Cli, _request: ClientRequest, mut reply: ServerReply) {
    let all = match reply.specific_answer {
        ServerReplyKind::Info(InfoReply::All(all)) => all,
        _ => panic!("match didn't work for InfoReply::All")
    };

    let details: Vec<BasicStopwatchDetails> = {
        let mut d = vec![];
        let builder = BasicStopwatchDetailsBuilder::new(&args.datetime_fmt, &args.duration_fmt);
        for identifier in all.access_order {
            let success = match reply.successful.remove(&identifier) {
                Some(s) => s,
                None => continue
            };
            d.push(builder.get_details(success, args.show_datetime_info));
        }
        d
    };
    let details_table = Table::new(details).to_string();
    println!("{}", details_table);
}
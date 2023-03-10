use std::process;

#[macro_use]
extern crate log;
use clap::Parser;
use formatted::{get_basic_single_builder, get_verbose_table_builder, get_basic_table_builder, get_error_table_builder, add_errors_to_builder};
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    communication::{
        client_message::{ClientMessage, ClientRequest},
        server_message::{ServerMessage, ServerReplyKind, ServerReply, ServerError},
        info::InfoReply, details::StopwatchDetails
    },
    traits::Codecable, identifiers::Identifier
};
use tokio::net::UnixStream;

use crate::formatted::Formatter;

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
    println!("{:?}", reply);

    match reply.reply.specific_answer {
        ServerReplyKind::Default => panic!("should not be ServerReply::Default"),
        ServerReplyKind::Info(InfoReply::All(_)) =>
            info_all_print(&cli, message.request, reply.reply).await,
        _ => generic_print(&cli, message.request, reply.reply).await
    }

    info!("exiting");
}

async fn generic_print(
    args: &cli::Cli,
    request: ClientRequest,
    mut reply: ServerReply
) {
    let mut details = Vec::with_capacity(reply.successful.len());
    let mut errors = Vec::with_capacity(reply.errors.len());
    
    for identifier in &request.identifiers {
        if let Some(d) = reply.successful.remove(&identifier) {
            details.push(d);
        }
        let o_id = Some(identifier.clone());
        if let Some(e) = reply.errors.remove(&o_id) {
            errors.push((o_id, e));
        }
    }

    let formatter = Formatter::new(&args.datetime_fmt, &args.duration_fmt);
    println!("{}", generate_output(args, details, &formatter));
    println!("{}", generate_errors(args, errors, &formatter));
}

async fn info_all_print(
    args: &cli::Cli,
    _request: ClientRequest,
    mut reply: ServerReply
) {
    let all = match reply.specific_answer {
        ServerReplyKind::Info(InfoReply::All(all)) => all,
        _ => panic!("match didn't work for InfoReply::All")
    };

    let mut details = Vec::with_capacity(reply.successful.len());
    let mut errors = Vec::with_capacity(reply.errors.len());
    
    for identifier in &all.access_order {
        if let Some(d) = reply.successful.remove(&identifier) {
            details.push(d);
        }
        let o_id = Some(identifier.clone());
        if let Some(e) = reply.errors.remove(&o_id) {
            errors.push((o_id, e));
        }
    }

    let formatter = Formatter::new(&args.datetime_fmt, &args.duration_fmt);
    println!("{}", generate_output(args, details, &formatter));
    println!("{}", generate_errors(args, errors, &formatter));
}

fn generate_output<I>(args: &cli::Cli, details: I, formatter: &Formatter) -> String
where
    I: IntoIterator<Item = StopwatchDetails>
{
    if args.verbose {
        let mut out = String::new();
        let basic = get_basic_single_builder(args.show_datetime_info);
        let verbose = get_verbose_table_builder(args.show_datetime_info);
        let builders = formatter
            .from_details_verbose(basic, verbose, details, args.show_datetime_info);
        for (b, v) in builders {
            if out.len() != 0 {
                out.push('\n');
            }
            out.push_str(&b.build().to_string());
            out.push('\n');
            out.push_str(&v.build().to_string());
        }
        out
    } else {
        let mut builder = get_basic_table_builder(args.show_datetime_info);
        let row_count = formatter.from_details(&mut builder, details, args.show_datetime_info);
        if row_count == 0 {
            String::new()
        } else {
            builder.build().to_string()
        }
    }
}

fn generate_errors<I>(args: &cli::Cli, iter: I, formatter: &Formatter) -> String
where
    I: IntoIterator<Item = (Option<Identifier>, Vec<ServerError>)>
{
    let mut out = String::new();
    for (identifier, errors) in iter {
        let mut builder = get_error_table_builder(args.show_datetime_info);
        let record = formatter.get_errors(identifier, errors, args.show_datetime_info);
        add_errors_to_builder(&mut builder, record, args.show_datetime_info);
        if out.len() != 0 {
            out.push('\n');
        }
        out.push_str(&builder.build().to_string());
    }
    out
}
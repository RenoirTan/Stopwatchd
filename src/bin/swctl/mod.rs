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

    let (details, errors) = match reply.reply.specific_answer {
        ServerReplyKind::Default => panic!("should not be ServerReply::Default"),
        ServerReplyKind::Info(InfoReply::All(ref all)) => {
            let ao = all.access_order.clone();
            get_details_errors(&message.request, reply.reply, Some(&ao))
        },
        _ => get_details_errors(&message.request, reply.reply, None)
    };

    let formatter = Formatter::new(&cli.datetime_fmt, &cli.duration_fmt);

    let good = generate_output(&cli, details, &formatter);
    let bad = generate_errors(&cli, errors, &formatter);

    if good.len() > 0 {
        println!("{}", good);
    }
    if bad.len() > 0 {
        println!("!! ERRORS:\n{}", bad);
    } else {
        println!("ALL OK");
    }

    info!("exiting");
}

fn get_details_errors(
    request: &ClientRequest,
    mut reply: ServerReply,
    access_order: Option<&Vec<Identifier>>
) -> (Vec<StopwatchDetails>, Vec<(Option<Identifier>, Vec<ServerError>)>) {
    let mut details = Vec::with_capacity(reply.successful.len());
    let mut errors = Vec::with_capacity(reply.errors.len());

    // If InfoAll, then an access order would have been provided, used that instead.
    // Otherwise, use the cmd args as the access order.
    let ao = match access_order {
        Some(ao) => ao.iter(),
        None => request.identifiers.iter()
    };

    /* 
        TODO: Make sure that successes/errors that didn't match any `identifier`
        eventually get printed out somewhere
    */
    for identifier in ao {
        if let Some(d) = reply.successful.remove(&identifier) {
            details.push(d);
        }
        let o_id = Some(identifier.clone());
        if let Some(e) = reply.errors.remove(&o_id) {
            errors.push((o_id, e));
        }
    }
    if let Some(e) = reply.errors.remove(&None) {
        errors.insert(0, (None, e));
    }

    (details, errors)
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
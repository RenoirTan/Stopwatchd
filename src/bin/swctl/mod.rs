//! User process that talks to `swd` to interact with and get details about
//! stopwatches.

use std::process::{self, exit};

#[macro_use]
extern crate log;
use clap::Parser;
use formatted::{get_basic_single_builder, get_verbose_table_builder, get_basic_table_builder, get_error_table_builder, add_errors_to_builder, Styles};
use stopwatchd::{
    logging,
    pidfile::{open_pidfile, get_swd_pid, pidfile_path},
    runtime::{server_socket_path, get_uid},
    communication::{
        client::{ClientMessage, Request},
        server::{ServerMessage, ReplyKind, Reply, ServerError},
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
    exit(run(cli).await);
}

/// Actual function that does stuff
async fn run(cli: cli::Cli) -> i32 {
    let pid = process::id();
    logging::setup(&format!("swctl.{}", pid), Some(cli.log_level.into()))
        .expect("could not setup logging");
    debug!("swctl has started outputting logs");

    #[cfg(not(feature = "users"))]
    let uid = get_uid();
    #[cfg(feature = "users")]
    let uid = if cli.system_swd { None } else { get_uid() };

    let swd_pid = {
        let ppath = pidfile_path(uid);
        let mut pidfile = open_pidfile(false, uid)
            .expect(&format!("could not open pidfile: {:?}", ppath));
        get_swd_pid(&mut pidfile)
            .expect(&format!("could not get swd PID from {:?}", ppath))
    };
    debug!("swd_pid is {}", swd_pid);

    let ssock_path = server_socket_path(Some(swd_pid), uid);
    let ssock_path_str = ssock_path.to_string_lossy();
    trace!("connecting to {:?}", ssock_path);
    let stream = UnixStream::connect(&ssock_path).await
        .expect(&format!("could not connect to {}", ssock_path_str));
    trace!("connected to {:?}", ssock_path);

    trace!("checking if can write to server");
    stream.writable().await.expect(&format!("{} is not writeable", ssock_path_str));

    let message: ClientMessage = request::args_to_request(&cli).into();
    let message_bytes = message.to_bytes()
        .expect("could not convert request into message");

    info!("writing message to server");
    stream.try_write(&message_bytes)
        .expect(&format!("could not write request message to {}", ssock_path_str));

    trace!("checking if can read from server");
    stream.readable().await
        .expect(&format!("{} is not readable", ssock_path_str));
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw)
        .expect(&format!("could not read reply message from {}", ssock_path_str));

    let reply = ServerMessage::from_bytes(&braw)
        .expect(&format!("could not convert message to reply"));

    let (details, errors) = match reply.reply.specific_answer {
        ReplyKind::Default => panic!("should not be ServerReply::Default"),
        ReplyKind::Info(InfoReply::All(ref all)) => {
            let ao = all.access_order.clone();
            get_details_errors(&message.request, reply.reply, Some(&ao))
        },
        _ => get_details_errors(&message.request, reply.reply, None)
    };

    let formatter = Formatter::new(&cli.datetime_fmt, &cli.duration_fmt);

    let good = generate_output(&cli, details, &formatter, cli.table_style);
    let bad = generate_errors(&cli, errors, &formatter, cli.table_style);

    if good.len() > 0 {
        println!("{}", good);
    } else {
        println!("Found nothing");
    }
    if bad.len() > 0 {
        println!("!! ERRORS:\n{}", bad);
        info!("exiting with errors");
        1
    } else {
        info!("exiting without errors");
        0
    }
}

/// Extract [`StopwatchDetails`] and [`ServerError`] from `reply`.
/// 
/// # Arguments
/// * request - Request sent to the server. This function uses the details from
/// `request` to decide the order in which stopwatches should be displayed.
fn get_details_errors(
    request: &Request,
    mut reply: Reply,
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

    // Errors associated with `None` identifier are likely more serious and
    // should therefore be bubbled to the top
    if let Some(e) = reply.errors.remove(&None) {
        errors.push((None, e));
    }
    // Add stuff that has an identifier in access order
    for identifier in ao {
        if let Some(d) = reply.successful.remove(&identifier) {
            details.push(d);
        }
        let o_id = Some(identifier.clone());
        if let Some(e) = reply.errors.remove(&o_id) {
            errors.push((o_id, e));
        }
    }
    // Then drain everything else into their respective vectors
    details.extend(reply.successful.drain().map(|(_k, v)| v));
    errors.extend(reply.errors.drain());

    (details, errors)
}

/// Format [`StopwatchDetails`] into a string.
fn generate_output<I>(args: &cli::Cli, details: I, formatter: &Formatter, style: Styles) -> String
where
    I: IntoIterator<Item = StopwatchDetails>
{
    if args.verbose {
        let mut out = String::new();
        let basic = get_basic_single_builder(args.show_datetime_info);
        let verbose = get_verbose_table_builder(args.show_datetime_info);
        for d in details {
            let mut b = basic.clone();
            let mut v = verbose.clone();

            formatter.from_verbose(&mut b, &mut v, d, args.show_datetime_info);

            if out.len() != 0 {
                out.push('\n');
            }

            let mut btable = b.build();
            style.style_table(&mut btable);
            out.push_str(&btable.to_string());
            out.push('\n');
            let mut vtable = v.build();
            style.style_table(&mut vtable);
            out.push_str(&vtable.to_string());
        }
        out
    } else {
        let mut builder = get_basic_table_builder(args.show_datetime_info);
        let row_count = formatter.from_details(&mut builder, details, args.show_datetime_info);
        if row_count == 0 {
            String::new()
        } else {
            let mut table = builder.build();
            style.style_table(&mut table);
            table.to_string()
        }
    }
}

/// Format [`ServerError`] into strings.
fn generate_errors<I>(args: &cli::Cli, iter: I, formatter: &Formatter, style: Styles) -> String
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
        let mut table = builder.build();
        style.style_table(&mut table);
        out.push_str(&table.to_string());
    }
    out
}
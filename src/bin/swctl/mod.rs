//! User process that talks to `swd` to interact with and get details about
//! stopwatches.

use std::process::{self, exit};

#[macro_use]
extern crate log;
use clap::Parser;
use formatted::{ErrorRecord, BasicDetails, BasicDetailsNoDT, VerboseDetails, VerboseDetailsNoDT};
use stopwatchd::{
    fmt::Formatter,
    logging,
    pidfile::{open_pidfile, get_swd_pid, pidfile_path},
    runtime::{server_socket_path, get_uid},
    communication::{
        client::Request,
        server::{Reply, ServerError},
        details::StopwatchDetails,
        reply_specifics::{SpecificAnswer, InfoAnswer}
    },
    traits::Codecable
};
use tabled::{builder::Builder, Tabled};
use tokio::net::UnixStream;

use crate::formatted::Styles;

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

    let request = request::args_to_request(&cli);
    let message_bytes = request.to_bytes()
        .expect("could not serialize request to bytes");

    #[cfg(feature = "debug-ipc")]
    if cli.debug_ipc {
        println!("From swctl.{} to swd.{}: {:?}", pid, swd_pid, message_bytes);
    }

    info!("sending request to server");
    stream.try_write(&message_bytes)
        .expect(&format!("could not write request message to {}", ssock_path_str));

    trace!("checking if can read from server");
    stream.readable().await
        .expect(&format!("{} is not readable", ssock_path_str));
    let mut braw = Vec::with_capacity(4096);
    info!("reading response from server");
    stream.try_read_buf(&mut braw)
        .expect(&format!("could not read reply message from {}", ssock_path_str));

    #[cfg(feature = "debug-ipc")]
    if cli.debug_ipc {
        println!("From swd.{} to swctl.{}: {:?}", swd_pid, pid, braw);
    }

    let reply = Reply::from_bytes(&braw)
        .expect(&format!("could not convert message to reply"));

    let (details, errors) = match reply.specific_answer {
        SpecificAnswer::Info(InfoAnswer::All(ref all)) => {
            let ao = all.access_order.clone();
            get_details_errors(&request, reply, Some(&ao))
        },
        _ => get_details_errors(&request, reply, None)
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
    access_order: Option<&Vec<String>>
) -> (Vec<StopwatchDetails>, Vec<(Option<String>, Vec<ServerError>)>) {
    let mut details = Vec::with_capacity(reply.successful.len());
    let mut errors = Vec::with_capacity(reply.errors.len());

    // If InfoAll, then an access order would have been provided, used that instead.
    // Otherwise, use the cmd args as the access order.
    let ao = match access_order {
        Some(ao) => ao.iter(),
        None => request.common_args.raw_identifiers.iter()
    };

    // Errors associated with `None` identifier are likely more serious and
    // should therefore be bubbled to the top
    if let Some(e) = reply.errors.remove(&None) {
        errors.push((None, e));
    }
    // Add stuff that has an identifier in access order
    for identifier in ao {
        if let Some(d) = reply.successful.remove(identifier) {
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
        generate_output_verbose(args, details, formatter, style)
    } else {
        generate_output_normal(args, details, formatter, style)
    }
}

fn generate_output_normal<I>(
    args: &cli::Cli,
    details: I,
    formatter: &Formatter,
    style: Styles
) -> String
where
    I: IntoIterator<Item = StopwatchDetails>
{
    let mut builder = Builder::new();
    if args.show_datetime_info {
        builder.set_header(BasicDetails::headers());
    } else {
        builder.set_header(BasicDetailsNoDT::headers());
    }
    for d in details {
        let record = BasicDetails::format(formatter, &d, args.show_datetime_info);
        if args.show_datetime_info {
            builder.push_record(record.fields());
        } else {
            builder.push_record(BasicDetailsNoDT::from(record).fields());
        }
    }
    if builder.count_rows() == 0 {
        "".to_string()
    } else {
        let mut table = builder.build();
        style.style_table(&mut table);
        table.to_string()
    }
}

fn generate_output_verbose<I>(
    args: &cli::Cli,
    details: I,
    formatter: &Formatter,
    style: Styles
) -> String
where
    I: IntoIterator<Item = StopwatchDetails>
{
    let mut n_stopwatches: usize = 0;
    let mut out = "+++\n".to_string();
    'l: for d in details {
        n_stopwatches += 1;
        let mut basic_builder = Builder::default();
        let basic_record = BasicDetails::format(formatter, &d, args.show_datetime_info);
        if args.show_datetime_info {
            basic_builder.set_header(BasicDetails::headers());
            basic_builder.push_record(basic_record.fields());
        } else {
            basic_builder.set_header(BasicDetailsNoDT::headers());
            basic_builder.push_record(BasicDetailsNoDT::from(basic_record).fields());
        }
        let mut table = basic_builder.index().column(0).transpose().build();
        style.style_table(&mut table);
        out.push_str(&table.to_string());

        let verbose = match d.verbose_info {
            Some(v) => v,
            None => continue 'l
        };
        out.push_str("\n---\n");
        let mut verbose_builder = Builder::default();
        if args.show_datetime_info {
            verbose_builder.set_header(VerboseDetails::headers());
        } else {
            verbose_builder.set_header(VerboseDetailsNoDT::headers());
        }
        for lap in &verbose.laps {
            let vd = VerboseDetails::format(formatter, lap, args.show_datetime_info);
            if args.show_datetime_info {
                verbose_builder.push_record(vd.fields());
            } else {
                verbose_builder.push_record(VerboseDetailsNoDT::from(vd).fields());
            }
        }
        let mut table = verbose_builder.build();
        style.style_table(&mut table);
        out.push_str(&table.to_string());

        out.push_str("\n+++");
    }
    if n_stopwatches == 0 {
        "".to_string()
    } else {
        out
    }
}

/// Format [`ServerError`] into strings.
fn generate_errors<I>(_args: &cli::Cli, iter: I, formatter: &Formatter, style: Styles) -> String
where
    I: IntoIterator<Item = (Option<String>, Vec<ServerError>)>
{
    let mut builder = Builder::default();
    builder.set_header(ErrorRecord::headers());
    for (identifier, errors) in iter {
        for error in errors {
            let record = ErrorRecord::format(formatter, identifier.as_ref(), error);
            builder.push_record(record.fields());
        }
    }
    if builder.count_rows() == 0 {
        return String::new();
    }
    let mut table = builder.build();
    style.style_table(&mut table);
    table.to_string()
}

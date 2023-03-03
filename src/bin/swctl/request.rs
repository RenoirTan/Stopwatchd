use stopwatchd::{
    communication::{
        client_message::ClientRequest,
        start::StartRequest,
        info::InfoRequest,
        stop::StopRequest,
        lap::LapRequest,
        pause::PauseRequest
    },
    models::stopwatch::Name, identifiers::Identifier
};

use crate::cli::{self, Subcommands};

pub fn args_to_request(args: cli::Cli) -> ClientRequest {
    match args.action {
        Subcommands::Start(start_args) => start_args_to_request(start_args),
        Subcommands::Info(info_args) => info_args_to_request(info_args),
        Subcommands::Stop(stop_args) => stop_args_to_request(stop_args),
        Subcommands::Lap(lap_args) => lap_args_to_request(lap_args),
        Subcommands::Pause(pause_args) => pause_args_to_request(pause_args)
    }
}

fn start_args_to_request(args: cli::Start) -> ClientRequest {
    StartRequest {
        name: Name::from(args.identifier),
        verbose: args.verbose
    }.into()
}

fn info_args_to_request(args: cli::Info) -> ClientRequest {
    InfoRequest {
        identifiers: args.identifiers.into_iter().map(Identifier::new).collect(),
        verbose: args.verbose
    }.into()
}

fn stop_args_to_request(args: cli::Stop) -> ClientRequest {
    StopRequest {
        identifiers: args.identifiers.into_iter().map(Identifier::new).collect(),
        verbose: args.verbose
    }.into()
}

fn lap_args_to_request(args: cli::Lap) -> ClientRequest {
    LapRequest {
        identifiers: args.identifiers.into_iter().map(Identifier::new).collect(),
        verbose: args.verbose
    }.into()
}

fn pause_args_to_request(args: cli::Pause) -> ClientRequest {
    PauseRequest {
        identifiers: args.identifiers.into_iter().map(Identifier::new).collect(),
        verbose: args.verbose
    }.into()
}
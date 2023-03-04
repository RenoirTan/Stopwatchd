use stopwatchd::communication::{
    client_message::ClientRequest,
    start::StartRequest,
    info::InfoRequest,
    stop::StopRequest,
    lap::LapRequest,
    pause::PauseRequest,
    play::PlayRequest,
    delete::DeleteRequest
};

use crate::cli::{self, Subcommands};

pub fn args_to_request(args: cli::Cli) -> ClientRequest {
    let (identifiers, verbose, specific) = match args.action {
        Subcommands::Start(args) => return start_args_to_request(args),
        Subcommands::Info(args) => (args.identifiers, args.verbose, InfoRequest.into()),
        Subcommands::Stop(args) => (args.identifiers, args.verbose, StopRequest.into()),
        Subcommands::Lap(args) => (args.identifiers, args.verbose, LapRequest.into()),
        Subcommands::Pause(args) => (args.identifiers, args.verbose, PauseRequest.into()),
        Subcommands::Play(args) => (args.identifiers, args.verbose, PlayRequest.into()),
        Subcommands::Delete(args) => (args.identifiers, args.verbose, DeleteRequest.into())
    };
    ClientRequest::new(identifiers, verbose, specific)
}

fn start_args_to_request(args: cli::Start) -> ClientRequest {
    let identifiers = match args.identifier {
        Some(i) => vec![i],
        None => vec![]
    };
    let verbose = args.verbose;
    ClientRequest::new(identifiers, verbose, StartRequest.into())
}
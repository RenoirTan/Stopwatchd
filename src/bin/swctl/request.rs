//! Convert command line arguments to a request to `swd`.

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

/// Convert arguments to a request. See [`ClientRequest`] on how to send
/// a serialised message to `swd`.
pub fn args_to_request(args: &cli::Cli) -> ClientRequest {
    let (identifiers, specific) = match &args.action {
        Subcommands::Start(args) => (
            args.identifier.iter().map(Clone::clone).collect(),
            StartRequest.into()
        ),
        Subcommands::Info(args) => (args.identifiers.clone(), InfoRequest.into()),
        Subcommands::Stop(args) => (args.identifiers.clone(), StopRequest.into()),
        Subcommands::Lap(args) => (args.identifiers.clone(), LapRequest.into()),
        Subcommands::Pause(args) => (args.identifiers.clone(), PauseRequest.into()),
        Subcommands::Play(args) => (args.identifiers.clone(), PlayRequest.into()),
        Subcommands::Delete(args) => (args.identifiers.clone(), DeleteRequest.into())
    };
    ClientRequest::new(identifiers, args.verbose, specific)
}
//! Convert command line arguments to a request to `swd`.

use stopwatchd::{
    communication::{
        client::{Request, CommonArgs},
        request_specifics::{StartArgs, InfoArgs, StopArgs, LapArgs, PauseArgs, PlayArgs, DeleteArgs}
    }
};

use crate::cli::{self, Subcommands};

/// Convert arguments to a request. See [`Request`] on how to send
/// a serialised message to `swd`.
pub fn args_to_request(args: &cli::Cli) -> Request {
    let (identifiers, specific) = match &args.action {
        Subcommands::Start(args) => (
            args.identifier.iter().map(Clone::clone).collect(),
            StartArgs.into()
        ),
        Subcommands::Info(args) => (args.identifiers.clone(), InfoArgs.into()),
        Subcommands::Stop(args) => (args.identifiers.clone(), StopArgs.into()),
        Subcommands::Lap(args) => (args.identifiers.clone(), LapArgs.into()),
        Subcommands::Pause(args) => (args.identifiers.clone(), PauseArgs.into()),
        Subcommands::Play(args) => (args.identifiers.clone(), PlayArgs.into()),
        Subcommands::Delete(args) => (args.identifiers.clone(), DeleteArgs.into())
    };
    let common = CommonArgs::from_iter(identifiers, args.verbose);
    Request::new(common, specific)
}
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
            args.raw_identifier.iter().map(Clone::clone).collect(),
            StartArgs { fix_bad_names: args.fix_bad_names }.into()
        ),
        Subcommands::Info(args) => (args.raw_identifiers.clone(), InfoArgs.into()),
        Subcommands::Stop(args) => (args.raw_identifiers.clone(), StopArgs.into()),
        Subcommands::Lap(args) => (args.raw_identifiers.clone(), LapArgs.into()),
        Subcommands::Pause(args) => (args.raw_identifiers.clone(), PauseArgs.into()),
        Subcommands::Play(args) => (args.raw_identifiers.clone(), PlayArgs.into()),
        Subcommands::Delete(args) => (args.raw_identifiers.clone(), DeleteArgs.into())
    };
    let common = CommonArgs::from_iter(identifiers, args.verbose);
    Request::new(common, specific)
}
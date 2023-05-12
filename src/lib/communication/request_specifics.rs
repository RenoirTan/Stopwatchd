//! Specific request types and arguments for them.

use serde::{Serialize, Deserialize};

/// Possible actions `swd` can take and the extra arguments the action needs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificArgs {
    Info(InfoArgs),
    Start(StartArgs),
    Stop(StopArgs),
    Play(PlayArgs),
    Pause(PauseArgs),
    Lap(LapArgs),
    Delete(DeleteArgs)
}

/// Request for information about stopwatches managed by `swd`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoArgs;

/// Get `swd` to create a new [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartArgs;

/// Stop a [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopArgs;

/// Request to play a [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayArgs;

/// Request to pause a [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseArgs;

/// Request to create a new lap.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapArgs;

/// Delete action.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteArgs;

macro_rules! impl_into_specific_args {
    ( $( $datatype:ty, $variant:ident ),* ) => {
        $(
            impl Into<SpecificArgs> for $datatype {
                fn into(self) -> SpecificArgs {
                    SpecificArgs::$variant(self)
                }
            }
        )*
    };
}

impl_into_specific_args!(InfoArgs, Info, StartArgs, Start, StopArgs, Stop,
    PlayArgs, Play, PauseArgs, Pause, LapArgs, Lap, DeleteArgs, Delete);
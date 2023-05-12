//! Specific request types and arguments for them.

use serde::{Serialize, Deserialize};

use crate::impl_into_enum_variant;

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

impl_into_enum_variant!(SpecificArgs {
    Info(InfoArgs),
    Start(StartArgs),
    Stop(StopArgs),
    Play(PlayArgs),
    Pause(PauseArgs),
    Lap(LapArgs),
    Delete(DeleteArgs)
});
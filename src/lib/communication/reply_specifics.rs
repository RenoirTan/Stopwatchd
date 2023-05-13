//! Additional information for replies from `swd`. Crucially, this allows
//! clients like `swctl` to confirm that their request went through and the
//! correct action was taken.

use serde::{Serialize, Deserialize};

use crate::{identifiers::Identifier, impl_into_enum_variant};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificAnswer {
    Info(InfoAnswer),
    Start(StartAnswer),
    Stop(StopAnswer),
    Play(PlayAnswer),
    Pause(PauseAnswer),
    Lap(LapAnswer),
    Delete(DeleteAnswer)
}

/// Kind of information coming from `swd`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfoAnswer {
    /// [`StopwatchDetails`] are returned in the order requested by the client.
    #[default] Basic,
    /// No stopwatch in particular was requested.
    All(InfoAll)
}

/// Stores details on how information should be presented when no particular
/// [`Stopwatch`] or stopwatches were requested.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoAll {
    /// Order in which stopwatches were last accessed.
    /// Provides a sequence that the client can show details in.
    pub access_order: Vec<Identifier>
}

/// Reply from `swd` after creating new stopwatches.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartAnswer;

/// Reply from `swd` after stopping a [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopAnswer;

/// Reply from `swd` after playing a [`Stopwatch`]'s lap.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayAnswer;

/// Reply from `swd` after pausing a [`Stopwatch`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauseAnswer;

/// Reply from `swd` after creating new laps.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LapAnswer;

/// Reply from `swd` after deleting [`Stopwatch`]es.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteAnswer;

impl_into_enum_variant!(SpecificAnswer {
    Info(InfoAnswer),
    Start(StartAnswer),
    Stop(StopAnswer),
    Play(PlayAnswer),
    Pause(PauseAnswer),
    Lap(LapAnswer),
    Delete(DeleteAnswer)
});
use std::io;

use ciborium::{ser::into_writer, de::from_reader};
use serde::{Serialize, Deserialize};

use crate::{models::stopwatch::Stopwatch, communication::details::StopwatchDetails};

pub trait Codecable<'a>: Serialize + Deserialize<'a> {
    fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![];
        match into_writer(self, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }

    fn from_bytes(buffer: &dyn AsRef<[u8]>) -> io::Result<Self> {
        from_reader(buffer.as_ref()).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidInput, e)
        )
    }
}

/// Automatically implemented if [`FromDetails`] is implemented.
/// [`FromDetails`] itself may also be automatically implemented.
/// Click on the link to see when that might happen under what conditions.
pub trait FromStopwatch {
    fn from_stopwatch(stopwatch: &Stopwatch, verbose: bool) -> Self;
}

pub trait FromStopwatches {
    fn from_stopwatches<'s, I>(iter: I, verbose: bool) -> Self
    where
        I: Iterator<Item = &'s Stopwatch>;
}

impl<T: FromDetails> FromStopwatches for T {
    fn from_stopwatches<'s, I>(iter: I, verbose: bool) -> Self
    where
        I: Iterator<Item = &'s Stopwatch>
    {
        Self::from_details(iter.map(|sw| StopwatchDetails::from_stopwatch(sw, verbose)))
    }
}

/// Automatically implemented if [`FromSuccessfuls`] is implemented where
/// [`FromSuccessfuls::Successful`] is [`From<StopwatchDetails>`]
pub trait FromDetails {
    fn from_details<I>(iter: I) -> Self
    where
        I: Iterator<Item = StopwatchDetails>;
}

impl<S, T> FromDetails for T
where
    S: From<StopwatchDetails>,
    T: FromSuccessfuls<Successful = S>
{
    fn from_details<I>(iter: I) -> Self
    where
        I: Iterator<Item = StopwatchDetails>
    {
        Self::from_successfuls(iter.map(|d| S::from(d)))
    }
}

pub trait FromSuccessfuls {
    type Successful;

    fn from_successfuls<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self::Successful>;
}
//! Compatibility between [`clap`] and Stopwatchd's logging facility.

use std::fmt;

use clap::ValueEnum;
use log::LevelFilter;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<LevelFilter>::into(*self))
    }
}

macro_rules! one_to_one {
    ($me:expr, $first:ident, $second:ident, $($variant:ident),+ $(,)?) => {
        match $me {
            $($first::$variant => $second::$variant,)+
        }
    };
}

impl From<LevelFilter> for LogLevel {
    fn from(filter: LevelFilter) -> Self {
        one_to_one!(filter, LevelFilter, LogLevel, Off, Error, Warn, Info, Debug, Trace)
    }
}

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        one_to_one!(self, LogLevel, LevelFilter, Off, Error, Warn, Info, Debug, Trace)
    }
}

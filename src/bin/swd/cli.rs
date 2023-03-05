use std::fmt;

use clap::{Parser, ValueEnum};
use log::LevelFilter;
use stopwatchd::logging::DEFAULT_LOGGER_LEVEL;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the log level for the daemon.
    #[arg(short, long, default_value_t = DEFAULT_LOGGER_LEVEL.into(), value_enum)]
    pub log_level: LogLevel
}

#[derive(Copy, Clone, ValueEnum)]
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
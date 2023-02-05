use std::{process, panic, backtrace::Backtrace};

use log::{SetLoggerError, LevelFilter};
use syslog::{Formatter3164, Facility, BasicLogger};

// If debug
#[cfg(debug_assertions)]
pub const DEFAULT_LOGGER_LEVEL: LevelFilter = LevelFilter::Trace;

// If release
#[cfg(not(debug_assertions))]
pub const DEFAULT_LOGGER_LEVEL: LevelFilter = LevelFilter::Info;

pub fn create_syslogger(process: &str) -> syslog::Result<BasicLogger> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_DAEMON,
        hostname: None,
        process: process.into(),
        pid: process::id()
    };
    let logger = syslog::unix(formatter)?;
    Ok(BasicLogger::new(logger))
}

pub fn set_syslogger(logger: BasicLogger, level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(level)) //  Necessary for messages to show in log
}

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        println!("{}", panic_info);
        println!("stack backtrace:\n{}", backtrace);
        error!("{}", panic_info);
    }));
}

pub fn setup(process: &str, level: Option<LevelFilter>) -> syslog::Result<()> {
    let level = level.unwrap_or(DEFAULT_LOGGER_LEVEL);
    let logger = create_syslogger(process)?;
    set_syslogger(logger, level).map_err(|e| {
        syslog::Error::with_chain(e, syslog::ErrorKind::Initialization)
    })?;
    Ok(())
}

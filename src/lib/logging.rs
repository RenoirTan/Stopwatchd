use std::{process, panic, backtrace::Backtrace};

use log::{SetLoggerError, LevelFilter};
use syslog::{Formatter3164, Facility, BasicLogger};

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

pub fn setup_syslogger(syslogger: BasicLogger) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(syslogger))
        .map(|()| log::set_max_level(LevelFilter::Trace)) //  Necessary for messages to show in log
}

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        println!("{}", panic_info);
        println!("stack backtrace:\n{}", backtrace);
        error!("{}", panic_info);
    }));
}

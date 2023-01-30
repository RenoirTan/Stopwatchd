use std::process;

#[macro_use]
extern crate log;
use stopwatchd::{logging::{create_syslogger, setup_syslogger, set_panic_hook}, pidfile::{open_pidfile, get_swd_pid}};

fn main() {
    let pid = process::id();
    { // Logging
        let logging_process_name = format!("sw-start_{}", pid);
        let logger = create_syslogger(&logging_process_name).unwrap();
        setup_syslogger(logger).unwrap();
        set_panic_hook();
    }
    info!("logging started");

    let swd_pid = {
        let mut pidfile = open_pidfile(false).unwrap();
        get_swd_pid(&mut pidfile).unwrap()
    };
    info!("swd_pid is {}", swd_pid);
}
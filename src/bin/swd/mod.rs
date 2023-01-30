use std::{
    fs::create_dir_all,
    process
};

#[macro_use]
extern crate log;
use stopwatchd::{
    runtime::{DEFAULT_RUNTIME_PATH, DEFAULT_PIDFILE_PATH},
    logging::{create_syslogger, setup_syslogger},
    util::press_enter_to_continue
};

use crate::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile},
    cleanup::Cleanup
};

mod cleanup;
mod pidfile;

fn main() {
    println!("starting swd");
    println!("setting up logging");
    let logging_process_name = format!("swd_{}", process::id());
    let logger = create_syslogger(&logging_process_name).unwrap();
    setup_syslogger(logger).unwrap();
    trace!("swd says trace");
    debug!("swd says debug");
    info!("swd says hi!!!");
    warn!("swd says warn");
    error!("swd says error");

    println!("setting up runtime directory: {}", DEFAULT_RUNTIME_PATH);
    create_dir_all(DEFAULT_RUNTIME_PATH).unwrap();

    println!("setting up pidfile");
    let mut pidfile = open_pidfile().unwrap();
    if pidfile_is_empty(&mut pidfile).unwrap() {
        write_pidfile(&mut pidfile).unwrap();
    } else {
        panic!("{} exists. Please delete it if no other swd is running", DEFAULT_PIDFILE_PATH);
    }
    drop(pidfile);

    press_enter_to_continue().unwrap();

    println!("cleaning up swd");
    Cleanup {remove_pidfile: true}.cleanup().unwrap();
}
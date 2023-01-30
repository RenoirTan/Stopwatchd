use std::{
    fs::create_dir_all,
    process
};

#[macro_use]
extern crate log;
use stopwatchd::{
    runtime::{DEFAULT_RUNTIME_PATH, DEFAULT_PIDFILE_PATH, server_socker_path},
    logging::{create_syslogger, setup_syslogger, set_panic_hook}
};

use crate::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile},
    cleanup::Cleanup, socket::{clear_socket, create_socket, listen_to_socket}
};

mod cleanup;
mod pidfile;
mod socket;

fn main() {
    let pid = process::id();
    { // Logging
        let logging_process_name = format!("swd_{}", pid);
        let logger = create_syslogger(&logging_process_name).unwrap();
        setup_syslogger(logger).unwrap();
        set_panic_hook();
    }
    info!("logging started");

    // Filesystem
    debug!("setting up runtime directory: {}", DEFAULT_RUNTIME_PATH);
    create_dir_all(DEFAULT_RUNTIME_PATH).unwrap();

    { // PID File
        debug!("setting up pidfile");
        let mut pidfile = open_pidfile().unwrap();
        if pidfile_is_empty(&mut pidfile).unwrap() {
            write_pidfile(&mut pidfile).unwrap();
        } else {
            panic!("{} exists. Please delete it if no other swd is running", DEFAULT_PIDFILE_PATH)
        }
    }

    { // Handle sockets
        let ssock_path = server_socker_path(Some(pid));
        clear_socket(&ssock_path).unwrap();
        let socket = create_socket(&ssock_path).unwrap();
        listen_to_socket(&socket);
    }

    // Clean up
    info!("cleaning up swd");
    Cleanup {remove_pidfile: true}.cleanup().unwrap();
    info!("going under!");
}
use std::{
    fs::create_dir_all,
    process, sync::{Arc, atomic::AtomicBool}, time::Duration
};

#[macro_use]
extern crate log;
use stopwatchd::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile},
    runtime::{DEFAULT_RUNTIME_PATH, DEFAULT_PIDFILE_PATH, server_socket_path},
    logging
};

use crate::{
    cleanup::Cleanup,
    socket::{clear_socket, create_socket, listen_to_socket}
};

mod cleanup;
mod socket;

fn main() {
    let pid = process::id();
    logging::setup(&format!("swd.{}", pid), None).unwrap();
    info!("logging started");

    // Filesystem
    debug!("setting up runtime directory: {}", DEFAULT_RUNTIME_PATH);
    create_dir_all(DEFAULT_RUNTIME_PATH).unwrap();

    // Setup interrupt handling
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();

    { // PID File
        debug!("setting up pidfile");
        let mut pidfile = open_pidfile(true).unwrap();
        if pidfile_is_empty(&mut pidfile).unwrap() {
            write_pidfile(&mut pidfile).unwrap();
        } else {
            panic!("{} exists. Please delete it if no other swd is running", DEFAULT_PIDFILE_PATH)
        }
    }

    // Handle sockets
    let ssock_path = server_socket_path(Some(pid));
    clear_socket(&ssock_path).unwrap();
    let socket = create_socket(&ssock_path).unwrap();
    socket.set_nonblocking(true).unwrap();
    listen_to_socket(&socket, terminate.clone(), Duration::from_millis(10));
    
    // Clean up
    info!("cleaning up swd");
    Cleanup {remove_pidfile: true, remove_sockfile: Some(&ssock_path)}.cleanup().unwrap();
    info!("going under!");
}
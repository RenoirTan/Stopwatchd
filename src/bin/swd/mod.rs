use std::{
    fs::create_dir_all,
    process
};

use clap::Parser;
#[macro_use]
extern crate log;
use stopwatchd::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile},
    runtime::{DEFAULT_RUNTIME_PATH, DEFAULT_PIDFILE_PATH, server_socket_path},
    logging
};
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    cleanup::Cleanup,
    signal::{handle_signals, get_signals},
    socket::{clear_socket, create_socket, listen_to_socket}
};

mod cleanup;
mod cli;
mod handlers;
mod signal;
mod socket;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let interval: u64 = cli.interval.unwrap_or(10);
    println!("interval: {}", interval);

    let pid = process::id();
    logging::setup(&format!("swd.{}", pid), None).unwrap();
    info!("logging started");

    // Filesystem
    debug!("setting up runtime directory: {}", DEFAULT_RUNTIME_PATH);
    create_dir_all(DEFAULT_RUNTIME_PATH).unwrap();

    // Setup interrupt handling
    let (signal_tx, signal_rx) = unbounded_channel();
    let signals = get_signals().unwrap();
    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals, signal_tx));

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

    // Application
    listen_to_socket(&socket, signal_rx).await;

    // Signal handling
    handle.close();
    signals_task.await.unwrap();
    
    // Clean up
    info!("cleaning up swd");
    Cleanup {remove_pidfile: true, remove_sockfile: Some(&ssock_path)}.cleanup().unwrap();
    info!("going under!");
}
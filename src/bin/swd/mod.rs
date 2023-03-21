use std::{
    fs::create_dir_all,
    process,
    sync::{
        Arc,
        atomic::AtomicBool
    }
};
#[cfg(feature = "swd-config")]
use std::sync::atomic::Ordering;

#[macro_use]
extern crate log;
use clap::Parser;
use stopwatchd::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile, pidfile_path},
    runtime::{runtime_dir, get_uid, server_socket_path},
    logging
};
use tokio::net::UnixListener;

use crate::{
    cleanup::Cleanup,
    signal::{make_signal_handler, close_signal_handler},
    socket::{clear_socket, create_socket, listen_to_socket, set_socket_perms},
    manager::{Manager, make_request_channels, manage, RequestSender},
};

mod cleanup;
mod config;
mod handlers;
mod manager;
mod signal;
mod socket;
mod utils;

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    let mut cli = config::Cli::parse();
    #[cfg(feature = "swd-config")]
    cli.supplement_file(None).unwrap();

    let log_level = cli.log_level().into();

    let uid = get_uid();

    let pid = process::id();
    logging::setup(&format!("swd.{}", pid), Some(log_level)).unwrap();
    info!("logging started");

    // Filesystem
    let rt_dir = runtime_dir(uid);
    debug!("setting up runtime directory: {:?}", rt_dir);
    create_dir_all(rt_dir).unwrap();

    // Start stopwatch manager
    // Must come before interrupt handler for some reason
    let manager = Manager::new();
    let (req_tx, req_rx) = make_request_channels();
    let manager_handle = tokio::spawn(manage(manager, req_rx));

    { // PID File
        debug!("setting up pidfile");
        let mut pidfile = open_pidfile(true, uid).unwrap();
        if pidfile_is_empty(&mut pidfile).unwrap() {
            write_pidfile(&mut pidfile).unwrap();
        } else {
            panic!("{:?} exists. Please delete it if no other swd is running", pidfile_path(uid))
        }
    }

    // Handle sockets
    let ssock_path = server_socket_path(Some(pid), uid);
    clear_socket(&ssock_path).unwrap();
    let socket = create_socket(&ssock_path).unwrap();
    set_socket_perms(&ssock_path).unwrap();


    #[cfg(not(feature = "swd-config"))]
    run(&socket, &req_tx).await;
    #[cfg(feature = "swd-config")]
    run(&socket, &req_tx, &cli.config_path).await;

    // Clean up manager
    debug!("cleaning up manager");
    drop(req_tx); // Force close manager_handle
    manager_handle.await.unwrap();
    
    // Clean up
    info!("cleaning up swd");
    Cleanup { uid, remove_pidfile: true, remove_sockfile: Some(&ssock_path) }.cleanup().unwrap();
    info!("going under!");
}

#[cfg(not(feature = "swd-config"))]
async fn run(socket: &UnixListener, req_tx: &RequestSender) {
    // Setup interrupt handling
    let restart = Arc::new(AtomicBool::new(true)); // Useless
    let (handle, signals_task, signal_rx) = make_signal_handler(restart);

    // * START OF MAIN LOGIC *
    listen_to_socket(&socket, signal_rx, req_tx.clone()).await;

    // Signal handling
    debug!("closing signals");
    close_signal_handler(handle, signals_task).await;
}

#[cfg(feature = "swd-config")]
async fn run(socket: &UnixListener, req_tx: &RequestSender, config_path: &str) {
    let restart = Arc::new(AtomicBool::new(true));
    // Application
    while restart.load(Ordering::Relaxed) {
        debug!("restarting after signal");
        // Setup interrupt handling
        let (handle, signals_task, signal_rx) = make_signal_handler(restart.clone());

        // * START OF MAIN LOGIC *
        listen_to_socket(&socket, signal_rx, req_tx.clone()).await;

        // Signal handling
        debug!("closing signals");
        close_signal_handler(handle, signals_task).await;

        // Why we need do whiles
        if restart.load(Ordering::Relaxed) {
            let mut cli = config::Cli::default();
            cli.supplement_file(Some(config_path)).unwrap();
            log::set_max_level(cli.log_level().into());
            info!("logging started");
        }
    }
}
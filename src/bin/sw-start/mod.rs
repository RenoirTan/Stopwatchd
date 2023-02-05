use std::{
    process,
    os::unix::net::UnixStream,
    io::{Write, Read},
    time::Duration
};

#[macro_use]
extern crate log;
use stopwatchd::{
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path,
    logging
};

fn main() {
    let pid = process::id();
    logging::setup(&format!("sw-start.{}", pid), None).unwrap();
    info!("logging started");

    let swd_pid = {
        let mut pidfile = open_pidfile(false).unwrap();
        get_swd_pid(&mut pidfile).unwrap()
    };
    debug!("swd_pid is {}", swd_pid);

    let ssock_path = server_socket_path(Some(swd_pid));
    if ssock_path.exists() {
        debug!("{:?} exists", ssock_path);
    } else {
        debug!("{:?} does not exist", ssock_path);
    }
    trace!("connecting to {:?}", ssock_path);
    let mut stream = UnixStream::connect(&ssock_path).unwrap();
    stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    stream.set_write_timeout(Some(Duration::new(1, 0))).unwrap();
    trace!("connected to {:?}", ssock_path);

    info!("writing message to server");
    stream.write_all(b"hi").unwrap();
    stream.flush().unwrap();

    info!("waiting for response from server");
    let mut braw = vec![0; 256];
    info!("reading response from server");
    stream.read(&mut braw).unwrap();
    let response = String::from_utf8(braw).unwrap();
    println!("{}", response);

    info!("exiting");
}
use std::{
    process,
    os::unix::net::UnixStream,
    io::{Write, Read},
    time::Duration
};

#[macro_use]
extern crate log;
use stopwatchd::{
    logging::{create_syslogger, setup_syslogger, set_panic_hook},
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socket_path
};

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

    let ssock_path = server_socket_path(Some(swd_pid));
    if ssock_path.exists() {
        println!("{:?} exists", ssock_path);
    }
    info!("creating stream");
    let mut stream = UnixStream::connect(&ssock_path).unwrap();
    stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    stream.set_write_timeout(Some(Duration::new(1, 0))).unwrap();

    info!("writing message to server");
    stream.write_all(b"hi").unwrap();
    stream.flush().unwrap();

    info!("waiting for response from server");
    let mut braw = vec![0; 256];
    info!("reading response from server");
    stream.read(&mut braw).unwrap();
    let response = String::from_utf8(braw).unwrap();
    println!("{}", response);
}
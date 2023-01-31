use std::{
    process,
    os::unix::net::{UnixStream, UnixListener},
    io::{Write, Read}
};

#[macro_use]
extern crate log;
use stopwatchd::{
    logging::{create_syslogger, setup_syslogger, set_panic_hook},
    pidfile::{open_pidfile, get_swd_pid},
    runtime::server_socker_path
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

    info!("creating client socket");
    let csock_path = format!("/tmp/stopwatchd/sw-start_{}", pid);
    let listener = UnixListener::bind(&csock_path).unwrap();

    let ssock_path = server_socker_path(Some(swd_pid));
    info!("creating stream");
    let mut stream = UnixStream::connect(&ssock_path).unwrap();

    info!("writing message to server");
    stream.write_all(format!("{}//hi", csock_path).as_bytes()).unwrap();
    stream.flush().unwrap();

    info!("waiting for response from server");
    let (mut response_stream, _response_saddr) = listener.accept().unwrap();
    let mut response = String::new();
    info!("reading response from server");
    response_stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
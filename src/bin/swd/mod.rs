use std::fs::create_dir_all;

use stopwatchd::runtime::{DEFAULT_RUNTIME_PATH, DEFAULT_PIDFILE_PATH};

use crate::{
    pidfile::{open_pidfile, pidfile_is_empty, write_pidfile},
    cleanup::Cleanup
};

mod cleanup;
mod pidfile;

fn main() {
    println!("starting swd");
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

    println!("cleaning up swd");
    Cleanup {remove_pidfile: false}.cleanup().unwrap();
}
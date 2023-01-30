use std::{
    fs::{File, OpenOptions, remove_file},
    io::{Result, Read, Write},
    process
};

use stopwatchd::runtime::DEFAULT_PIDFILE_PATH;

pub fn open_pidfile() -> Result<File> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(DEFAULT_PIDFILE_PATH)
}

pub fn pidfile_is_empty(pidfile: &mut File) -> Result<bool> {
    let mut pidfile_content = String::new();
    pidfile.read_to_string(&mut pidfile_content)?;
    Ok(pidfile_content.is_empty())
}

pub fn write_pidfile(pidfile: &mut File) -> Result<usize> {
    let pidfile_content = format!("{}", process::id());
    pidfile.write(pidfile_content.as_bytes())
}

pub fn remove_pidfile() -> Result<()> {
    remove_file(DEFAULT_PIDFILE_PATH)
}
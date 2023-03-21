use std::{
    fs::{File, OpenOptions, remove_file},
    io::{Result, Read, Write, Error, ErrorKind},
    process,
    path::PathBuf
};

use crate::runtime::runtime_dir;

pub fn pidfile_path(uid: Option<u32>) -> PathBuf {
    runtime_dir(uid).join("pidfile")
}

pub fn open_pidfile(is_daemon: bool, uid: Option<u32>) -> Result<File> {
    let ppath = pidfile_path(uid);
    OpenOptions::new()
        .create(is_daemon)
        .read(true)
        .write(is_daemon)
        .open(ppath)
}

pub fn get_swd_pid(pidfile: &mut File) -> Result<u32> {
    let mut pidfile_content = String::new();
    pidfile.read_to_string(&mut pidfile_content)?;
    pidfile_content.parse::<u32>().map_err(|pie| Error::new(ErrorKind::InvalidData, pie))
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

pub fn remove_pidfile(uid: Option<u32>) -> Result<()> {
    remove_file(pidfile_path(uid))
}
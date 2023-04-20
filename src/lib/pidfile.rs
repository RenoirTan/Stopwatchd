//! Manage and read Stopwatchd PID files.

use std::{
    fs::{File, OpenOptions, remove_file},
    io::{Result, Read, Write, Error, ErrorKind},
    process,
    path::PathBuf
};

use crate::runtime::runtime_dir;

/// Get path to pidfile.
/// 
/// Supply an optional `uid` user id for `swd` that is run as a non-root user.
pub fn pidfile_path(uid: Option<u32>) -> PathBuf {
    runtime_dir(uid).join("pidfile")
}

/// Open/create a pidfile.
/// 
/// # Arguments
/// * is_daemon - Whether the program currently running is `swd`.
/// * uid - Set this to the relevant user id if `swd` is run by a non-root user.
/// 
/// See [`get_swd_pid`] on how to obtain the PID.
pub fn open_pidfile(is_daemon: bool, uid: Option<u32>) -> Result<File> {
    let ppath = pidfile_path(uid);
    OpenOptions::new()
        .create(is_daemon)
        .read(true)
        .write(is_daemon)
        .open(ppath)
}

/// Get PID from pidfile. The pidfile can be opened using [`open_pidfile`].
pub fn get_swd_pid(pidfile: &mut File) -> Result<u32> {
    let mut pidfile_content = String::new();
    pidfile.read_to_string(&mut pidfile_content)?;
    pidfile_content.parse::<u32>().map_err(|pie| Error::new(ErrorKind::InvalidData, pie))
}

/// Check if pidfile is empty. This is used by `swd` to check if another
/// instance is already running.
pub fn pidfile_is_empty(pidfile: &mut File) -> Result<bool> {
    let mut pidfile_content = String::new();
    pidfile.read_to_string(&mut pidfile_content)?;
    Ok(pidfile_content.is_empty())
}

/// Write the current process' PID to the pidfile.
pub fn write_pidfile(pidfile: &mut File) -> Result<usize> {
    let pidfile_content = format!("{}", process::id());
    pidfile.write(pidfile_content.as_bytes())
}

/// Clean up the pidfile to clear up space.
pub fn remove_pidfile(uid: Option<u32>) -> Result<()> {
    remove_file(pidfile_path(uid))
}
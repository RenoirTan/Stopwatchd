use std::path::PathBuf;

pub const DEFAULT_RUNTIME_PATH: &'static str = "/tmp/stopwatchd";
pub const DEFAULT_PIDFILE_PATH: &'static str = "/tmp/stopwatchd/pidfile";

pub fn server_socket_path(pid: Option<u32>) -> PathBuf {
    let path = PathBuf::from(DEFAULT_RUNTIME_PATH);
    match pid {
        Some(pid) => path.join(format!("server_socket_{}", pid)),
        None => path.join(format!("server_socket"))
    }
}
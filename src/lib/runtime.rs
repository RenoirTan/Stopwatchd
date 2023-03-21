use std::path::PathBuf;
#[cfg(feature = "users")]
use std::env;

#[cfg(feature = "users")]
use users::get_current_uid;

pub const SYSTEM_RUNTIME_PATH: &'static str = "/tmp/stopwatchd";

pub fn get_uid() -> Option<u32> {
    #[cfg(not(feature = "users"))]
    return None;

    #[cfg(feature = "users")]
    return Some(get_current_uid())
}

pub fn socket_name(pid: Option<u32>) -> String {
    match pid {
        Some(pid) => format!("swd.{}.sock", pid),
        None => "swd.sock".to_string()
    }
}

pub fn server_socket_path(pid: Option<u32>, uid: Option<u32>) -> PathBuf {
    runtime_dir(uid).join(socket_name(pid))
}

#[cfg(feature = "users")]
pub fn user_runtime_path(fallback: Option<String>, uid: u32) -> PathBuf {
    PathBuf::from(env::var("XDG_RUNTIME_DIR")
        .map(|s| s + "/stopwatchd") // $XDG_RUNTIME_DIR/stopwatchd
        .ok()
        .or_else(|| fallback) // Use fallback
        .unwrap_or_else(|| format!("/run/user/{}/stopwatchd", uid))) // If no fallback, use default
}

pub fn runtime_dir(uid: Option<u32>) -> PathBuf {
    match uid {
        #[cfg(feature = "users")]
        Some(uid) if uid != 0 => user_runtime_path(None, uid),
        _ => PathBuf::from(SYSTEM_RUNTIME_PATH)
    }
}
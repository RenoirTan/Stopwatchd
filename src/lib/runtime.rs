//! Special functions for runtime-dependent data.

use std::path::PathBuf;
#[cfg(feature = "users")]
use std::env;

#[cfg(feature = "users")]
use users::get_current_uid;

/// Path to default system runtime directory.
pub const SYSTEM_RUNTIME_PATH: &'static str = "/tmp/stopwatchd";

/// Get uid of the current user.
/// 
/// If the `users` feature is enabled, then [`Some`] is returned containing the
/// user id.
/// 
/// Otherwise, [`None`] is returned since user id doesn't matter.
pub fn get_uid() -> Option<u32> {
    #[cfg(not(feature = "users"))]
    return None;

    #[cfg(feature = "users")]
    return Some(get_current_uid())
}

/// Get name of the socket file used to communicate with `swd`.
/// The name of the socket file contains the PID of `swd`, which needs to be
/// passed in as an argument.
pub fn socket_name(pid: Option<u32>) -> String {
    match pid {
        Some(pid) => format!("swd.{}.sock", pid),
        None => "swd.sock".to_string()
    }
}

/// Get path to the socket file used to communicate with `swd`.
/// 
/// `uid` can be taken from [`get_uid`].
/// 
/// See [`socket_name`] for more information of `pid`.
pub fn server_socket_path(pid: Option<u32>, uid: Option<u32>) -> PathBuf {
    runtime_dir(uid).join(socket_name(pid))
}

/// Get runtime directory for `swd` being run by a non-root user.
/// 
/// This function tries the following directories in this order:
///     1. `$XDG_RUNTIME_DIR/stopwatchd`
///     2. `fallback`, if provided
///     3. `/run/user/{uid}/stopwatchd`
#[cfg(feature = "users")]
pub fn user_runtime_path(fallback: Option<String>, uid: u32) -> PathBuf {
    PathBuf::from(env::var("XDG_RUNTIME_DIR")
        .map(|s| s + "/stopwatchd") // $XDG_RUNTIME_DIR/stopwatchd
        .ok()
        .or_else(|| fallback) // Use fallback
        .unwrap_or_else(|| format!("/run/user/{}/stopwatchd", uid))) // If no fallback, use default
}

/// Get runtime directory.
/// 
/// If the `users` feature is enabled, and `uid` is non-zero (i.e. not root),
/// then a user runtime directory is returned.
/// 
/// Otherwise a system runtime directory is used.
/// 
/// `uid` can be taken from [`get_uid`].
pub fn runtime_dir(uid: Option<u32>) -> PathBuf {
    match uid {
        #[cfg(feature = "users")]
        Some(uid) if uid != 0 => user_runtime_path(None, uid),
        _ => PathBuf::from(SYSTEM_RUNTIME_PATH)
    }
}
//! Remove things created by `swd` during runtime.

use std::{io::Result, path::Path, fs::remove_file};

use stopwatchd::pidfile::remove_pidfile;

/// Configuration for what to clean up.
pub struct Cleanup<'sock> {
    /// Uid of user. [`None`] if root or `users` feature not enabled.
    pub uid: Option<u32>,
    /// Whether to remove Pidfile.
    pub remove_pidfile: bool,
    /// Whether to remove socket file. If so, provide a path.
    pub remove_sockfile: Option<&'sock dyn AsRef<Path>>
}

impl<'sock> Cleanup<'sock> {
    /// Perform clean up based on the configs in [`Cleanup`].
    pub fn cleanup(&self) -> Result<()> {
        trace!("cleanup called");
        if self.remove_pidfile {
            trace!("remove pidfile specified, removing...");
            remove_pidfile(self.uid)?;
            trace!("remove pidfile successful");
        }
        if let Some(ssock_path) = self.remove_sockfile {
            trace!("remove_sockfile specified, removing...");
            remove_file(ssock_path)?;
            trace!("remove_sockfile successful");
        }
        Ok(())
    }
}
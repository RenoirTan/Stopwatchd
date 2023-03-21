use std::{io::Result, path::Path, fs::remove_file};

use stopwatchd::pidfile::remove_pidfile;

pub struct Cleanup<'sock> {
    pub uid: Option<u32>,
    pub remove_pidfile: bool,
    pub remove_sockfile: Option<&'sock dyn AsRef<Path>>
}

impl<'sock> Cleanup<'sock> {
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
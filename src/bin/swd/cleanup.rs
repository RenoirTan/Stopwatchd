use std::io::Result;

use stopwatchd::pidfile::remove_pidfile;

pub struct Cleanup {
    pub remove_pidfile: bool
}

impl Cleanup {
    pub fn cleanup(&self) -> Result<()> {
        trace!("cleanup called");
        if self.remove_pidfile {
            trace!("remove pidfile specified, removing...");
            remove_pidfile()?;
            trace!("remove pidfile successful");
        }
        Ok(())
    }
}
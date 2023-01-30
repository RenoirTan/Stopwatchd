use std::io::Result;

use crate::pidfile::remove_pidfile;

pub struct Cleanup {
    pub remove_pidfile: bool
}

impl Cleanup {
    pub fn cleanup(&self) -> Result<()> {
        if self.remove_pidfile {
            remove_pidfile()?;
        }
        Ok(())
    }
}
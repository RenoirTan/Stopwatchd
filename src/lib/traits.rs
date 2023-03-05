use std::io;

use ciborium::{ser::into_writer, de::from_reader};
use serde::{Serialize, Deserialize};

pub trait Codecable<'a>: Serialize + Deserialize<'a> {
    fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![];
        match into_writer(self, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }

    fn from_bytes(buffer: &dyn AsRef<[u8]>) -> io::Result<Self> {
        from_reader(buffer.as_ref()).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidInput, e)
        )
    }
}
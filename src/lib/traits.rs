//! Custom traits for types in `stopwatchd`.

use std::io;

use ciborium::{ser::into_writer, de::from_reader};
use serde::{Serialize, Deserialize};

/// Convert to and from byte representation (using [`ciborium`]).
pub trait Codecable<'a>: Serialize {
    /// Encode data type to CBOR bytes using [`ciborium`].
    fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![];
        match into_writer(self, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }

    /// Decode CBOR bytes into data type using [`ciborium`].
    fn from_bytes(buffer: &dyn AsRef<[u8]>) -> io::Result<Self> where for <'de> Self: Deserialize<'de> {
        from_reader(buffer.as_ref()).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidInput, e)
        )
    }
}

impl<'a, T> Codecable<'a> for T where T: Serialize + Deserialize<'a> { }

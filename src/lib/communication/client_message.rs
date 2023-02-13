use std::{process, io::{self, Write}};

use crate::communication::intention::Intention;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientMessage {
    pub pid: u32,
    pub intention: Intention,
    pub message: Vec<u8>
}

impl ClientMessage {
    pub fn create(intention: Intention, message: Vec<u8>) -> Self {
        let pid = process::id();
        Self { pid, intention, message }
    }

    /// Format: `<pid><intention>=<message>`
    pub fn write_as_bytes(&self, destination: &mut dyn Write) -> io::Result<usize> {
        let mut bytes_written = 0;

        bytes_written += destination.write(&self.pid.to_le_bytes())?;
        bytes_written += self.intention.write_as_bytes(destination)?;
        bytes_written += destination.write(b"=")?;
        bytes_written += destination.write(&self.message)?;

        Ok(bytes_written)
    }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut output = vec![];
        self.write_as_bytes(&mut output)?;
        Ok(output)
    }

    pub fn from_bytes(input: &[u8]) -> io::Result<Self> {
        let len = input.len();
        if len < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "malformed pid"));
        }
        let pid = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);

        // [4..] to skip past pid
        let mut iter = input[4..].splitn(2, |b| *b == b'=');

        let raw_intention = iter.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no intention found"))?;
        let intention = Intention::from_bytes(raw_intention)?;
        
        let raw_msg = iter.next().unwrap_or(b"");
        let message = raw_msg.to_vec();

        Ok(Self { pid, intention, message })
    }
}

impl Default for ClientMessage {
    fn default() -> Self {
        let intention = Intention::default();
        let message = Vec::default();
        Self::create(intention, message)
    }
}

impl From<Intention> for ClientMessage {
    fn from(intention: Intention) -> Self {
        Self::create(intention, Vec::default())
    }
}

impl TryFrom<&[u8]> for ClientMessage {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl TryInto<Vec<u8>> for ClientMessage {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::communication::intention::{Intention, Command};

    use super::ClientMessage;

    #[test]
    fn test_client_message_to_bytes() {
        let pid = 0xabcdef;
        let intention = Intention { command: Command::Info, verbose: true };
        let message = b"test message".to_vec();

        let cm = ClientMessage { pid, intention, message };

        let result = cm.to_bytes().unwrap();

        assert_eq!(result, b"\xef\xcd\xab\x00info:verbose=test message");
    }

    #[test]
    fn test_bytes_to_client_message() {
        let raw = b"\x01\x00\x00\x00start:verbose=something";
        let cm = ClientMessage::from_bytes(raw).unwrap();
        let correct = ClientMessage {
            pid: 1,
            intention: Intention {
                command: Command::Start,
                verbose: true
            },
            message: b"something".to_vec()
        };

        assert_eq!(cm, correct);
    }

    #[test]
    fn test_bytes_to_client_message_2() {
        let raw = b"\x21\x43\x65\x87play";
        let cm = ClientMessage::from_bytes(raw).unwrap();
        let correct = ClientMessage {
            pid: 0x87654321,
            intention: Intention {
                command: Command::Play,
                verbose: false
            },
            message: vec![]
        };
        assert_eq!(cm, correct);
    }

    #[test]
    fn test_bytes_to_client_message_bad() {
        let raw = b"aaa_nosuchthing";
        assert!(ClientMessage::from_bytes(raw).is_err());
    }

    #[test]
    fn test_bytes_to_client_message_bad_2() {
        let raw = b"<3";
        assert!(ClientMessage::from_bytes(raw).is_err());
    }
}
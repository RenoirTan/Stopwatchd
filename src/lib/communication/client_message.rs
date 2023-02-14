use std::{process, io};

use serde::{Serialize, Deserialize};

use crate::{communication::intention::Intention, traits::Codecable};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl Codecable<'_> for ClientMessage { }

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
        Self::from_bytes(&value)
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
    use crate::{
        communication::intention::{Intention, Command},
        traits::Codecable
    };

    use super::ClientMessage;

    #[test]
    fn test_cycle_0() {
        let cm = ClientMessage {
            pid: 100,
            intention: Intention {
                command: Command::Play,
                verbose: false
            },
            message: b"random_message".to_vec()
        };

        let encoded = cm.to_bytes().unwrap();
        let decoded = ClientMessage::from_bytes(&encoded).unwrap();

        assert_eq!(cm, decoded);
    }

    #[test]
    fn test_cycle_1() {
        let cm = ClientMessage {
            pid: 0x87654321,
            intention: Intention {
                command: Command::Start,
                verbose: true
            },
            message: b"more things".to_vec()
        };

        let encoded = cm.to_bytes().unwrap();
        let decoded = ClientMessage::from_bytes(&encoded).unwrap();

        assert_eq!(cm, decoded);
    }
}
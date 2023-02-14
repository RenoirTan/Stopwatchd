use std::io;

use serde::{Serialize, Deserialize};

use crate::traits::Codecable;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Start,
    End,
    Delete,
    Play,
    Pause,
    Lap,
    #[default] Info
}

impl Codecable<'_> for Command { }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Intention {
    pub command: Command,
    pub verbose: bool
}

impl Intention {
    pub fn from_command(command: Command) -> Self {
        Self { command, verbose: false }
    }
}

impl Codecable<'_> for Intention { }

impl From<Command> for Intention {
    fn from(command: Command) -> Self {
        Self::from_command(command)
    }
}

impl TryFrom<&[u8]> for Intention {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl Default for Intention {
    fn default() -> Self {
        let command = Command::default();
        Self { command, verbose: false }
    }
}

impl TryInto<Vec<u8>> for Intention {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Codecable;

    use super::{Intention, Command};

    #[test]
    fn test_commands_cycle() {
        use Command::*;

        fn assert_command(command: Command) {
            let encoded = command.to_bytes().unwrap();
            let decoded = Command::from_bytes(&encoded).unwrap();
            assert_eq!(command, decoded);
        }

        assert_command(Start);
        assert_command(End);
        assert_command(Delete);
        assert_command(Play);
        assert_command(Pause);
        assert_command(Lap);
        assert_command(Info);
    }

    #[test]
    fn test_intention_to_bytes_to_intention() {
        let intention = Intention { command: Command::Pause, verbose: true };
        let bytes = intention.to_bytes().unwrap();
        let intention2 = Intention::from_bytes(&bytes).unwrap();
        assert_eq!(intention, intention2);
    }
}
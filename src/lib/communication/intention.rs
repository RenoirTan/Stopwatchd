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

/*
    /// Convert intention into bytes and write into anything that writes bytes.
    /// 
    /// Format: `<command>(:verbose)?`
    pub fn write_as_bytes(&self, destination: &mut dyn Write) -> io::Result<usize> {
        let mut bytes_written = 0;

        // write command
        bytes_written += destination.write(&self.command.to_bytes())?;

        // write verbose if self.verbose is true
        if self.verbose {
            bytes_written += destination.write(b":verbose")?;
        }

        Ok(bytes_written)
    } */

/*     pub fn from_bytes(input: &[u8]) -> io::Result<Self> {
        let mut parts_iter = input.split(|b| *b == b':');

        let raw_command = parts_iter.next().ok_or_else(||
            io::Error::new(io::ErrorKind::InvalidInput, "could not iterate to command")
        )?;
        let command = Command::from_bytes(raw_command)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid command string"))?;
        
        let mut verbose = false;
        for bflag in parts_iter {
            if bflag == b"verbose" {
                verbose = true;
            }
        }

        Ok(Self { command, verbose })
    } */
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

    /* #[test]
    fn test_intention_start_to_bytes() {
        let intention = Intention { command: Command::Start, verbose: false };
        let bytes = intention.to_bytes().unwrap();
        assert_eq!(bytes, b"start");
    }

    #[test]
    fn test_intention_start_verbose_to_bytes() {
        let intention = Intention { command: Command::Start, verbose: true };
        let bytes = intention.to_bytes().unwrap();
        assert_eq!(bytes, b"start:verbose")
    }

    #[test]
    fn test_intention_info_to_bytes() {
        let intention = Intention { command: Command::Info, verbose: false };
        let bytes = intention.to_bytes().unwrap();
        assert_eq!(bytes, b"info");
    }

    #[test]
    fn test_bytes_delete_to_intention() {
        let bytes = b"delete";
        let intention = Intention { command: Command::Delete, verbose: false };
        assert_eq!(Intention::from_bytes(bytes).unwrap(), intention);
    }

    #[test]
    fn test_bytes_lap_verbose_to_intention() {
        let bytes = b"lap:verbose";
        let intention = Intention { command: Command::Lap, verbose: true };
        assert_eq!(Intention::from_bytes(bytes).unwrap(), intention);
    }

    #[test]
    fn test_bad_bytes_to_intention() {
        let bytes = b"no";
        let intention = Intention::from_bytes(bytes);
        assert!(intention.is_err());
    } */

    #[test]
    fn test_intention_to_bytes_to_intention() {
        let intention = Intention { command: Command::Pause, verbose: true };
        let bytes = intention.to_bytes().unwrap();
        let intention2 = Intention::from_bytes(&bytes).unwrap();
        assert_eq!(intention, intention2);
    }

    /* #[test]
    fn test_bytes_to_intention_to_bytes() {
        let bytes = b"end";
        let intention = Intention::from_bytes(bytes).unwrap();
        let bytes2 = intention.to_bytes().unwrap();
        assert_eq!(bytes, bytes2.as_slice());
    } */
}
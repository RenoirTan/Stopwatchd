use std::{
    io::{self, Read},
    str::FromStr,
    fs::OpenOptions
};

use clap::Parser;
use log::LevelFilter;
use stopwatchd::logging::{DEFAULT_LOGGER_LEVEL, cli::LogLevel};
use toml::{Table, Value};

pub const DEFAULT_CONFIG_PATH: &'static str = "/etc/stopwatchd/swd.toml";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the log level for the daemon.
    #[arg(short, long, value_enum, help = "Set log level")]
    pub log_level: Option<LogLevel>
}

impl Cli {
    pub fn collect(config_path: &str) -> Result<Self, io::Error> {
        let mut cli = Self::parse();
        cli.supplement_file(config_path)?;
        Ok(cli)
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level.unwrap_or(DEFAULT_LOGGER_LEVEL.into())
    }

    pub fn supplement_toml(&mut self, table: Table) -> Result<&mut Self, io::Error> {
        if let None = self.log_level {
            self.log_level = match table.get("log_level") {
                Some(Value::String(s)) => Some(LevelFilter::from_str(s).map_err(|e|
                    io::Error::new(io::ErrorKind::InvalidInput, format!("config file error: {}", e))
                )?.into()),
                None => None,
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "log_level in config file must be a string"
                ))
            };
        }
        Ok(self)
    }

    pub fn supplement_file(&mut self, config_path: &str) -> Result<&mut Self, io::Error> {
        let mut config_file = match OpenOptions::new()
            .read(true)
            .open(config_path)
        {
            Ok(f) => f,
            Err(e) => {
                println!("skipping config file, '{}' could not be opened: {}", config_path, e);
                return Ok(self)
            }
        };
        let mut config_raw = String::new();
        config_file.read_to_string(&mut config_raw)?;
        let table = config_raw.parse::<Table>()
            .map_err(|e| io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("could not parse {}: {}", config_path, e)
            ))?;
        self.supplement_toml(table)?;

        Ok(self)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self { log_level: None }
    }
}
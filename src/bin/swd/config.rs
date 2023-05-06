//! Configuration and command line arguments for `swd`.

#[cfg(feature = "swd-config")]
use std::{
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
    fs::OpenOptions
};
#[cfg(all(feature = "swd-config", feature = "users"))]
use std::env;

use clap::Parser;
#[cfg(feature = "swd-config")]
use log::LevelFilter;
use stopwatchd::logging::{DEFAULT_LOGGER_LEVEL, cli::LogLevel};
#[cfg(feature = "swd-config")]
use toml::{Table, Value};
#[cfg(feature = "users")]
use users::{get_user_by_uid, get_current_uid};

/// Default system config file for `swd`.
#[cfg(feature = "swd-config")]
pub const SYSTEM_CONFIG_PATH: &'static str = "/etc/stopwatchd/swd.toml";

/// Get config file path for a user.
/// 
/// # Arguments
/// - config_home: Fallback config_home, tried after function looks for
/// `$XDG_CONFIG_HOME` and `$HOME/.config`.
#[cfg(all(feature = "swd-config", feature = "users"))]
pub fn user_config_path(config_home: Option<String>) -> Result<PathBuf, env::VarError> {
    // $HOME/.config/stopwatchd/swd.toml
    let config_home = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|s| s + "/.config"))
        .or_else(|_| config_home.ok_or_else(|| env::VarError::NotPresent))?; // Fallback
    Ok(PathBuf::from(config_home).join("stopwatchd/swd.toml"))
}

/// Use [`get_config_path`].
#[cfg(all(feature = "swd-config", feature = "users"))]
pub fn calculate_config_path() -> PathBuf {
    match get_current_uid() {
        0 => PathBuf::from(SYSTEM_CONFIG_PATH),
        uid => {
            let username = get_user_by_uid(uid).unwrap().name().to_str().unwrap().to_string();
            user_config_path(Some(username)).unwrap()
        }
    }
}

/// Find config path for `swd`.
#[cfg(feature = "swd-config")]
pub fn get_config_path() -> PathBuf {
    #[cfg(not(feature = "users"))]
    return PathBuf::from(SYSTEM_CONFIG_PATH);
    
    #[cfg(feature = "users")]
    return calculate_config_path();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the log level for the daemon.
    #[arg(short, long, value_enum, help = "Set log level")]
    pub log_level: Option<LogLevel>,

    /// Path to config file.
    #[cfg(feature = "swd-config")]
    #[arg(
        short = 'c',
        long = "config",
        default_value_t = get_config_path().to_str().unwrap().to_string()
    )]
    pub config_path: String
}

impl Cli {
    /// Get minimum [`LogLevel`].
    pub fn log_level(&self) -> LogLevel {
        self.log_level.unwrap_or(DEFAULT_LOGGER_LEVEL.into())
    }

    /// Combine command line arguments stored in `self`
    /// with options in TOML [`Table`].
    #[cfg(feature = "swd-config")]
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

    /// Combine command line arguments stored in `self`
    /// with config file at `config_path`.
    #[cfg(feature = "swd-config")]
    pub fn supplement_file(&mut self, config_path: Option<&str>) -> Result<&mut Self, io::Error> {
        let config_path = config_path.unwrap_or(&self.config_path);
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
        // `return` because rust doesn't know how to deal with expressions with
        // cfg yet

        #[cfg(not(feature = "swd-config"))]
        return Self { log_level: None };

        #[cfg(feature = "swd-config")]
        return Self { log_level: None, config_path: SYSTEM_CONFIG_PATH.to_string() };
    }
}
use clap::Parser;
use stopwatchd::logging::{DEFAULT_LOGGER_LEVEL, cli::LogLevel};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the log level for the daemon.
    #[arg(short, long, default_value_t = DEFAULT_LOGGER_LEVEL.into(), value_enum)]
    pub log_level: LogLevel
}
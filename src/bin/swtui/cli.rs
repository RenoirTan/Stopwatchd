use clap::Parser;
use stopwatchd::logging::{cli::LogLevel, DEFAULT_LOGGER_LEVEL};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Create a new stopwatch on startup. The new stopwatch will be focused
    /// if created successfully.
    #[arg(long = "new", global = true)]
    pub new_stopwatch: Option<String>,

    /// Focus on a stopwatch on startup.
    #[arg(short = 'f', long = "focus", global = true)]
    pub focus_stopwatch: Option<String>,

    /// Whether to communicate with system swd instead of user-started swd.
    #[cfg(feature = "users")]
    #[arg(long = "system", global = true)]
    pub system_swd: bool,

    /// Show the cursor in the terminal. You shouldn't have to turn this on
    /// unless you are trying to debug something.
    #[arg(short = 'c', long = "cursor", global = true)]
    pub show_cursor: bool,

    /// Set the log level for the daemon.
    #[arg(short, long, default_value_t = DEFAULT_LOGGER_LEVEL.into(), value_enum)]
    pub log_level: LogLevel,
}

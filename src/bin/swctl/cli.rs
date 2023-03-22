use clap::{Parser, Subcommand, Args};
use stopwatchd::logging::{cli::LogLevel, DEFAULT_LOGGER_LEVEL};

use crate::formatted::{DEFAULT_DATETIME_FORMAT, DEFAULT_DURATION_FORMAT, Styles};

#[derive(Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Subcommands,

    /// Display detailed information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Strftime format for durations
    #[arg(
        short = 'd',
        long = "dur-fmt",
        global = true,
        default_value_t = DEFAULT_DURATION_FORMAT.to_string()
    )]
    pub duration_fmt: String,

    /// Show date-time information. This can refer to the times when a stopwatch
    /// or its laps were started.
    #[arg(
        long = "show-dt",
        global = true
    )]
    pub show_datetime_info: bool,

    /// Strftime format for date-time fields
    #[arg(
        short = 'D',
        long = "dt-fmt",
        global = true,
        default_value_t = DEFAULT_DATETIME_FORMAT.to_string()
    )]
    pub datetime_fmt: String,

    /// Table appearance
    #[arg(short = 's', long = "style", global = true, default_value_t = Styles::default())]
    pub table_style: Styles,

    /// Set the log level for the daemon.
    #[arg(short, long, default_value_t = DEFAULT_LOGGER_LEVEL.into(), value_enum)]
    pub log_level: LogLevel,

    /// Whether to communicate with system swd instead of user-started swd.
    #[cfg(feature = "users")]
    #[arg(long = "system", global = true)]
    pub system_swd: bool
}

#[derive(Subcommand, Clone, Debug)]
pub enum Subcommands {
    /// Create and start a new stopwatch.
    #[command(visible_aliases = ["s", "new", "n"])]
    Start(Start),

    /// Get information about the specified stopwatches.
    #[command(visible_aliases = ["i", "get", "g"])]
    Info(Info),

    /// End the specified stopwatches, preventing them from starting again.
    #[command(visible_aliases = ["end", "e", "terminate", "term", "t"])]
    Stop(Stop),

    /// Start a new lap for the specified stopwatches.
    #[command(visible_aliases = ["l"])]
    Lap(Lap),

    /// Pause the current lap for each stopwatch.
    /// 
    /// Aliases: pause
    Pause(Pause),

    /// Continue the current lap for each stopwatch.
    /// 
    /// Aliases: play
    Play(Play),

    /// Delete a stopwatch from the daemon.
    #[command(visible_aliases = ["d", "del", "remove", "rm", "r"])]
    Delete(Delete)
}

#[derive(Args, Clone, Debug)]
pub struct Start {
    /// Name of the new stopwatch
    pub identifier: Option<String>
}

#[derive(Args, Clone, Debug)]
pub struct Info {
    /// List of stopwatches you want information about.
    /// Leave blank to query all stopwatches
    pub identifiers: Vec<String>
}

#[derive(Args, Clone, Debug)]
pub struct Stop {
    /// List of stopwatches to stop.
    /// Must specify more than 1 stopwatch
    pub identifiers: Vec<String>
}

#[derive(Args, Clone, Debug)]
pub struct Lap {
    /// List stopwatches to start a new lap for.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>
}

#[derive(Args, Clone, Debug)]
pub struct Pause {
    /// List stopwatches to pause.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>
}

#[derive(Args, Clone, Debug)]
pub struct Play {
    /// List of stopwatches to play.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>
}

#[derive(Args, Clone, Debug)]
pub struct Delete {
    /// List of stopwatches to delete.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>
}
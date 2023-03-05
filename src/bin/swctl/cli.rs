use clap::{Parser, Subcommand, Args};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Subcommands
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
    #[command(visible_aliases = ["d", "remove", "rm", "r"])]
    Delete(Delete)
}

#[derive(Args, Clone, Debug)]
pub struct Start {
    /// Name of the new stopwatch
    pub identifier: Option<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Info {
    /// List of stopwatches you want information about.
    /// Leave blank to query all stopwatches
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Stop {
    /// List of stopwatches to stop.
    /// Must specify more than 1 stopwatch
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Lap {
    /// List stopwatches to start a new lap for.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Pause {
    /// List stopwatches to pause.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Play {
    /// List of stopwatches to play.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}

#[derive(Args, Clone, Debug)]
pub struct Delete {
    /// List of stopwatches to delete.
    /// Must specify more than 1 stopwatch.
    pub identifiers: Vec<String>,

    /// Display detailed information
    #[arg(short, long)]
    pub verbose: bool
}
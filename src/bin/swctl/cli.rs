use clap::{Parser, Subcommand, Args};

/// Interact with the swd daemon that manages your stopwatches.
#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Subcommands
}

#[derive(Subcommand, Clone, Debug)]
pub enum Subcommands {
    /// Create and start a new stopwatch
    Start(Start),

    /// Get information about the specified stopwatches
    Info(Info),

    /// End the specified stopwatches, preventing them from starting again
    Stop(Stop),

    /// Start a new lap for the specified stopwatches
    Lap(Lap),

    /// Pause the current lap for each stopwatch
    Pause(Pause)
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
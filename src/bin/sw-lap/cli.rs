use clap::Parser;

/// Start a new lap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List of stopwatches to start a new lap for
    pub identifiers: Vec<String>,

    /// Set verbosity to get more detailed info
    #[arg(short, long)]
    pub verbose: bool
}
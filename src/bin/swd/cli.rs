use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set interval for checking for connections
    #[arg(short, long)]
    pub interval: Option<u64>
}
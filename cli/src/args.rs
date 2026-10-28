pub mod play;

use clap::{Parser, Subcommand};
use play::PlayArgs;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CliArgs {
    /// More diagnostic output on stderr (repeat for more)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Print the result as JSON on stdout
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Store,
}

#[derive(Subcommand, Debug)]
pub enum Store {
    /// Upload builds to Google Play
    Play(PlayArgs),
}

impl CliArgs {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

mod play;

use clap::{Parser, Subcommand};
use play::PlayArgs;

#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
pub struct CliArgs {

    #[command(subcommand)]
    pub command: Store,
}

#[derive(Subcommand, Debug)]
pub enum Store {
    PlayStore(PlayArgs),
}

impl CliArgs {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

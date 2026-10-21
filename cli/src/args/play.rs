use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct PlayArgs {
    #[command(subcommand)]
    pub action: PlayAction,
}

#[derive(Subcommand, Debug)]
pub enum PlayAction {
    Upload(UploadArgs),
}

#[derive(Args, Debug)]
pub struct UploadArgs;

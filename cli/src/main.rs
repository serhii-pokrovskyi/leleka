mod args;

use std::process::ExitCode;

use args::play::PlayAction;
use args::{CliArgs, Store};

/// The action parses, but isn't built yet. Nothing was sent.
const EXIT_NOT_IMPLEMENTED: u8 = 7;

fn main() -> ExitCode {
    let cli = CliArgs::parse_args();
    if cli.verbose > 0 {
        eprintln!("{cli:#?}");
    }

    match cli.command {
        Store::Play(play) => match play.action {
            PlayAction::Upload(_) => not_implemented("play upload"),
        },
    }
}

fn not_implemented(command: &str) -> ExitCode {
    eprintln!(
        "error: 'leleka {command}' is not implemented yet (leleka {})",
        env!("CARGO_PKG_VERSION")
    );
    ExitCode::from(EXIT_NOT_IMPLEMENTED)
}

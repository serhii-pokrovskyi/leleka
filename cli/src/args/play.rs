use std::path::PathBuf;

use clap::builder::NonEmptyStringValueParser;
use clap::{Args, Subcommand, ValueEnum};

#[derive(Args, Debug)]
pub struct PlayArgs {
    /// Google service account JSON key file
    #[arg(long, value_name = "PATH", global = true, help_heading = "Credentials")]
    pub key_file: Option<PathBuf>,

    /// Application ID, e.g. com.example.app (read from the bundle when omitted)
    #[arg(
        long,
        value_name = "NAME",
        global = true,
        value_parser = NonEmptyStringValueParser::new()
    )]
    pub package_name: Option<String>,

    #[command(subcommand)]
    pub action: PlayAction,
}

#[derive(Subcommand, Debug)]
pub enum PlayAction {
    /// Upload an Android App Bundle (.aab) and release it on a track
    Upload(UploadArgs),
}

#[derive(Args, Debug)]
pub struct UploadArgs {
    /// Android App Bundle to upload (.aab)
    #[arg(value_parser = aab_path)]
    pub file: PathBuf,

    /// Track to release on: internal, alpha, beta, production or a custom track name
    #[arg(long, value_parser = NonEmptyStringValueParser::new())]
    pub track: String,

    /// Status of the new release; use draft for an app that has never been published
    #[arg(
        long,
        value_name = "STATUS",
        value_enum,
        default_value_t = ReleaseStatus::Completed
    )]
    pub release_status: ReleaseStatus,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseStatus {
    Completed,
    Draft,
}

/// Accepts only file names ending in `.aab` (any case). The file itself isn't opened.
fn aab_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    let is_aab = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("aab"));
    if is_aab {
        Ok(path)
    } else {
        Err(String::from("expected an Android App Bundle (.aab)"))
    }
}

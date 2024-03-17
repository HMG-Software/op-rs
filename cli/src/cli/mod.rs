pub(crate) mod default;
pub(crate) mod parser;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(
    about = ABOUT,
    author,
    name = NAME,
    long_about = None,
    version,
)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[cfg(feature = "config")]
    #[arg(
        global = true,
        long,
        short = 'c',
        value_name = "CONFIG-FILE",
        env = "OPENPROJECT_CONFIG_PATH",
    )]
    pub config_path: Option<PathBuf>,

    #[arg(
        global = true,
        hide_default_value = true,
        long,
        short = None,
        value_name = "API-TOKEN",
        value_parser = parser::token,
    )]
    pub token: Option<String>,
}

// Add new subcommands here
#[derive(Subcommand)]
enum Commands {
    #[command(
        aliases = ["timesheet", "ts"],
        arg_required_else_help = true,
        subcommand,
    )]
    TimeEntry(crate::time_entry::command::TimeEntry),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

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
    /// Provide an OpenProject API Token or store it in
    /// `~/.op/token`
    #[arg(
        global = true,
        hide_default_value = true,
        long,
        short = None,
        value_name = "API-TOKEN",
    )]
    token: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

// Add new subcommands here
#[derive(Subcommand)]
enum Commands {
    #[command(
        aliases = ["timesheet", "ts"],
        arg_required_else_help = true,
        subcommand,
    )]
    TimeEntry(crate::timesheet::command::TimeEntry),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

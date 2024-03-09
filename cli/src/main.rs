use clap::{Parser, Subcommand};


const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = &format!(
    "{}\nPass `-h` for help",
    DESCRIPTION,
);


#[derive(Parser)]
#[command(
    about = ABOUT,
    author = AUTHOR,
    long_about = None,
    version = VERSION,
)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
    #[clap(long, short)]
    token: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Timesheet,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Get(value) => get_something(value),
        Commands::Set{key, value, is_true} => set_something(key, value, is_true),
    }
}


// https://docs.rs/clap/latest/clap/_faq/index.html#when-should-i-use-the-builder-vs-derive-apis

// https://docs.rs/clap_complete/latest/clap_complete/generator/fn.generate_to.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Bash.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Zsh.html

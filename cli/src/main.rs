mod cli;
mod time_entry;

use clap::Parser;
use cli::Cli;

#[cfg(feature = "config")]
use lib::config::Config;
#[cfg(feature = "config")]
use once_cell::sync::OnceCell;

#[cfg(feature = "config")]
pub static CONFIG: OnceCell<Config> = OnceCell::new();

fn main() {
    // get custom config path
    #[cfg(feature = "config")]
    {
        let config: Config = Config::load();
        CONFIG.set(config).unwrap();
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(command) => todo!("handle command"),
        None => println!("No command provided"),
    }
}

// https://docs.rs/clap/latest/clap/_faq/index.html#when-should-i-use-the-builder-vs-derive-apis

// https://docs.rs/clap_complete/latest/clap_complete/generator/fn.generate_to.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Bash.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Zsh.html

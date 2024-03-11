mod cli;
mod timesheet;

use clap::Parser;

use cli::Cli;

fn main() {
    let _cli = Cli::parse();
}

// https://docs.rs/clap/latest/clap/_faq/index.html#when-should-i-use-the-builder-vs-derive-apis

// https://docs.rs/clap_complete/latest/clap_complete/generator/fn.generate_to.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Bash.html
// https://docs.rs/clap_complete/latest/clap_complete/shells/struct.Zsh.html

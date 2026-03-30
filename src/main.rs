mod cli;
mod error;
mod git;
mod output;

use clap::Parser;
use colored::Colorize;

use cli::Cli;
use error::StandupError;

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        eprintln!("{}: {}", "error".red(), e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), StandupError> {
    let since_ts = git::parse_since(&args.since)?;
    let author = args.author.as_deref();

    let commits = git::get_commits(&args.repo, since_ts, author)?;

    output::print_commits(&args.repo, &commits, &args.since);

    Ok(())
}

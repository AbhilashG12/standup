mod cli;
mod commands;
mod config;
mod error;
mod git;
mod llm;
mod output;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Command, Format};
use error::StandupError;
use llm::RepoSummary;

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        eprintln!("{}: {}", "error".red(), e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), StandupError> {
    match args.command {
        Some(Command::Init) => return commands::init(),
        Some(Command::Add { path, name }) => return commands::add(path, name),
        Some(Command::List) => return commands::list(),
        Some(Command::Remove { name }) => return commands::remove(&name),
        Some(Command::SetKey { key }) => return commands::set_key(key),
        None => {}
    }

    run_report(args)
}

fn run_report(args: Cli) -> Result<(), StandupError> {
    let cfg = config::load()?;

    let author = args
        .author
        .as_deref()
        .or(cfg.settings.author.as_deref());

    let since = args
        .since
        .unwrap_or_else(|| cfg.settings.default_since.clone());

    let since_ts = git::parse_since(&since)?;
    let format = &args.format;
    let summarize = args.summarize;

    if let Some(repo_path) = args.repo {
        let commits = git::get_commits(&repo_path, since_ts, author)?;
        output::print_commits(&repo_path, &commits, &since, format);

        if summarize {
            run_summary(
                &[RepoSummary { name: &repo_path, commits: &commits }],
                &since,
                &cfg,
            )?;
        }

        return Ok(());
    }

    if cfg.repos.is_empty() {
        println!("No repos configured.");
        println!(
            "Run {} to add repos, or use {} for a single repo.",
            "`standup add <path>`".cyan(),
            "`standup --repo <path>`".cyan()
        );
        return Ok(());
    }

    let invalid = config::validate(&cfg);
    for missing in &invalid {
        eprintln!("{} Skipping missing repo: {}", "!".yellow(), missing);
    }

    let mut total_commits = 0;
    let mut active_repos = 0;
    let mut all_repo_data: Vec<(String, Vec<git::CommitInfo>)> = Vec::new();

    for repo in &cfg.repos {
        if !std::path::Path::new(&repo.path).exists() {
            continue;
        }

        let commits = git::get_commits(&repo.path, since_ts, author)?;

        if !commits.is_empty() {
            active_repos += 1;
            total_commits += commits.len();
        }

        output::print_commits(&repo.name, &commits, &since, format);
        all_repo_data.push((repo.name.clone(), commits));
    }

    if cfg.repos.len() > 1 {
        if !matches!(format, Format::Json) {
            println!(
                "{}",
                format!(
                    "─── {} repos · {} commits · since {} ───",
                    active_repos, total_commits, since
                )
                .dimmed()
            );
        }
    }

    if summarize {
        let summaries: Vec<RepoSummary> = all_repo_data
            .iter()
            .map(|(name, commits)| RepoSummary { name, commits })
            .collect();

        run_summary(&summaries, &since, &cfg)?;
    }

    Ok(())
}

fn run_summary(
    repos: &[RepoSummary],
    since: &str,
    cfg: &config::Config,
) -> Result<(), StandupError> {
    let api_key = cfg
        .settings
        .openai_api_key
        .as_deref()
        .ok_or_else(|| {
            StandupError::Llm(
                "No API key set. Run `standup set-key <key>` or add it during `standup init`."
                    .to_string(),
            )
        })?;

    let total: usize = repos.iter().map(|r| r.commits.len()).sum();

    if total == 0 {
        println!("\n{}", "No commits to summarize.".dimmed());
        return Ok(());
    }

    println!("\n{}", "Generating summary...".dimmed());

    let summary = llm::summarize(repos, since, api_key)?;

    println!();
    println!("{}", "── AI Summary ──────────────────────────".dimmed());
    println!("{}", summary);
    println!("{}", "────────────────────────────────────────".dimmed());

    Ok(())
}

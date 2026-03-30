use colored::Colorize;
use serde::Serialize;

use crate::cli::Format;
use crate::git::CommitInfo;

#[derive(Serialize)]
pub struct RepoReport<'a> {
    pub repo: &'a str,
    pub since: &'a str,
    pub commit_count: usize,
    pub commits: Vec<CommitJson<'a>>,
}

#[derive(Serialize)]
pub struct CommitJson<'a> {
    pub hash: &'a str,
    pub message: &'a str,
    pub author: &'a str,
    pub timestamp: String,
}

pub fn print_commits(repo: &str, commits: &[CommitInfo], since: &str, format: &Format) {
    match format {
        Format::Plain => print_plain(repo, commits, since),
        Format::Markdown => print_markdown(repo, commits, since),
        Format::Json => print_json(repo, commits, since),
    }
}

fn print_plain(repo: &str, commits: &[CommitInfo], since: &str) {
    if commits.is_empty() {
        println!("{}", format!("{} — no commits since {}", repo, since).dimmed());
        return;
    }

    println!("{}", repo.cyan().bold());

    for commit in commits {
        let time_str = commit.timestamp.format("%H:%M").to_string();
        println!(
            "  {} {} {}",
            commit.hash.yellow(),
            commit.message,
            format!("({})", time_str).dimmed(),
        );
    }

    let noun = if commits.len() == 1 { "commit" } else { "commits" };
    println!("{}", format!("  {} {} since {}", commits.len(), noun, since).dimmed());
    println!();
}

fn print_markdown(repo: &str, commits: &[CommitInfo], since: &str) {
    println!("## {}\n", repo);

    if commits.is_empty() {
        println!("_No commits since {}_\n", since);
        return;
    }

    for commit in commits {
        let time_str = commit.timestamp.format("%Y-%m-%d %H:%M").to_string();
        println!(
            "- `{}` {} _{}_",
            commit.hash, commit.message, time_str
        );
    }

    let noun = if commits.len() == 1 { "commit" } else { "commits" };
    println!("\n_{} {} since {}_\n", commits.len(), noun, since);
}

fn print_json(repo: &str, commits: &[CommitInfo], since: &str) {
    let report = RepoReport {
        repo,
        since,
        commit_count: commits.len(),
        commits: commits
            .iter()
            .map(|c| CommitJson {
                hash: &c.hash,
                message: &c.message,
                author: &c.author,
                timestamp: c.timestamp.format("%Y-%m-%dT%H:%M:%S").to_string(),
            })
            .collect(),
    };

    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

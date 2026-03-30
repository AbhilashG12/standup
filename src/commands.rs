use std::io::{self, Write};
use std::path::Path;

use colored::Colorize;

use crate::config::{self, Config, RepoEntry, Settings};
use crate::error::StandupError;

pub fn init() -> Result<(), StandupError> {
    println!("{}", "Setting up standup...".bold());
    println!();

    let path = config::config_path()?;
    if path.exists() {
        println!("{} Config already exists at {}", "✓".green(), path.display());
        println!("Use {} to add repos.", "`standup add <path>`".cyan());
        return Ok(());
    }

    print!("Your name or email for filtering (press Enter to skip): ");
    io::stdout().flush().unwrap();
    let mut author_input = String::new();
    io::stdin().read_line(&mut author_input).unwrap();
    let author_input = author_input.trim().to_string();
    let author = if author_input.is_empty() { None } else { Some(author_input) };

    print!("Default time range [yesterday]: ");
    io::stdout().flush().unwrap();
    let mut since_input = String::new();
    io::stdin().read_line(&mut since_input).unwrap();
    let since_input = since_input.trim().to_string();
    let default_since = if since_input.is_empty() { "yesterday".to_string() } else { since_input };

    print!("OpenAI API key for --summarize (press Enter to skip): ");
    io::stdout().flush().unwrap();
    let mut key_input = String::new();
    io::stdin().read_line(&mut key_input).unwrap();
    let key_input = key_input.trim().to_string();
    let openai_api_key = if key_input.is_empty() { None } else { Some(key_input) };

    let config = Config {
        settings: Settings { author, default_since, openai_api_key },
        repos: Vec::new(),
    };

    let saved_path = config::save(&config)?;

    println!();
    println!("{} Config created at {}", "✓".green(), saved_path.display());
    println!("Now add repos with {}", "`standup add <path>`".cyan());

    Ok(())
}

pub fn add(path: String, name: Option<String>) -> Result<(), StandupError> {
    let abs_path = Path::new(&path)
        .canonicalize()
        .map_err(|_| StandupError::NoRepoFound(path.clone()))?;

    let abs_path_str = abs_path.to_string_lossy().to_string();

    git2::Repository::open(&abs_path)
        .map_err(|_| StandupError::NoRepoFound(abs_path_str.clone()))?;

    let repo_name = name.unwrap_or_else(|| {
        abs_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| abs_path_str.clone())
    });

    let mut config = config::load()?;

    if config.repos.iter().any(|r| r.path == abs_path_str) {
        println!("{} Repo at {} is already in your config.", "!".yellow(), abs_path_str);
        return Ok(());
    }

    config.repos.push(RepoEntry { name: repo_name.clone(), path: abs_path_str });
    config::save(&config)?;
    println!("{} Added '{}'", "✓".green(), repo_name.cyan());

    Ok(())
}

pub fn list() -> Result<(), StandupError> {
    let config = config::load()?;

    if config.repos.is_empty() {
        println!("No repos configured yet.");
        println!("Run {} to add one.", "`standup add <path>`".cyan());
        return Ok(());
    }

    println!("{}", "Configured repos:".bold());
    println!();

    for repo in &config.repos {
        let status = if Path::new(&repo.path).exists() { "✓".green() } else { "✗".red() };
        println!("  {} {}  {}", status, repo.name.cyan(), repo.path.dimmed());
    }

    println!();
    println!("{} Default since: {}", "→".dimmed(), config.settings.default_since.yellow());

    if let Some(author) = &config.settings.author {
        println!("{} Default author: {}", "→".dimmed(), author.yellow());
    }

    let key_status = if config.settings.openai_api_key.is_some() {
        "set".green()
    } else {
        "not set".dimmed()
    };
    println!("{} OpenAI API key: {}", "→".dimmed(), key_status);

    Ok(())
}

pub fn remove(name: &str) -> Result<(), StandupError> {
    let mut config = config::load()?;
    let before = config.repos.len();
    config.repos.retain(|r| r.name != name);

    if before == config.repos.len() {
        println!(
            "{} No repo named '{}' found. Use {} to see names.",
            "!".yellow(), name, "`standup list`".cyan()
        );
        return Ok(());
    }

    config::save(&config)?;
    println!("{} Removed '{}'", "✓".green(), name.cyan());
    Ok(())
}

pub fn set_key(key: String) -> Result<(), StandupError> {
    let mut config = config::load()?;
    config.settings.openai_api_key = Some(key);
    config::save(&config)?;
    println!("{} OpenAI API key saved.", "✓".green());
    Ok(())
}

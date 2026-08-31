use reqwest::blocking::Client;
use serde_json::{json, Value};

use crate::error::StandupError;
use crate::git::CommitInfo;

pub struct RepoSummary<'a> {
    pub name: &'a str,
    pub commits: &'a [CommitInfo],
}

pub fn summarize(
    repos: &[RepoSummary],
    since: &str,
    api_key: &str,
) -> Result<String, StandupError> {
    let commit_text = build_prompt_text(repos, since);

    let prompt = format!(
        "You are a helpful assistant that writes concise standup summaries for developers.\n\
        Below are git commits from the past {}. Write a short, natural standup summary \
        (2-5 sentences) grouping related work together. Use plain language, no bullet points, \
        no markdown. Sound like a developer talking to their team.\n\n\
        Commits:\n{}",
        since, commit_text
    );

    let client = Client::new();

    let body = json!({
        "model": "openai/gpt-oss-120b",
        "messages": [
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 300,
        "temperature": 0.7
    });

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| StandupError::Llm(format!("Request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().unwrap_or_default();
        return Err(StandupError::Llm(format!(
            "API returned {}: {}",
            status, text
        )));
    }

    let json: Value = response
        .json()
        .map_err(|e| StandupError::Llm(format!("Failed to parse response: {}", e)))?;

    let summary = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("(no summary returned)")
        .trim()
        .to_string();

    Ok(summary)
}

fn build_prompt_text(repos: &[RepoSummary], since: &str) -> String {
    let mut lines = Vec::new();

    for repo in repos {
        if repo.commits.is_empty() {
            continue;
        }

        lines.push(format!("[{}] since {}:", repo.name, since));

        for commit in repo.commits {
            lines.push(format!("  - {}", commit.message));
        }
    }

    lines.join("\n")
}

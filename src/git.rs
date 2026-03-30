use chrono::{DateTime, Local, TimeZone};
use git2::{Repository, Sort};

use crate::error::StandupError;

#[derive(Debug)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Local>,
}

pub fn parse_since(since: &str) -> Result<i64, StandupError> {
    let now = Local::now();

    let dt = match since {
        "yesterday" => {
            let yesterday = now - chrono::Duration::days(1); 
            let naive = yesterday
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| StandupError::InvalidDate(since.to_string()))?;
            Local
                .from_local_datetime(&naive) 
                .single()
                .ok_or_else(|| StandupError::InvalidDate(since.to_string()))?
        }
        "today" => {
            let naive = now
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| StandupError::InvalidDate(since.to_string()))?;
            Local
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| StandupError::InvalidDate(since.to_string()))?
        }
        s if s.ends_with('d') => {
            let days: i64 = s
                .trim_end_matches('d')
                .parse()
                .map_err(|_| StandupError::InvalidDate(s.to_string()))?;
            now - chrono::Duration::days(days) 
        }
        s => {
            let naive_date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") 
                .map_err(|_| StandupError::InvalidDate(s.to_string()))?;
            let naive_dt = naive_date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| StandupError::InvalidDate(s.to_string()))?;
            Local
                .from_local_datetime(&naive_dt)
                .single()
                .ok_or_else(|| StandupError::InvalidDate(s.to_string()))?
        }
    };

    Ok(dt.timestamp())
}

pub fn get_commits(
    repo_path: &str,
    since_ts: i64,
    author_filter: Option<&str>,
) -> Result<Vec<CommitInfo>, StandupError> {
    let repo = Repository::open(repo_path)
        .map_err(|_| StandupError::NoRepoFound(repo_path.to_string()))?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_time = commit.time().seconds();

        if commit_time < since_ts {
            break;
        }

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("").to_string();

        if let Some(filter) = author_filter {
            let filter_lower = filter.to_lowercase();
            let name_match = author_name.to_lowercase().contains(&filter_lower);
            let email_match = author_email.to_lowercase().contains(&filter_lower);
            if !name_match && !email_match {
                continue;
            }
        }

        let message = commit
            .message()
            .unwrap_or("")
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        let timestamp = Local
            .timestamp_opt(commit_time, 0)
            .single()
            .unwrap_or_else(Local::now);

        commits.push(CommitInfo {
            hash: format!("{:.7}", oid),
            message,
            author: author_name,
            timestamp,
        });
    }

    Ok(commits)
}

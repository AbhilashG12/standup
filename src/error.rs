use std::fmt;

#[derive(Debug)]
pub enum StandupError {
    Git(git2::Error),
    InvalidDate(String),
    NoRepoFound(String),
}

impl fmt::Display for StandupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StandupError::Git(e) => write!(f, "Git error: {}", e),
            StandupError::InvalidDate(s) => write!(f, "Invalid date: {}", s),
            StandupError::NoRepoFound(path) => {
                write!(f, "No git repository found at: {}", path)
            }
        }
    }
}

impl From<git2::Error> for StandupError {
    fn from(e: git2::Error) -> Self {
        StandupError::Git(e)
    }
}

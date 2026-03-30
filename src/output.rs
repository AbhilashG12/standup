use colored::Colorize;
use crate::git::CommitInfo;


pub fn print_commits(repo_path:&str, commits : &[CommitInfo], since : &str) {

    if commits.is_empty(){
        println!("{} - no commit since {}" , repo_path.dimmed(), since.dimmed());
        return;

    }

    println!("{}" , repo_path.cyan().bold());

    for commit in commits {
        let time_str = commit.timestamp.format("%H-%M").to_string();
        println!("{} {} {}" , commit.hash.yellow(), commit.message, format!("({})",time_str).dimmed());

    }

    let count = commits.len();
    let noun = if count == 1 {"commit"} else {"commits"};
    println!("{}", format!("{} {} since {} ", count, noun, since).dimmed());
    println!();

}

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "standup", version, about = "Summarize your git commits for standups")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long)]
    pub since: Option<String>,

    #[arg(short, long)]
    pub author: Option<String>,

    #[arg(short, long)]
    pub repo: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Format::Plain)]
    pub format: Format,

    #[arg(long)]
    pub summarize: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Format {
    Plain,
    Markdown,
    Json,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Add {
        path: String,
        #[arg(short, long)]
        name: Option<String>,
    },
    List,
    Remove {
        name: String,
    },
    SetKey {
        key: String,
    },
}

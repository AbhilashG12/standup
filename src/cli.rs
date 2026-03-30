use clap::Parser;

#[derive(Parser,Debug)]
#[command(name="standup", version, about="Summarize your git commits for your next standup")]
pub struct Cli {

    #[arg(short,long,default_value = ".")]
    pub repo : String,

    #[arg(short,long,default_value = "yesterday")]
    pub since : String,

    #[arg(short,long)]
    pub author : Option<String>,


}


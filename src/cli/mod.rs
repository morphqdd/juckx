use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "git-ai-commit", version, about = "AI-powered git commit utility")]
pub struct Cli {
    /// Don't push after commit
    #[arg(long)]
    pub no_push: bool,

    /// Only print the commit message, don't commit or push
    #[arg(long)]
    pub dry_run: bool,

    /// Language for commit message: en or ru
    #[arg(long, default_value = "en")]
    pub lang: String,

    /// API key for use
    #[arg(long)]
    pub with_api: Option<String>
}
use std::{env, fs};
use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use crate::cli::Cli;

mod cli;
mod git;
mod gemini;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(api) = cli.with_api {
        let path = env::home_dir().unwrap().join(".config/juckx");
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        } 
        OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(format!("{}/.env", path.display())).await?
            .write_all(format!("GEMINI_API_KEY={api}").as_bytes()).await?;
    }

    if !git::is_git_repo().await {
        eprintln!("❌ This is not a git repository.");
        std::process::exit(1);
    }

    let diff_or_files = if git::has_commits().await {
        git::get_git_diff().await?
    } else {
        git::get_untracked_files().await?
    };

    if diff_or_files.trim().is_empty() {
        println!("✅ No changes to commit.");
        return Ok(());
    }

    let prompt = gemini::build_prompt(&diff_or_files, &cli.lang);
    let commit_msg = gemini::get_commit_message(&prompt).await?;

    println!("💬 Commit message:\n{commit_msg}");

    if cli.dry_run {
        println!("🧪 Dry run enabled, exiting.");
        return Ok(());
    }

    git::stage_all().await?;
    git::commit(&commit_msg).await?;
    if !cli.no_push {
        git::push().await?;
    }

    Ok(())
}

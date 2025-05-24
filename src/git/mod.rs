use std::process::Stdio;
use tokio::process::Command;

/// Checks if the current folder is a git repository
pub async fn is_git_repo() -> bool {
    Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Checks if the repository has any commits (HEAD exists)
pub async fn has_commits() -> bool {
    Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Gets the diff of staged changes
pub async fn get_git_diff() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(&["diff", "--cached"])
        .output()
        .await?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Gets the list of untracked files (for initial commit)
pub async fn get_untracked_files() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .await?;

    let status = String::from_utf8_lossy(&output.stdout);

    let files = status
        .lines()
        .filter(|line| line.starts_with("??"))
        .map(|line| format!("New file: {}", &line[3..]))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(files)
}

/// Stages all changes (git add .)
pub async fn stage_all() -> anyhow::Result<()> {
    let status = Command::new("git")
        .args(&["add", "."])
        .status()
        .await?;
    if !status.success() {
        anyhow::bail!("Failed to stage files");
    }
    Ok(())
}

/// Creates a commit with the given message
pub async fn commit(message: &str) -> anyhow::Result<()> {
    let status = Command::new("git")
        .args(&["commit", "-m", message])
        .status()
        .await?;
    if !status.success() {
        anyhow::bail!("Failed to commit");
    }
    Ok(())
}

/// Pushes commits to the remote repository
pub async fn push() -> anyhow::Result<()> {
    let status = Command::new("git")
        .args(&["push"])
        .status()
        .await?;
    if !status.success() {
        anyhow::bail!("Failed to push");
    }
    Ok(())
}

use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Checks if the current directory is inside a Git work tree.
///
/// # Returns
///
/// A Result containing a boolean indicating if inside a work tree or an error.
pub fn is_inside_work_tree() -> Result<bool> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(exit) => Ok(exit.success()),
        Err(_) => Ok(false),
    }
}

/// Determines if the given diff represents a binary file.
pub fn is_binary_diff(diff: &str) -> bool {
    diff.contains("Binary files")
        || diff.contains("GIT binary patch")
        || diff.contains("[Binary file changed]")
}

/// Executes a git command and returns the output as a string
///
/// # Arguments
///
/// * `args` - The arguments to pass to git
///
/// # Returns
///
/// A Result containing the output as a String or an error.
pub fn run_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout =
        String::from_utf8(output.stdout).context("Invalid UTF-8 output from git command")?;

    Ok(stdout.trim().to_string())
}

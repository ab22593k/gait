use super::changelog::ChangelogGenerator;
use super::releasenotes::ReleaseNotesGenerator;
use crate::common::{CommonParams, DetailLevel};
use crate::config::Config;
use crate::git::GitRepo;
use crate::ui;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

/// Handles the changelog generation command.
///
/// This function orchestrates the process of generating a changelog based on the provided
/// parameters. It sets up the necessary environment, creates a `GitRepo` instance,
/// and delegates the actual generation to the `ChangelogGenerator`.
///
/// # Arguments
///
/// * `common` - Common parameters for the command, including configuration overrides.
/// * `from` - The starting point (commit or tag) for the changelog.
/// * `to` - The ending point for the changelog. Defaults to "HEAD" if not provided.
/// * `repository_url` - Optional URL of the remote repository to use.
/// * `update_file` - Whether to update the changelog file.
/// * `changelog_path` - Optional path to the changelog file.
/// * `version_name` - Optional version name to use instead of extracting from Git refs.
///
/// # Returns
///
/// Returns a Result indicating success or containing an error if the operation failed.
pub async fn handle_changelog_command(
    common: CommonParams,
    from: String,
    to: Option<String>,
    repository_url: Option<String>,
    update_file: bool,
    changelog_path: Option<String>,
    version_name: Option<String>,
) -> Result<()> {
    // Load and apply configuration
    let mut config = Config::load()?;
    common.apply_to_config(&mut config)?;

    // Create a spinner to indicate progress
    let spinner = ui::create_spinner("Generating changelog...");

    // Ensure we're in a git repository
    if let Err(e) = config.check_environment() {
        ui::print_error(&format!("Error: {e}"));
        ui::print_info("\nPlease ensure the following:");
        ui::print_info("1. Git is installed and accessible from the command line.");
        ui::print_info(
            "2. You are running this command from within a Git repository or provide a repository URL with --repo.",
        );
        ui::print_info("3. You have set up your configuration using 'git-iris config'.");
        return Err(e);
    }

    // Use the repository URL from command line or common params
    let repo_url = repository_url.or(common.repository_url);

    // Create a GitRepo instance based on the URL or current directory
    let git_repo = if let Some(url) = repo_url {
        Arc::new(GitRepo::clone_remote_repository(&url).context("Failed to clone repository")?)
    } else {
        let repo_path = env::current_dir()?;
        Arc::new(GitRepo::new(&repo_path).context("Failed to create GitRepo")?)
    };

    // Keep a clone of the Arc for updating the changelog later if needed
    let git_repo_for_update = Arc::clone(&git_repo);

    // Set the default 'to' reference if not provided
    let to = to.unwrap_or_else(|| "HEAD".to_string());

    // Parse the detail level for the changelog
    let detail_level = DetailLevel::from_str(&common.detail_level)?;

    // Generate the changelog
    let changelog =
        ChangelogGenerator::generate(git_repo, &from, &to, &config, detail_level).await?;

    // Clear the spinner and display the result
    spinner.finish_and_clear();

    // Output the changelog with decorative borders
    ui::print_bordered_content(&changelog);

    // Update the changelog file if requested
    if update_file {
        let path = changelog_path.unwrap_or_else(|| "CHANGELOG.md".to_string());
        let update_spinner = ui::create_spinner(&format!("Updating changelog file at {path}..."));

        match ChangelogGenerator::update_changelog_file(
            &changelog,
            &path,
            &git_repo_for_update,
            &to,
            version_name,
        ) {
            Ok(()) => {
                update_spinner.finish_and_clear();
                ui::print_success(&format!(
                    "✨ Changelog successfully updated at {}",
                    path.bright_green()
                ));
            }
            Err(e) => {
                update_spinner.finish_and_clear();
                ui::print_error(&format!("Failed to update changelog file: {e}"));
            }
        }
    }

    Ok(())
}

/// Handles the release notes generation command.
///
/// This function orchestrates the process of generating release notes based on the provided
/// parameters. It sets up the necessary environment, creates a `GitRepo` instance,
/// and delegates the actual generation to the `ReleaseNotesGenerator`.
///
/// # Arguments
///
/// * `common` - Common parameters for the command, including configuration overrides.
/// * `from` - The starting point (commit or tag) for the release notes.
/// * `to` - The ending point for the release notes. Defaults to "HEAD" if not provided.
/// * `repository_url` - Optional URL of the remote repository to use.
/// * `version_name` - Optional version name to use instead of extracting from Git refs.
///
/// # Returns
///
/// Returns a Result indicating success or containing an error if the operation failed.
pub async fn handle_release_notes_command(
    common: CommonParams,
    from: String,
    to: Option<String>,
    repository_url: Option<String>,
    version_name: Option<String>,
) -> Result<()> {
    // Load and apply configuration
    let mut config = Config::load()?;
    common.apply_to_config(&mut config)?;

    // Create a spinner to indicate progress
    let spinner = ui::create_spinner("Generating release notes...");

    // Check environment prerequisites
    if let Err(e) = config.check_environment() {
        ui::print_error(&format!("Error: {e}"));
        ui::print_info("\nPlease ensure the following:");
        ui::print_info("1. Git is installed and accessible from the command line.");
        ui::print_info(
            "2. You are running this command from within a Git repository or provide a repository URL with --repo.",
        );
        ui::print_info("3. You have set up your configuration using 'git-iris config'.");
        return Err(e);
    }

    // Use the repository URL from command line or common params
    let repo_url = repository_url.or(common.repository_url);

    // Create a GitRepo instance based on the URL or current directory
    let git_repo = if let Some(url) = repo_url {
        Arc::new(GitRepo::clone_remote_repository(&url).context("Failed to clone repository")?)
    } else {
        let repo_path = env::current_dir()?;
        Arc::new(GitRepo::new(&repo_path).context("Failed to create GitRepo")?)
    };

    // Set the default 'to' reference if not provided
    let to = to.unwrap_or_else(|| "HEAD".to_string());

    // Parse the detail level for the release notes
    let detail_level = DetailLevel::from_str(&common.detail_level)?;

    // Generate the release notes
    let release_notes =
        ReleaseNotesGenerator::generate(git_repo, &from, &to, &config, detail_level, version_name)
            .await?;

    // Clear the spinner and display the result
    spinner.finish_and_clear();

    // Output the release notes with decorative borders
    ui::print_bordered_content(&release_notes);

    Ok(())
}

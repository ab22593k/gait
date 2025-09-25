use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use textwrap;

/// Model for commit message generation results
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct GeneratedMessage {
    /// Optional emoji for the commit message
    pub emoji: Option<String>,
    /// Commit message title/subject line
    pub title: String,
    /// Detailed commit message body
    pub message: String,
}

/// Model for pull request description generation results
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct GeneratedPullRequest {
    /// Optional emoji for the pull request title
    pub emoji: Option<String>,
    /// Pull request title
    pub title: String,
    /// Brief summary of the changes
    pub summary: String,
    /// Detailed description of what was changed and why
    pub description: String,
    /// List of commit messages included in this PR
    pub commits: Vec<String>,
    /// Breaking changes if any
    pub breaking_changes: Vec<String>,
    /// Testing instructions for reviewers
    pub testing_notes: Option<String>,
    /// Additional notes or context
    pub notes: Option<String>,
}

/// Formats a commit message from a `GeneratedMessage`
pub fn format_commit_message(response: &GeneratedMessage) -> String {
    let mut message = String::new();

    if let Some(emoji) = &response.emoji {
        write!(&mut message, "{emoji} ").expect("write to string should not fail");
    }

    message.push_str(&response.title);
    message.push_str("\n\n");

    let wrapped_message = textwrap::wrap(&response.message, 78);
    for line in wrapped_message {
        message.push_str(&line);
        message.push('\n');
    }

    message
}

/// Formats a pull request description from a `GeneratedPullRequest`
pub fn format_pull_request(response: &GeneratedPullRequest) -> String {
    let mut message = String::new();

    // Title with optional emoji
    if let Some(emoji) = &response.emoji {
        writeln!(&mut message, "# {emoji} {}", response.title)
            .expect("write to string should not fail");
    } else {
        writeln!(&mut message, "# {}", response.title).expect("write to string should not fail");
    }
    message.push('\n');

    // Summary - no word wrapping for web UI display
    writeln!(&mut message, "## Summary").expect("write to string should not fail");
    writeln!(&mut message, "{}", response.summary).expect("write to string should not fail");
    message.push('\n');

    // Description - no word wrapping for web UI display
    writeln!(&mut message, "## Description").expect("write to string should not fail");
    writeln!(&mut message, "{}", response.description).expect("write to string should not fail");
    message.push('\n');

    // Commits
    if !response.commits.is_empty() {
        writeln!(&mut message, "## Commits").expect("write to string should not fail");
        for commit in &response.commits {
            writeln!(&mut message, "- {commit}").expect("write to string should not fail");
        }
        message.push('\n');
    }

    // Breaking changes
    if !response.breaking_changes.is_empty() {
        writeln!(&mut message, "## Breaking Changes").expect("write to string should not fail");
        for change in &response.breaking_changes {
            writeln!(&mut message, "- {change}").expect("write to string should not fail");
        }
        message.push('\n');
    }

    // Testing notes - no word wrapping for web UI display
    if let Some(testing) = &response.testing_notes {
        writeln!(&mut message, "## Testing").expect("write to string should not fail");
        writeln!(&mut message, "{testing}").expect("write to string should not fail");
        message.push('\n');
    }

    // Additional notes - no word wrapping for web UI display
    if let Some(notes) = &response.notes {
        writeln!(&mut message, "## Notes").expect("write to string should not fail");
        writeln!(&mut message, "{notes}").expect("write to string should not fail");
    }

    message
}

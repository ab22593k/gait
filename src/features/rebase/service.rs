//! Rebase service implementation

use super::types::{RebaseAction, RebaseAnalysis, RebaseCommit, RebaseResult};
use crate::config::Config;
use crate::core::llm;
use crate::git::GitRepo;
use crate::ui;

use anyhow::Result;
use git2::Status;
use std::sync::Arc;
use log::debug;

/// Service for handling AI-assisted rebase operations
pub struct RebaseService {
    config: Config,
    repo: Arc<GitRepo>,
    test_mode: bool,
}

impl RebaseService {
    /// Create a new RebaseService instance
    pub fn new(config: Config, repo: GitRepo) -> Result<Self> {
        Ok(Self {
            config,
            repo: Arc::new(repo),
            test_mode: false,
        })
    }

    /// Create a new RebaseService instance in test mode (skips AI calls)
    pub fn new_test(config: Config, repo: GitRepo) -> Result<Self> {
        Ok(Self {
            config,
            repo: Arc::new(repo),
            test_mode: true,
        })
    }

    /// Check if the environment is suitable for rebase operations
    pub fn check_environment(&self) -> Result<()> {
        // Check if we're in a git repository
        if self.repo.is_remote() {
            return Err(anyhow::anyhow!(
                "Cannot perform rebase operations on remote repositories"
            ));
        }

        // Check if there are any uncommitted changes
        let repo_binding = self.repo.open_repo()?;
        let status = repo_binding.statuses(None)?;
        if status.iter().any(|s| s.status() != Status::CURRENT) {
            ui::print_warning(
                "You have uncommitted changes. Consider committing or stashing them before rebasing.",
            );
        }

        Ok(())
    }

    /// Analyze commits that will be rebased and suggest actions
    pub async fn analyze_commits_for_rebase(
        &self,
        upstream: &str,
        branch: Option<&str>,
    ) -> Result<RebaseAnalysis> {
        debug!(
            "Analyzing commits for rebase: upstream={}, branch={:?}",
            upstream, branch
        );

        let repo = self.repo.open_repo()?;

        // Determine which branch to rebase
        let default_branch = self.repo.get_current_branch().unwrap_or("HEAD".to_string());
        let branch_name = branch.unwrap_or(default_branch.as_str());

        // Find the merge base between upstream and branch
        let upstream_commit = repo.revparse_single(upstream)?.peel_to_commit()?;
        let branch_commit = repo.revparse_single(&branch_name)?.peel_to_commit()?;

        let merge_base_oid = repo.merge_base(upstream_commit.id(), branch_commit.id())?;
        let merge_base = repo.find_commit(merge_base_oid)?;

        // Get all commits from merge_base to branch_commit
        let mut revwalk = repo.revwalk()?;
        revwalk.push(branch_commit.id())?;
        revwalk.hide(merge_base.id())?;

        let mut commits = Vec::new();
        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = repo.find_commit(oid)?;

            let rebase_commit = RebaseCommit {
                hash: format!("{:?}", oid).chars().take(7).collect(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                date: format!("{}", commit.time().seconds()), // TODO: Format properly
                suggested_action: RebaseAction::Pick,         // Default to pick, will be analyzed
                confidence: 0.5,
                reasoning: "Default action".to_string(),
            };

            commits.push(rebase_commit);
        }

        commits.reverse();

        println!("!!!!!!!!!!!!!!!!!!");
        // Reverse to get chronological order (oldest first)
        // Analyze commits with AI to suggest actions
        let analyzed_commits = self.analyze_commit_actions(commits).await?;

        let analysis = RebaseAnalysis {
            commits: analyzed_commits,
            upstream: upstream.to_string(),
            branch: branch_name.to_string(),
            suggested_operations: 0, // TODO: Calculate based on non-pick actions
        };

        Ok(analysis)
    }

    /// Analyze commits and suggest rebase actions using AI
    async fn analyze_commit_actions(
        &self,
        commits: Vec<RebaseCommit>,
    ) -> Result<Vec<RebaseCommit>> {
        if commits.is_empty() {
            return Ok(commits);
        }

        if self.test_mode {
            debug!(
                "Test mode: using fallback analysis for {} commits",
                commits.len()
            );
            return self.fallback_analysis(commits);
        }

        debug!(
            "Analyzing {} commits with AI for rebase actions",
            commits.len()
        );

        // Create system prompt for rebase analysis
        let system_prompt = r#"You are an expert Git rebase assistant. Your task is to analyze a series of commits and suggest appropriate rebase actions for each one.

Available actions:
- pick: Keep the commit as-is
- reword: Change only the commit message
- edit: Stop for manual editing of both message and content
- squash: Combine this commit with the previous one, keeping both messages
- fixup: Combine this commit with the previous one, keeping only the previous message
- drop: Remove this commit entirely

Guidelines:
- Fix commits should generally be picked unless they're trivial
- WIP/Work-in-progress commits should be squashed or fixup'd
- Typos in commit messages should be reworded
- Duplicate functionality commits should be squashed
- Test commits should be dropped unless they're significant
- Refactor commits that don't change behavior can be squashed
- Breaking changes should be picked with clear messages

Return a JSON array of objects with this structure:
[
  {
    "action": "pick|reword|edit|squash|fixup|drop",
    "confidence": 0.0-1.0,
    "reasoning": "Brief explanation of why this action was chosen"
  },
  ...
]

The array should have exactly one object per input commit, in the same order."#;

        // Create user prompt with commit information
        let mut user_prompt =
            "Please analyze these commits and suggest rebase actions:\n\n".to_string();

        for (i, commit) in commits.iter().enumerate() {
            user_prompt.push_str(&format!("Commit {}: {}\n", i + 1, commit.message.trim()));
            user_prompt.push_str(&format!("Author: {}\n", commit.author));
            user_prompt.push_str(&format!("Hash: {}\n\n", commit.hash));
        }

        user_prompt.push_str("Respond with only the JSON array, no additional text.");

        println!("!!!!!!!!!!!!!!!!!!");
        // Call LLM
        let response: String = llm::get_message(
            &self.config,
            &self.config.default_provider,
            system_prompt,
            &user_prompt,
        )
        .await?;

        // Parse the JSON response
        self.parse_ai_response(&response, commits)
    }

    /// Parse AI response and apply suggestions to commits
    fn parse_ai_response(
        &self,
        response: &str,
        mut commits: Vec<RebaseCommit>,
    ) -> Result<Vec<RebaseCommit>> {
        // Try to parse as JSON array
        match serde_json::from_str::<Vec<serde_json::Value>>(response.trim()) {
            Ok(suggestions) => {
                if suggestions.len() != commits.len() {
                    debug!(
                        "AI returned {} suggestions but we have {} commits, using fallback",
                        suggestions.len(),
                        commits.len()
                    );
                    return self.fallback_analysis(commits);
                }

                for (i, suggestion) in suggestions.iter().enumerate() {
                    if let Some(commit) = commits.get_mut(i) {
                        if let (Some(action_str), Some(confidence), Some(reasoning)) = (
                            suggestion.get("action").and_then(|v| v.as_str()),
                            suggestion.get("confidence").and_then(|v| v.as_f64()),
                            suggestion.get("reasoning").and_then(|v| v.as_str()),
                        ) {
                            commit.suggested_action = match action_str {
                                "pick" => RebaseAction::Pick,
                                "reword" => RebaseAction::Reword,
                                "edit" => RebaseAction::Edit,
                                "squash" => RebaseAction::Squash,
                                "fixup" => RebaseAction::Fixup,
                                "drop" => RebaseAction::Drop,
                                _ => {
                                    debug!(
                                        "Unknown action '{}' from AI, defaulting to pick",
                                        action_str
                                    );
                                    RebaseAction::Pick
                                }
                            };
                            commit.confidence = confidence as f32;
                            commit.reasoning = reasoning.to_string();
                        } else {
                            debug!("Invalid suggestion format for commit {}, using fallback", i);
                            self.apply_fallback_action(commit);
                        }
                    }
                }
                Ok(commits)
            }
            Err(e) => {
                debug!(
                    "Failed to parse AI response as JSON: {}, using fallback analysis",
                    e
                );
                self.fallback_analysis(commits)
            }
        }
    }

    /// Fallback analysis using simple heuristics
    fn fallback_analysis(&self, commits: Vec<RebaseCommit>) -> Result<Vec<RebaseCommit>> {
        Ok(commits
            .into_iter()
            .map(|mut commit| {
                self.apply_fallback_action(&mut commit);
                commit
            })
            .collect())
    }

    /// Apply fallback action based on simple heuristics
    fn apply_fallback_action(&self, commit: &mut RebaseCommit) {
        let msg_lower = commit.message.to_lowercase();
        if msg_lower.contains("fix") || msg_lower.contains("refactor") {
            commit.suggested_action = RebaseAction::Pick;
            commit.reasoning = "Fix/refactor commits are typically kept as-is".to_string();
            commit.confidence = 0.8;
        } else if msg_lower.contains("wip") || msg_lower.contains("work in progress") {
            commit.suggested_action = RebaseAction::Squash;
            commit.reasoning = "WIP commits should be squashed".to_string();
            commit.confidence = 0.9;
        } else if msg_lower.contains("test") && msg_lower.contains("add") {
            commit.suggested_action = RebaseAction::Drop;
            commit.reasoning = "Test additions are often not needed in final history".to_string();
            commit.confidence = 0.6;
        } else {
            commit.suggested_action = RebaseAction::Pick;
            commit.reasoning = "Standard commit, keep as-is".to_string();
            commit.confidence = 0.7;
        }
    }

    /// Perform rebase with auto-applied AI suggestions
    pub async fn perform_rebase_auto(&self, analysis: RebaseAnalysis) -> Result<RebaseResult> {
        debug!(
            "Performing auto rebase with {} commits",
            analysis.commits.len()
        );

        // If there are no commits to rebase, return early
        if analysis.commits.is_empty() {
            ui::print_info("No commits to rebase.");
            return Ok(RebaseResult {
                operations_performed: 0,
                commits_processed: 0,
                success: true,
                conflicts: vec![],
            });
        }

        ui::print_info("Performing rebase operations...");

        // For now, perform a basic rebase that picks all commits
        // TODO: Implement selective rebase based on actions
        let result = self
            .repo
            .rebase(&analysis.upstream, Some(&analysis.branch))?;

        if result.success {
            ui::print_success(&format!(
                "Rebase completed successfully with {} operations",
                result.operations_performed
            ));
        } else {
            ui::print_warning("Rebase completed with conflicts that need to be resolved manually");
        }

        Ok(result)
    }
}

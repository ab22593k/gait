#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::as_conversions)]

use super::prompt::{create_completion_system_prompt, create_completion_user_prompt};
use super::types::GeneratedMessage;
use crate::config::Config;
use crate::core::context::CommitContext;
use crate::core::llm;
use crate::core::token_optimizer::TokenOptimizer;
use crate::git::{CommitResult, GitRepo};

use anyhow::Result;
use log::debug;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

/// Service for handling Git commit message completion with AI assistance
pub struct CompletionService {
    config: Config,
    repo: Arc<GitRepo>,
    provider_name: String,
    verify: bool,
    cached_context: Arc<RwLock<Option<CommitContext>>>,
}

impl CompletionService {
    /// Create a new `CompletionService` instance
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the service
    /// * `repo_path` - The path to the Git repository (unused but kept for API compatibility)
    /// * `provider_name` - The name of the LLM provider to use
    /// * `verify` - Whether to verify commits
    /// * `git_repo` - An existing `GitRepo` instance
    ///
    /// # Returns
    ///
    /// A Result containing the new `CompletionService` instance or an error
    pub fn new(
        config: Config,
        _repo_path: &Path,
        provider_name: &str,
        verify: bool,
        git_repo: GitRepo,
    ) -> Result<Self> {
        Ok(Self {
            config,
            repo: Arc::new(git_repo),
            provider_name: provider_name.to_string(),
            verify,
            cached_context: Arc::new(RwLock::new(None)),
        })
    }

    /// Check if the repository is remote
    pub fn is_remote_repository(&self) -> bool {
        self.repo.is_remote()
    }

    /// Check the environment for necessary prerequisites
    pub fn check_environment(&self) -> Result<()> {
        self.config.check_environment()
    }

    /// Get Git information for the current repository
    pub async fn get_git_info(&self) -> Result<CommitContext> {
        {
            let cached_context = self.cached_context.read().await;
            if let Some(context) = &*cached_context {
                return Ok(context.clone());
            }
        }

        let context = self.repo.get_git_info(&self.config).await?;

        {
            let mut cached_context = self.cached_context.write().await;
            *cached_context = Some(context.clone());
        }
        Ok(context)
    }

    /// Generate a commit message completion using AI
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix text to complete
    /// * `context_ratio` - The ratio of the original message to use as context (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A Result containing the generated completion or an error
    pub async fn complete_message(
        &self,
        prefix: &str,
        context_ratio: f32,
    ) -> anyhow::Result<GeneratedMessage> {
        let mut config_clone = self.config.clone();

        // Set instructions to include completion context
        let completion_instructions = format!(
            "Complete the commit message starting with the prefix: '{}'. Use {}% of the original message as context.",
            prefix,
            (context_ratio * 100.0) as i32
        );
        config_clone.instructions = completion_instructions;

        let mut context = self.get_git_info().await?;

        // Enhance context with semantically similar history
        context.author_history = context.get_enhanced_history(10);

        // Create system prompt for completion
        let system_prompt = create_completion_system_prompt(&config_clone)?;

        // Use the shared optimization logic
        let (_, final_user_prompt) = self
            .optimize_prompt(&config_clone, &system_prompt, context, |ctx| {
                create_completion_user_prompt(ctx, prefix, context_ratio)
            })
            .await;

        let generated_message = llm::get_message::<GeneratedMessage>(
            &config_clone,
            &self.provider_name,
            &system_prompt,
            &final_user_prompt,
        )
        .await?;

        Ok(generated_message)
    }

    /// Private helper method to handle common token optimization logic
    ///
    /// # Arguments
    ///
    /// * `config_clone` - Configuration with preset and instructions
    /// * `system_prompt` - The system prompt to use
    /// * `context` - The commit context
    /// * `create_user_prompt_fn` - A function that creates a user prompt from a context
    ///
    /// # Returns
    ///
    /// A tuple containing the optimized context and final user prompt
    async fn optimize_prompt<F>(
        &self,
        config_clone: &Config,
        system_prompt: &str,
        mut context: CommitContext,
        create_user_prompt_fn: F,
    ) -> (CommitContext, String)
    where
        F: Fn(&CommitContext) -> String,
    {
        // Get the token limit for the provider from config or default value
        let token_limit = config_clone
            .providers
            .get(&self.provider_name)
            .and_then(|p| p.token_limit)
            .unwrap_or({
                match self.provider_name.as_str() {
                    "openai" => 16_000,
                    "anthropic" => 100_000,
                    "groq" | "openrouter" => 32_000,
                    "google" => 1_000_000,
                    _ => 8_000,
                }
            });

        // Create a token optimizer to count tokens
        let optimizer = TokenOptimizer::for_counting().expect("Failed to create TokenOptimizer");
        let system_tokens = optimizer.count_tokens(system_prompt);

        debug!("Token limit: {}", token_limit);
        debug!("System prompt tokens: {}", system_tokens);

        // Reserve tokens for system prompt and some buffer for formatting
        // 1000 token buffer provides headroom for model responses and formatting
        let context_token_limit = token_limit.saturating_sub(system_tokens + 1000);
        debug!("Available tokens for context: {}", context_token_limit);

        // Count tokens before optimization
        let user_prompt_before = create_user_prompt_fn(&context);
        let total_tokens_before = system_tokens + optimizer.count_tokens(&user_prompt_before);
        debug!("Total tokens before optimization: {}", total_tokens_before);

        // Optimize the context with remaining token budget
        context.optimize(context_token_limit, config_clone).await;

        let user_prompt = create_user_prompt_fn(&context);
        let user_tokens = optimizer.count_tokens(&user_prompt);
        let total_tokens = system_tokens + user_tokens;

        debug!("User prompt tokens after optimization: {}", user_tokens);
        debug!("Total tokens after optimization: {}", total_tokens);

        // If we're still over the limit, truncate the user prompt directly
        // 100 token safety buffer ensures we stay under the limit
        let final_user_prompt = if total_tokens > token_limit {
            debug!(
                "Total tokens {} still exceeds limit {}, truncating user prompt",
                total_tokens, token_limit
            );
            let max_user_tokens = token_limit.saturating_sub(system_tokens + 100);
            optimizer
                .truncate_string(&user_prompt, max_user_tokens)
                .expect("Failed to truncate user prompt")
        } else {
            user_prompt
        };

        let final_tokens = system_tokens + optimizer.count_tokens(&final_user_prompt);
        debug!(
            "Final total tokens after potential truncation: {}",
            final_tokens
        );

        (context, final_user_prompt)
    }

    /// Performs a commit with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - The commit message.
    ///
    /// # Returns
    ///
    /// A Result containing the `CommitResult` or an error.
    pub fn perform_commit(
        &self,
        message: &str,
        amend: bool,
        commit_ref: Option<&str>,
    ) -> Result<CommitResult> {
        // Check if this is a remote repository
        if self.is_remote_repository() {
            return Err(anyhow::anyhow!("Cannot commit to a remote repository"));
        }

        debug!(
            "Performing commit with message: {}, amend: {}, commit_ref: {:?}",
            message, amend, commit_ref
        );

        if !self.verify {
            debug!("Skipping pre-commit hook (verify=false)");
            if amend {
                return self
                    .repo
                    .amend_commit(message, commit_ref.unwrap_or("HEAD"));
            }
            return self.repo.commit(message);
        }

        // Execute pre-commit hook
        debug!("Executing pre-commit hook");
        if let Err(e) = self.repo.execute_hook("pre-commit") {
            debug!("Pre-commit hook failed: {}", e);
            return Err(e);
        }
        debug!("Pre-commit hook executed successfully");

        // Perform the commit
        let commit_result = if amend {
            self.repo
                .amend_commit(message, commit_ref.unwrap_or("HEAD"))
        } else {
            self.repo.commit(message)
        };

        match commit_result {
            Ok(result) => {
                // Execute post-commit hook
                debug!("Executing post-commit hook");
                if let Err(e) = self.repo.execute_hook("post-commit") {
                    debug!("Post-commit hook failed: {}", e);
                    // We don't fail the commit if post-commit hook fails
                }
                debug!("Commit performed successfully");
                Ok(result)
            }
            Err(e) => {
                debug!("Commit failed: {}", e);
                Err(e)
            }
        }
    }

    /// Execute the pre-commit hook if verification is enabled
    pub fn pre_commit(&self) -> Result<()> {
        // Skip pre-commit hook for remote repositories
        if self.is_remote_repository() {
            debug!("Skipping pre-commit hook for remote repository");
            return Ok(());
        }

        if self.verify {
            self.repo.execute_hook("pre-commit")
        } else {
            Ok(())
        }
    }

    /// Create a channel for message completion
    pub fn create_completion_channel(
        &self,
    ) -> (
        mpsc::Sender<Result<GeneratedMessage>>,
        mpsc::Receiver<Result<GeneratedMessage>>,
    ) {
        mpsc::channel(1)
    }
}

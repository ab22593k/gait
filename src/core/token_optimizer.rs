use crate::{core::context::CommitContext, config::Config};
use tiktoken_rs::cl100k_base;
use log::debug;

pub struct TokenOptimizer {
    encoder: tiktoken_rs::CoreBPE,
    max_tokens: usize,
    config: Config,
}

#[derive(Debug)]
pub enum TokenError {
    EncoderInit(String),
    EncodingFailed(String),
    DecodingFailed(String),
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::EncoderInit(e) => write!(f, "Failed to initialize encoder: {e}"),
            TokenError::EncodingFailed(e) => write!(f, "Encoding failed: {e}"),
            TokenError::DecodingFailed(e) => write!(f, "Decoding failed: {e}"),
        }
    }
}

impl std::error::Error for TokenError {}

impl TokenOptimizer {
    pub fn new(max_tokens: usize, config: Config) -> Result<Self, TokenError> {
        let encoder = cl100k_base().map_err(|e| TokenError::EncoderInit(e.to_string()))?;

        Ok(Self {
            encoder,
            max_tokens,
            config,
        })
    }

    /// Create a token optimizer for counting only (no config needed)
    pub fn for_counting() -> Result<Self, TokenError> {
        let encoder = cl100k_base().map_err(|e| TokenError::EncoderInit(e.to_string()))?;

        Ok(Self {
            encoder,
            max_tokens: 0, // Not used for counting
            config: Config::default(), // Not used for counting
        })
    }

    pub async fn optimize_context(&self, context: &mut CommitContext) -> Result<(), TokenError> {
        let mut remaining_tokens = self.max_tokens;

        // Step 1: Process diffs (highest priority)
        remaining_tokens = self.optimize_diffs(context, remaining_tokens).await?;
        if remaining_tokens == 0 {
            debug!("Token budget exhausted after diffs, clearing commits and contents");
            Self::clear_commits_and_contents(context);
            return Ok(());
        }

        // Step 2: Process commits (medium priority)
        remaining_tokens = self.optimize_commits(context, remaining_tokens)?;
        if remaining_tokens == 0 {
            debug!("Token budget exhausted after commits, clearing contents");
            Self::clear_contents(context);
            return Ok(());
        }

        // Step 3: Process file contents (lowest priority)
        self.optimize_contents(context, remaining_tokens)?;

        debug!("Final token count: {}", self.max_tokens - remaining_tokens);

        Ok(())
    }

    // Optimize diffs and return remaining tokens
    async fn optimize_diffs(
        &self,
        context: &mut CommitContext,
        mut remaining: usize,
    ) -> Result<usize, TokenError> {
        for file in &mut context.staged_files {
            let diff_tokens = self.count_tokens(&file.diff);

            if diff_tokens > remaining {
                debug!(
                    "Summarizing diff for {} from {} to {} tokens",
                    file.path, diff_tokens, remaining
                );
                file.diff = self.hierarchical_summarize(&file.diff, remaining).await?;
                return Ok(0);
            }

            remaining = remaining.saturating_sub(diff_tokens);
        }
        Ok(remaining)
    }

    // Optimize commits and return remaining tokens
    fn optimize_commits(
        &self,
        context: &mut CommitContext,
        mut remaining: usize,
    ) -> Result<usize, TokenError> {
        for commit in &mut context.recent_commits {
            let commit_tokens = self.count_tokens(&commit.message);

            if commit_tokens > remaining {
                debug!(
                    "Truncating commit message from {} to {} tokens",
                    commit_tokens, remaining
                );
                commit.message = self.truncate_string(&commit.message, remaining)?;
                return Ok(0);
            }

            remaining = remaining.saturating_sub(commit_tokens);
        }
        Ok(remaining)
    }

    // Optimize file contents and return remaining tokens
    fn optimize_contents(
        &self,
        context: &mut CommitContext,
        mut remaining: usize,
    ) -> Result<usize, TokenError> {
        for file in &mut context.staged_files {
            if let Some(content) = &mut file.content {
                let content_tokens = self.count_tokens(content);

                if content_tokens > remaining {
                    debug!(
                        "Truncating file content for {} from {} to {} tokens",
                        file.path, content_tokens, remaining
                    );
                    *content = self.truncate_string(content, remaining)?;
                    return Ok(0);
                }

                remaining = remaining.saturating_sub(content_tokens);
            }
        }
        Ok(remaining)
    }

    pub fn truncate_string(&self, s: &str, max_tokens: usize) -> Result<String, TokenError> {
        let tokens = self.encoder.encode_ordinary(s);

        if tokens.len() <= max_tokens {
            return Ok(s.to_string());
        }

        if max_tokens == 0 {
            return Ok(String::from("…"));
        }

        // Reserve space for ellipsis
        let truncation_limit = max_tokens.saturating_sub(1);
        let ellipsis_token = self
            .encoder
            .encode_ordinary("…")
            .first()
            .copied()
            .ok_or_else(|| TokenError::EncodingFailed("Failed to encode ellipsis".to_string()))?;

        let mut truncated_tokens = Vec::with_capacity(truncation_limit + 1);
        truncated_tokens.extend_from_slice(&tokens[..truncation_limit]);
        truncated_tokens.push(ellipsis_token);

        self.encoder
            .decode(truncated_tokens)
            .map_err(|e| TokenError::DecodingFailed(e.to_string()))
    }

    #[inline]
    fn clear_commits_and_contents(context: &mut CommitContext) {
        Self::clear_commits(context);
        Self::clear_contents(context);
    }

    #[inline]
    fn clear_commits(context: &mut CommitContext) {
        context
            .recent_commits
            .iter_mut()
            .for_each(|c| c.message.clear());
    }

    #[inline]
    fn clear_contents(context: &mut CommitContext) {
        context
            .staged_files
            .iter_mut()
            .for_each(|f| f.content = None);
    }

    #[inline]
    pub fn count_tokens(&self, s: &str) -> usize {
        self.encoder.encode_ordinary(s).len()
    }

    /// Summarize text using LLM
    async fn summarize_text(&self, text: &str, max_tokens: usize) -> Result<String, TokenError> {
        let system_prompt = "You are a code diff summarizer. Provide a concise summary of the changes in the given diff, focusing on what was added, modified, or removed.";
        let user_prompt = format!("Summarize the following diff in {} tokens or less:\n\n{}", max_tokens, text);

        match crate::core::llm::get_message::<String>(
            &self.config,
            &self.config.default_provider,
            system_prompt,
            &user_prompt,
        ).await {
            Ok(summary) => Ok(summary),
            Err(e) => Err(TokenError::EncodingFailed(format!("Summarization failed: {}", e))),
        }
    }

    /// Perform hierarchical summarization (map-reduce) on large text
    async fn hierarchical_summarize(&self, text: &str, max_tokens: usize) -> Result<String, TokenError> {
        // Try to summarize, but fall back to truncation if LLM fails
        match self.try_hierarchical_summarize(text, max_tokens).await {
            Ok(summary) => Ok(summary),
            Err(_) => {
                // Fallback to truncation
                debug!("Summarization failed, falling back to truncation");
                self.truncate_string(text, max_tokens)
            }
        }
    }

    async fn try_hierarchical_summarize(&self, text: &str, max_tokens: usize) -> Result<String, TokenError> {
        // Split text into chunks that fit within LLM context
        let chunk_size = 4000; // Conservative chunk size for LLM input
        let chunks: Vec<&str> = text
            .as_bytes()
            .chunks(chunk_size)
            .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
            .filter(|chunk| !chunk.is_empty())
            .collect();

        if chunks.len() <= 1 {
            // If only one chunk, summarize directly
            return self.summarize_text(text, max_tokens).await;
        }

        // Map: Summarize each chunk
        let mut chunk_summaries = Vec::new();
        for chunk in &chunks {
            let summary = self.summarize_text(chunk, max_tokens / chunks.len()).await?;
            chunk_summaries.push(summary);
        }

        // Reduce: Combine summaries
        let combined = chunk_summaries.join("\n\n");
        if self.count_tokens(&combined) <= max_tokens {
            Ok(combined)
        } else {
            // If still too large, summarize the combined summaries
            self.summarize_text(&combined, max_tokens).await
        }
    }
}

use crate::{config::Config, core::context::CommitContext};
use log::debug;
use tiktoken_rs::cl100k_base;

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

#[derive(Debug)]
struct ContextItem {
    item_type: ContextItemType,
    token_count: usize,
    importance: f32,
}

#[derive(Debug)]
enum ContextItemType {
    Diff { file_index: usize },
    Commit { commit_index: usize },
    Content { file_index: usize },
}

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
            max_tokens: 0,             // Not used for counting
            config: Config::default(), // Not used for counting
        })
    }

    pub async fn optimize_context(&self, context: &mut CommitContext) -> Result<(), TokenError> {
        // Calculate importance scores for all context items
        let mut context_items = Vec::new();

        // Add diffs with importance scores
        for (i, file) in context.staged_files.iter().enumerate() {
            let token_count = self.count_tokens(&file.diff);
            // Importance = token_count (larger diffs are more important)
            let importance = token_count as f32;
            context_items.push(ContextItem {
                item_type: ContextItemType::Diff { file_index: i },
                token_count,
                importance,
            });
        }

        // Add commits with importance scores
        for (i, commit) in context.recent_commits.iter().enumerate() {
            let token_count = self.count_tokens(&commit.message);
            // Importance = similarity score (from filtering) * recency factor
            // Since we don't have stored similarity scores, use a heuristic:
            // importance = token_count * (position_factor to prefer earlier commits)
            let position_factor = 1.0 / (i + 1) as f32; // Earlier commits are more important
            let importance = token_count as f32 * position_factor;
            context_items.push(ContextItem {
                item_type: ContextItemType::Commit { commit_index: i },
                token_count,
                importance,
            });
        }

        // Add file contents with importance scores
        for (i, file) in context.staged_files.iter().enumerate() {
            if let Some(content) = &file.content {
                let token_count = self.count_tokens(content);
                // Importance = token_count * relevance_factor
                // Files that are staged are more relevant
                let relevance_factor = if context.staged_files.iter().any(|f| f.path == file.path) { 1.0 } else { 0.5 };
                let importance = token_count as f32 * relevance_factor;
                context_items.push(ContextItem {
                    item_type: ContextItemType::Content { file_index: i },
                    token_count,
                    importance,
                });
            }
        }

        // Sort by importance (highest first)
        context_items.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal));

        // Allocate tokens proportionally based on importance
        let total_importance: f32 = context_items.iter().map(|item| item.importance).sum();
        let mut remaining_tokens = self.max_tokens;

        for item in &context_items {
            if remaining_tokens == 0 {
                break;
            }

            let allocated_tokens = if total_importance > 0.0 {
                ((item.importance / total_importance) * self.max_tokens as f32) as usize
            } else {
                0
            }.min(item.token_count).min(remaining_tokens);

            if allocated_tokens < item.token_count {
                // Need to truncate this item
                match &item.item_type {
                    ContextItemType::Diff { file_index } => {
                        if let Some(file) = context.staged_files.get_mut(*file_index) {
                            debug!("Truncating diff for {path} from {original} to {allocated} tokens",
                                 path = file.path, original = item.token_count, allocated = allocated_tokens);
                            file.diff = self.truncate_string(&file.diff, allocated_tokens)?;
                        }
                    }
                    ContextItemType::Commit { commit_index } => {
                        if let Some(commit) = context.recent_commits.get_mut(*commit_index) {
                            debug!("Truncating commit message from {original} to {allocated} tokens",
                                 original = item.token_count, allocated = allocated_tokens);
                            commit.message = self.truncate_string(&commit.message, allocated_tokens)?;
                        }
                    }
                    ContextItemType::Content { file_index } => {
                        if let Some(file) = context.staged_files.get_mut(*file_index) {
                            if let Some(content) = &mut file.content {
                                debug!("Truncating content for {path} from {original} to {allocated} tokens",
                                     path = file.path, original = item.token_count, allocated = allocated_tokens);
                                *content = self.truncate_string(content, allocated_tokens)?;
                            }
                        }
                    }
                }
            }

            remaining_tokens = remaining_tokens.saturating_sub(allocated_tokens);
        }

        // Clear any remaining items that didn't get tokens
        if remaining_tokens == 0 {
            // Clear remaining low-importance items
            for item in context_items.iter().skip_while(|item| {
                match &item.item_type {
                    ContextItemType::Diff { .. } => true,
                    ContextItemType::Commit { .. } => true,
                    ContextItemType::Content { .. } => false,
                }
            }) {
                if let ContextItemType::Content { file_index } = &item.item_type {
                    if let Some(file) = context.staged_files.get_mut(*file_index) {
                        file.content = None;
                        file.content_excluded = true;
                    }
                }
            }
        }

        debug!("Optimized context with importance weighting, final token usage: {}", self.max_tokens - remaining_tokens);

        Ok(())
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
    pub fn count_tokens(&self, s: &str) -> usize {
        self.encoder.encode_ordinary(s).len()
    }

    /// Summarize text using LLM
    async fn summarize_text(&self, text: &str, max_tokens: usize) -> Result<String, TokenError> {
        let system_prompt = "You are a code diff summarizer. Provide a concise summary of the changes in the given diff, focusing on what was added, modified, or removed.";
        let user_prompt =
            format!("Summarize the following diff in {max_tokens} tokens or less:\n\n{text}");

        match crate::core::llm::get_message::<String>(
            &self.config,
            &self.config.default_provider,
            system_prompt,
            &user_prompt,
        )
        .await
        {
            Ok(summary) => Ok(summary),
            Err(e) => Err(TokenError::EncodingFailed(format!(
                "Summarization failed: {e}"
            ))),
        }
    }

    /// Perform hierarchical summarization (map-reduce) on large text
    async fn hierarchical_summarize(
        &self,
        text: &str,
        max_tokens: usize,
    ) -> Result<String, TokenError> {
        // Try to summarize, but fall back to truncation if LLM fails
        if let Ok(summary) = self.try_hierarchical_summarize(text, max_tokens).await {
            Ok(summary)
        } else {
            // Fallback to truncation
            debug!("Summarization failed, falling back to truncation");
            self.truncate_string(text, max_tokens)
        }
    }

    async fn try_hierarchical_summarize(
        &self,
        text: &str,
        max_tokens: usize,
    ) -> Result<String, TokenError> {
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
            let summary = self
                .summarize_text(chunk, max_tokens / chunks.len())
                .await?;
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

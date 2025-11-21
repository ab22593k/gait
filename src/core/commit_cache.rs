use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents a cached commit message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCommitMessage {
    pub message: String,
    pub timestamp: String,
    pub hash: String,
}

/// Cache for commit messages organized by author and repository
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitMessageCache {
    /// Maps `"author_email:repo_path"` -> list of commit messages
    cache: HashMap<String, Vec<CachedCommitMessage>>,
    cache_dir: PathBuf,
}

impl CommitMessageCache {
    /// Create a new cache instance
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        let cache_file = cache_dir.join("commit_messages.json");
        let cache = if cache_file.exists() {
            let content = fs::read_to_string(&cache_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self { cache, cache_dir })
    }

    /// Get the cache directory path
    fn get_cache_dir() -> Result<PathBuf> {
        let mut cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine cache directory"))?;
        cache_dir.push("gitsw");
        cache_dir.push("commit_cache");
        Ok(cache_dir)
    }

    /// Get commit messages for a specific author and repository
    pub fn get_commit_messages(
        &self,
        author_email: &str,
        repo_path: &str,
    ) -> Vec<CachedCommitMessage> {
        let key = format!("{author_email}:{repo_path}");
        self.cache.get(&key).cloned().unwrap_or_default()
    }

    /// Add commit messages for a specific author and repository
    pub fn add_commit_messages(
        &mut self,
        author_email: &str,
        repo_path: &str,
        messages: Vec<CachedCommitMessage>,
    ) {
        const MAX_MESSAGES_PER_AUTHOR_REPO: usize = 1000;

        let key = format!("{author_email}:{repo_path}");
        let existing = self.cache.entry(key).or_default();
        existing.extend(messages);

        // Keep only the most recent messages (limit to prevent unbounded growth)
        if existing.len() > MAX_MESSAGES_PER_AUTHOR_REPO {
            // Sort by timestamp (most recent first) and keep only the latest
            existing.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            existing.truncate(MAX_MESSAGES_PER_AUTHOR_REPO);
        }
    }

    /// Save the cache to disk
    pub fn save(&self) -> Result<()> {
        let cache_file = self.cache_dir.join("commit_messages.json");
        let content = serde_json::to_string_pretty(&self.cache)?;
        fs::write(cache_file, content)?;
        Ok(())
    }

    /// Get all cached authors for a repository
    pub fn get_authors_for_repo(&self, repo_path: &str) -> Vec<String> {
        self.cache
            .keys()
            .filter(|key| key.split(':').nth(1) == Some(repo_path))
            .map(|key| key.split(':').next().unwrap_or("").to_string())
            .collect()
    }

    /// Clear cache for a specific repository
    pub fn clear_repo_cache(&mut self, repo_path: &str) {
        self.cache
            .retain(|key, _| key.split(':').nth(1) != Some(repo_path));
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let total_messages = self.cache.values().map(Vec::len).sum();
        let total_authors = self.cache.len();
        let repos: std::collections::HashSet<_> = self
            .cache
            .keys()
            .map(|key| key.split(':').nth(1).unwrap_or(""))
            .collect();
        let total_repos = repos.len();

        CacheStats {
            total_messages,
            total_authors,
            total_repos,
        }
    }
}

/// Statistics about the cache
#[derive(Debug)]
pub struct CacheStats {
    pub total_messages: usize,
    pub total_authors: usize,
    pub total_repos: usize,
}

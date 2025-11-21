use std::collections::HashMap;

/// Simple semantic similarity calculator for commit messages
pub struct SemanticSimilarity {
    // For now, we'll use enhanced keyword matching
    // In a full implementation, this could use embeddings
}

impl SemanticSimilarity {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate similarity between current changes and historical commit messages
    pub fn calculate_similarities(
        &self,
        change_keywords: &[String],
        historical_messages: &[String],
    ) -> Vec<(usize, f32)> {
        let mut similarities = Vec::new();

        for (idx, message) in historical_messages.iter().enumerate() {
            let similarity = self.calculate_message_similarity(change_keywords, message);
            similarities.push((idx, similarity));
        }

        // Sort by similarity (highest first)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities
    }

    /// Calculate similarity between change keywords and a single commit message
    fn calculate_message_similarity(&self, keywords: &[String], message: &str) -> f32 {
        if keywords.is_empty() {
            return 0.0;
        }

        let message_lower = message.to_lowercase();
        let mut matches = 0;
        let mut total_weight = 0.0;

        for keyword in keywords {
            let weight = self.get_keyword_weight(keyword);
            total_weight += weight;

            if message_lower.contains(keyword) {
                matches += 1;
            }
        }

        if total_weight == 0.0 {
            0.0
        } else {
            (matches as f32) / (keywords.len() as f32)
        }
    }

    /// Get weight for a keyword based on its type (file names get higher weight)
    fn get_keyword_weight(&self, keyword: &str) -> f32 {
        // File-related keywords get higher weight
        if keyword.contains('.') || keyword.contains('/') {
            2.0
        } else {
            1.0
        }
    }

    /// Extract enhanced keywords from staged files and their changes
    pub fn extract_keywords(&self, staged_files: &[crate::core::context::StagedFile]) -> Vec<String> {
        let mut keywords = Vec::new();
        let mut keyword_counts = HashMap::new();

        for file in staged_files {
            // Extract from file path
            self.extract_from_path(&file.path, &mut keywords, &mut keyword_counts);

            // Extract from diff content
            if let Some(content) = &file.content {
                self.extract_from_content(content, &mut keywords, &mut keyword_counts);
            }

            // Extract from diff
            self.extract_from_diff(&file.diff, &mut keywords, &mut keyword_counts);
        }

        // Sort by frequency and return top keywords
        let mut sorted_keywords: Vec<_> = keyword_counts.into_iter().collect();
        sorted_keywords.sort_by(|a, b| b.1.cmp(&a.1));

        sorted_keywords
            .into_iter()
            .take(20) // Limit to top 20 keywords
            .map(|(k, _)| k)
            .collect()
    }

    fn extract_from_path(&self, path: &str, keywords: &mut Vec<String>, counts: &mut HashMap<String, usize>) {
        let file_name = path.split('/').last().unwrap_or(path);
        let parts: Vec<&str> = file_name.split('.').collect();

        if let Some(name_without_ext) = parts.first() {
            // Split camelCase and snake_case
            let words: Vec<String> = name_without_ext
                .split('_')
                .flat_map(|part| split_camel_case(part))
                .map(|s| s.to_lowercase())
                .filter(|s| s.len() > 2) // Filter out very short words
                .collect();

            for word in words {
                if !word.is_empty() {
                    *counts.entry(word.clone()).or_insert(0) += 2; // Higher weight for file names
                    if !keywords.contains(&word) {
                        keywords.push(word);
                    }
                }
            }
        }
    }

    fn extract_from_content(&self, content: &str, keywords: &mut Vec<String>, counts: &mut HashMap<String, usize>) {
        let content_words: Vec<String> = content
            .split_whitespace()
            .take(100) // Limit processing
            .filter(|word| word.len() > 3 && word.chars().all(|c| c.is_alphanumeric() || c == '_'))
            .map(|word| word.to_lowercase())
            .collect();

        for word in content_words {
            *counts.entry(word.clone()).or_insert(0) += 1;
            if !keywords.contains(&word) {
                keywords.push(word);
            }
        }
    }

    fn extract_from_diff(&self, diff: &str, keywords: &mut Vec<String>, counts: &mut HashMap<String, usize>) {
        // Extract function names, variable names, etc. from diff
        let diff_words: Vec<String> = diff
            .lines()
            .filter(|line| line.starts_with('+') || line.starts_with('-'))
            .flat_map(|line| line.split_whitespace())
            .filter(|word| word.len() > 3 && word.chars().all(|c| c.is_alphanumeric() || c == '_'))
            .map(|word| word.to_lowercase())
            .take(50) // Limit processing
            .collect();

        for word in diff_words {
            *counts.entry(word.clone()).or_insert(0) += 1;
            if !keywords.contains(&word) {
                keywords.push(word);
            }
        }
    }
}

/// Split camelCase into individual words
fn split_camel_case(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_uppercase() {
            // If we have a current word, save it
            if !current_word.is_empty() {
                words.push(current_word);
                current_word = String::new();
            }

            // Check if this is the start of an acronym (multiple uppercase letters)
            let mut j = i;
            while j < chars.len() && chars[j].is_uppercase() {
                j += 1;
            }

            // If we have more than one uppercase letter followed by a lowercase,
            // or if it's the end of the string, treat as acronym
            if j > i + 1 && (j == chars.len() || (j < chars.len() && chars[j].is_lowercase())) {
                // Acronym: take all uppercase letters except the last one
                if j - i > 1 {
                    words.push(chars[i..j-1].iter().collect());
                    current_word.push(chars[j-1]);
                } else {
                    current_word.push(chars[i]);
                }
                i = j;
            } else {
                // Single uppercase letter
                current_word.push(chars[i]);
                i += 1;
            }
        } else {
            current_word.push(chars[i]);
            i += 1;
        }
    }

    if !current_word.is_empty() {
        words.push(current_word);
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_camel_case() {
        assert_eq!(split_camel_case("camelCase"), vec!["camel", "Case"]);
        assert_eq!(split_camel_case("XMLHttpRequest"), vec!["XML", "Http", "Request"]);
        assert_eq!(split_camel_case("simple"), vec!["simple"]);
    }

    #[test]
    fn test_semantic_similarity() {
        let similarity = SemanticSimilarity::new();
        let keywords = vec!["test".to_string(), "function".to_string()];
        let message = "add test function".to_string();

        let score = similarity.calculate_message_similarity(&keywords, &message);
        assert!(score > 0.0);
    }
}
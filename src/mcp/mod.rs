//! Model Context Protocol (MCP) integration
//!
//! This module contains the implementation of the MCP server
//! that allows `gitai` to be used directly from compatible
//! LLM-powered tools and assistants.

pub mod config;
pub mod server;
pub mod tools;

// Re-export main components
pub use server::serve;
pub use tools::{ChangelogTool, CodeReviewTool, CommitTool, PrTool, ReleaseNotesTool};

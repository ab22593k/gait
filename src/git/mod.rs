// Git module providing functionality for Git repository operations

mod commit;
mod files;
mod ignore_matcher;
mod repository;
mod utils;

// Re-export primary types for public use
pub use commit::CommitInfo;
pub use commit::CommitResult;
pub use repository::GitRepo;

// Re-export utility functions
pub use utils::*;

// Re-export type aliases to maintain backward compatibility
pub use crate::core::context::{RecentCommit, StagedFile};
pub use files::RepoFilesInfo;

//! `git wire` Tool that wires parts of other repositories' source code
//! into the current repository in a declarative manner.
//!
//! ## Features
//!
//! - Declarative cross-repository code synchronization
//! - JSON-based configuration for managing external code dependencies
//! - Multiple checkout methods (shallow, shallow_no_sparse, partial)
//! - Multi-threaded execution with single-threaded option
//! - **New**: Repository caching to avoid multiple git pulls for the same repository
//!
//! ## Repository Caching
//!
//! The new caching system implemented in this version significantly improves
//! performance by identifying when multiple configuration entries reference the
//! same remote repository and performing only one git pull operation per unique
//! repository during a sync process. This reduces redundant network operations,
//! saves bandwidth, and decreases overall sync time.

pub mod cache;
pub mod check;
pub mod common;
pub mod models;
pub mod sync;

pub use cache::manager::CacheManager;
pub use models::cached_repo::CachedRepository;
pub use models::repo_config::RepositoryConfiguration;
pub use models::wire_operation::WireOperation;

pub fn init_logger() {
    env_logger::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

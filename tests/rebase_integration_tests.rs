use anyhow::Result;
use gitai::{
    config::Config,
    features::rebase::{RebaseAnalysis, RebaseService},
    git::GitRepo,
};
use tempfile::TempDir;

// Use our centralized test infrastructure
#[path = "test_utils.rs"]
mod test_utils;
use test_utils::setup_git_repo_with_commits;

fn setup_test_repo() -> Result<(TempDir, GitRepo)> {
    let (temp_dir, git_repo) = setup_git_repo_with_commits()?;
    Ok((temp_dir, git_repo))
}

#[tokio::test]
async fn test_rebase_analysis() -> Result<()> {
    let (temp_dir, _git_repo) = setup_test_repo()?;
    let config = Config::default();

    let service_repo = GitRepo::new(temp_dir.path())?;
    let service = RebaseService::new_test(config, service_repo)?;

    // Create a branch and add some commits
    let git_repo = GitRepo::new(temp_dir.path())?;
    let repo = git_repo.open_repo()?;
    let head_commit = repo.head()?.peel_to_commit()?;

    // Create a new branch
    repo.branch("feature-branch", &head_commit, false)?;
    repo.set_head("refs/heads/feature-branch")?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

    // Add a few more commits
    for i in 1..=3 {
        // Create/modify a file
        std::fs::write(
            temp_dir.path().join(format!("file{i}.txt")),
            format!("Content {i}"),
        )?;

        // Stage and commit
        let mut index = repo.index()?;
        index.add_path(std::path::Path::new(&format!("file{i}.txt")))?;
        let tree = index.write_tree()?;
        let tree_obj = repo.find_tree(tree)?;

        let signature = repo.signature()?;
        let parent = repo.head()?.peel_to_commit()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("Commit message {i}"),
            &tree_obj,
            &[&parent],
        )?;
    }

    // Analyze commits for rebase
    let analysis = service
        .analyze_commits_for_rebase("main", Some("feature-branch"))
        .await?;

    assert!(
        !analysis.commits.is_empty(),
        "Should have found commits to rebase"
    );
    assert_eq!(analysis.upstream, "main");
    assert_eq!(analysis.branch, "feature-branch");

    for commit in &analysis.commits {
        assert!(!commit.message.is_empty(), "Commit should have a message");
        assert!(!commit.hash.is_empty(), "Commit should have a hash");
        // Check that AI suggested an action
        assert!(matches!(
            commit.suggested_action,
            gitai::features::rebase::RebaseAction::Pick
                | gitai::features::rebase::RebaseAction::Reword
                | gitai::features::rebase::RebaseAction::Squash
                | gitai::features::rebase::RebaseAction::Fixup
                | gitai::features::rebase::RebaseAction::Drop
                | gitai::features::rebase::RebaseAction::Edit
        ));
        assert!(
            commit.confidence >= 0.0 && commit.confidence <= 1.0,
            "Confidence should be between 0 and 1"
        );
        assert!(
            !commit.reasoning.is_empty(),
            "Should have reasoning for the action"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_rebase_auto_apply() -> Result<()> {
    let (temp_dir, _git_repo) = setup_test_repo()?;
    let config = Config::default();

    let service_repo = GitRepo::new(temp_dir.path())?;
    let service = RebaseService::new(config, service_repo)?;

    // Create a simple analysis for testing
    let analysis = RebaseAnalysis {
        commits: vec![], // Empty for now, will be populated by analysis
        upstream: "main".to_string(),
        branch: "feature-branch".to_string(),
        suggested_operations: 0,
    };

    // Test the auto-apply functionality (this will be a no-op for empty commits)
    let result = service.perform_rebase_auto(analysis).await?;

    assert!(result.success, "Rebase should succeed");
    assert_eq!(
        result.operations_performed, 0,
        "No operations should be performed on empty analysis"
    );
    assert_eq!(
        result.commits_processed, 0,
        "No commits should be processed"
    );
    assert!(result.conflicts.is_empty(), "No conflicts should occur");

    Ok(())
}

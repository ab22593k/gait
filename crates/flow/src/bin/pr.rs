use anyhow::Result;
use clap::Parser;
use gwtflow::common::CommonParams;

#[derive(Parser)]
#[command(name = "git-flow-pr", about = "Generate a pull request description using AI")]
struct PrArgs {
    #[command(flatten)]
    common: CommonParams,

    /// Print the generated PR description to stdout and exit
    #[arg(
        short,
        long,
        help = "Print the generated PR description to stdout and exit"
    )]
    print: bool,

    /// Starting branch, commit, or commitish for comparison
    #[arg(
        long,
        help = "Starting branch, commit, or commitish for comparison. For single commit analysis, specify just this parameter with a commit hash (e.g., --from abc1234). For reviewing multiple commits, use commitish syntax (e.g., --from HEAD~3 to review last 3 commits)"
    )]
    from: Option<String>,

    /// Target branch, commit, or commitish for comparison
    #[arg(
        long,
        help = "Target branch, commit, or commitish for comparison. For single commit analysis, specify just this parameter with a commit hash or commitish (e.g., --to HEAD~2)"
    )]
    to: Option<String>,

    /// Repository URL to use instead of local repository
    #[arg(
        short = 'r',
        long = "repo",
        help = "Repository URL to use instead of local repository"
    )]
    repository_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    gwtflow::logger::init().expect("Failed to initialize logger");
    
    let args = PrArgs::parse();
    
    match gwtflow::cli::handle_pr(
        args.common,
        args.print,
        args.from,
        args.to,
        args.repository_url,
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
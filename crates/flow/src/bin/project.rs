use anyhow::Result;
use clap::Parser;
use gwtflow::{commands::handle_project_config_command, common::CommonParams};

#[derive(Parser)]
#[command(
    name = "git-flow-project",
    about = "Manage project-specific configuration"
)]
struct ProjectArgs {
    #[command(flatten)]
    common: CommonParams,

    /// Set model for the specified provider
    #[arg(long, help = "Set model for the specified provider")]
    model: Option<String>,

    /// Set token limit for the specified provider
    #[arg(long, help = "Set token limit for the specified provider")]
    token_limit: Option<usize>,

    /// Set additional parameters for the specified provider
    #[arg(
        long,
        help = "Set additional parameters for the specified provider (key=value)"
    )]
    param: Option<Vec<String>>,

    /// Print the current project configuration
    #[arg(short, long, help = "Print the current project configuration")]
    print: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    gwtflow::logger::init().expect("Failed to initialize logger");

    let args = ProjectArgs::parse();

    match handle_project_config_command(
        &args.common,
        args.model,
        args.token_limit,
        args.param,
        args.print,
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

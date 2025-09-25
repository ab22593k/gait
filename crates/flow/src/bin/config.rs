use anyhow::Result;
use clap::Parser;
use gwtflow::common::CommonParams;

#[derive(Parser)]
#[command(
    name = "git-flow-config",
    about = "Configure Git-Iris settings and providers"
)]
struct ConfigArgs {
    #[command(flatten)]
    common: CommonParams,

    /// Set API key for the specified provider
    #[arg(long, help = "Set API key for the specified provider")]
    api_key: Option<String>,

    /// Set model for the specified provider
    #[arg(short, long, help = "Set model for the specified provider")]
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
}

#[tokio::main]
async fn main() -> Result<()> {
    gwtflow::logger::init().expect("Failed to initialize logger");

    let args = ConfigArgs::parse();

    match gwtflow::cli::handle_config(
        &args.common,
        args.api_key,
        args.model,
        args.token_limit,
        args.param,
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

use anyhow::Result;
use ritex_flow::cli;

/// Main entry point for the application
#[tokio::main]
async fn main() -> Result<()> {
    ritex_flow::logger::init().expect("Failed to initialize logger");
    match cli::main().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

use anyhow::Result;
use clap::Parser;
use gwtflow::commands::handle_list_presets_command;

#[derive(Parser)]
#[command(
    name = "git-flow-list-presets",
    about = "List available instruction presets"
)]
struct ListPresetsArgs {}

#[tokio::main]
async fn main() -> Result<()> {
    gwtflow::logger::init().expect("Failed to initialize logger");

    let _args = ListPresetsArgs::parse();

    match handle_list_presets_command() {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

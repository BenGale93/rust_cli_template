#![warn(clippy::all, clippy::nursery)]
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(about = "")]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {}

fn main() {
    pretty_env_logger::init();
    let cli = Cli::parse();

    let result = match cli.command {};

    if let Err(e) = result {
        log::error!("{}", e);
    }
}

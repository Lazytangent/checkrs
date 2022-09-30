use clap::Parser;

use crate::utils;

pub mod schemas;
pub mod subcommands;

pub fn run() {
    let cli = schemas::Cli::parse();

    match cli.command {
        Some(cmd) => subcommands::run_subcommand(cmd),
        None => utils::process_without_subcommand(&cli),
    }
}

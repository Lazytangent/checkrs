use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Command
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Check the status of a given path
    Status {
        /// Path of the local repository you want to check
        #[clap(value_parser)]
        path: String,
    },
}

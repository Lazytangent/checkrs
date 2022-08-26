use std::process::Command;

use crate::cli::schemas;

pub fn run_subcommand(command: schemas::Command) {
    match command {
        schemas::Command::Status { path } => handle_status(&path),
    }
}

fn handle_status(path: &str) {
    println!("In {path}\n");
    Command::new("git")
        .arg("status")
        .current_dir(path)
        .status()
        .expect("git status should be run within a repository");
}

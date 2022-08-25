use std::{env, process::Command};

pub fn run_subcommand(subcommand: String) {
    match subcommand.as_str() {
        "status" => handle_status(),
        _ => unimplemented!(),
    }
}

fn handle_status() {
    let path = env::args()
        .nth(2)
        .expect("status subcommand requires a path");
    println!("In {path}\n");
    Command::new("git")
        .arg("status")
        .current_dir(path)
        .status()
        .expect("git status should be run within a repository");
}

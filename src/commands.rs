use std::{
    process::{Command, Output},
    str,
    sync::{Arc, Mutex},
};

use log::debug;

use crate::config;

#[derive(Debug, Clone)]
pub struct Outputs {
    pub stdout: String,
    pub stderr: String,
    pub path: String,
}

fn run_command(cmd: &mut Command, dir: &str) -> Output {
    let file_path = config::parse_path_with_tilde(dir).unwrap();

    cmd.current_dir(file_path);
    match cmd.output() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Command: {:?} -- Args: {:?}", cmd.get_program(), cmd.get_args());
            eprintln!("Directory: {dir}");
            eprintln!("Error while running command:");
            eprintln!("{e:?}");
            eprintln!("{:-<80}", "");

            panic!();
        }
    }
}

pub fn git_status(path: String, mutex: Arc<Mutex<Vec<Outputs>>>) {
    let mut cmd = Command::new("git");
    let cmd = cmd.arg("status");
    let output = run_command(cmd, &path);
    let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
    let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
    debug!("Output:\n{}", stdout);
    debug!("Error:\n{}", stderr);
    debug!("---------------------------");

    let outputs = Outputs {
        stdout,
        stderr,
        path,
    };
    mutex.lock().unwrap().push(outputs);
}

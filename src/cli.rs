use std::{
    io,
    process::{Command, Output},
    str,
};

use log::debug;

use crate::config;

pub fn run() {
    let contents =
        get_config_file_contents(config::PATH_TO_CONFIG_DIR, config::CONFIG_FILE_NAME).unwrap();
    let paths = config::generate_list_of_paths(contents);

    let outputs = get_status_from_paths(paths);
    parse_outputs(outputs);
}

fn run_command(cmd: &mut Command, dir: &str) -> Output {
    let file_path = config::parse_path_with_tilde(dir).unwrap();

    cmd.current_dir(file_path);
    cmd.output().expect("Error while running command")
}

fn get_config_file_contents(path: &str, filename: &str) -> io::Result<String> {
    let mut path = config::parse_path_with_tilde(path).unwrap();
    path.push_str(filename);
    let contents = config::read_config_file(&path).unwrap();
    debug!("Contents:\t{:?}", contents);
    Ok(contents)
}

struct Outputs {
    stdout: String,
    stderr: String,
}

fn get_status_from_paths(paths: Vec<String>) -> Vec<Outputs> {
    let mut cmd = Command::new("git");
    let cmd = cmd.arg("status");

    let mut outputs: Vec<Outputs> = Vec::new();

    for path in paths {
        let output = run_command(cmd, &path);
        let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
        let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
        debug!("Output:\n{}", stdout);
        debug!("Error:\n{}", stderr);
        debug!("---------------------------");

        let output = Outputs { stdout, stderr };
        outputs.push(output);
    }

    outputs
}

fn parse_outputs(outputs: Vec<Outputs>) {
    // TODO: Parse outputs using Regex to figure out which repositories need to be looked at
    // TODO: Which repos can be safely ignored as part of a group ignore message
}

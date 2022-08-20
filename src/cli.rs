use std::{process::{Command, Output}, str};

use log::debug;

use crate::config;

pub fn run() {
    let mut path = config::parse_path_with_tilde(config::PATH_TO_CONFIG_DIR).unwrap();
    path.push_str(config::CONFIG_FILE_NAME);
    let contents = config::read_config_file(&path).unwrap();
    debug!("Contents:\t{:?}", contents);

    let paths = config::generate_list_of_paths(contents);
    debug!("Paths: {:?}", paths);
    let mut cmd = Command::new("git");
    let cmd = cmd.arg("status");

    for path in paths {
        let output = run_command(cmd, &path);
        print!("Output:\n{}", str::from_utf8(&output.stdout).unwrap());
        println!("---------------------------");
    }
}

fn run_command(cmd: &mut Command, dir: &str) -> Output {
    let file_path = config::parse_path_with_tilde(dir).unwrap();

    cmd.current_dir(file_path);
    cmd.output().expect("Error while running command")
}

use std::process::{Command, Output};

use crate::config;

pub fn run() {
    // run_command("ls", config::PATH_TO_CONFIG_DIR);

    let mut path = config::parse_path_with_tilde(config::PATH_TO_CONFIG_DIR).unwrap();
    path.push_str(config::CONFIG_FILE_NAME);
    let contents = config::read_config_file(&path).unwrap();
    // debug!("Contents:\n{:?}", contents);

    let paths = config::generate_list_of_paths(contents);
    // debug!("Paths: {:?}", paths);
    let mut cmd = Command::new("git");
    let cmd = cmd.arg("status");

    let output = run_command(cmd, &paths[0]);
    println!("Output: {:?}", output);
}

fn run_command(cmd: &mut Command, dir: &str) -> Output {
    let file_path = config::parse_path_with_tilde(dir).unwrap();

    cmd.current_dir(file_path);
    cmd.output().expect("Error while running command")
}

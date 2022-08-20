use std::process::Command;

use crate::config;

pub fn run() {
    // run_command("ls", config::PATH_TO_CONFIG_DIR);

    let mut path = config::parse_path_with_tilde(config::PATH_TO_CONFIG_DIR).unwrap();
    path.push_str(config::CONFIG_FILE_NAME);
    let contents = config::read_config_file(&path).unwrap();
    // debug!("Contents:\n{:?}", contents);

    let paths = config::generate_list_of_paths(contents);
    println!("Paths: {:?}", paths);
}

fn run_command(cmd: &str, dir: &str) {
    let mut command = Command::new(cmd);
    command.status().expect("Error while running command");
    println!();

    let file_path = config::parse_path_with_tilde(dir).unwrap();

    command.current_dir(file_path);
    command.status().expect("Error while running command");
}

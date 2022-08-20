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

fn get_config_file_contents(path: &str, filename: &str) -> io::Result<String> {
    let mut path = config::parse_path_with_tilde(path).unwrap();
    path.push_str(filename);
    let contents = config::read_config_file(&path).unwrap();
    debug!("Contents:\t{:?}", contents);
    Ok(contents)
}

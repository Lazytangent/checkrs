use std::{env, fs::File, io::{self, Read}};

pub static PATH_TO_CONFIG_DIR: &str = "~/.config/checkrs/";
pub static CONFIG_FILE_NAME: &str = "config";

pub fn parse_path_with_tilde(path: &str) -> Result<String, String> {
    let path = String::from(path);
    // debug!("Path starts as: {}", path);

    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => return Err(format!("Error while parsing HOME env var: {}", e)),
    };

    let path = path.replace("~", &home);
    // debug!("Path is now: {}", path);

    Ok(path)
}

pub fn read_config_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn generate_list_of_paths(contents: String) -> Vec<String> {
    let mut paths_to_check: Vec<String> = Vec::new();

    for line in contents.lines() {
        let path = parse_path_with_tilde(line).unwrap();
        paths_to_check.push(path);
    }

    paths_to_check
}

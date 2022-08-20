use std::env;

pub static PATH_TO_CONFIG_DIR: &str = "~/.config/checkrs/";

pub fn parse_path_with_tilde(path: &str) -> Result<String, String> {
    let path = String::from(path);
    println!("Path starts as: {}", path);

    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => return Err(format!("Error while parsing HOME env var: {}", e)),
    };

    let path = path.replace("~", &home);
    println!("Path is now: {}", path);

    Ok(path)
}
